// #![allow(invalid_value)]

// use std::sync::Mutex;
// use std::time::Instant;
// #[deny(soft_unstable)]
// extern crate test;

// // 1. test sanitizer validity union (1)
// union MyUninit {
//     init: (),
//     uninit: [bool; 1],
// }

// #[derive(Default)]
// struct MyStruct {
//     init: (),
//     uninit: [bool; 1],
// }
  
// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//     for i in 0..100000 {
//             vec.push(i);
//     }
//     let _b = unsafe { MyUninit { init: () }.uninit }; //~ ERROR: constructing invalid value
//     let duration =start.elapsed();
//         println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//     for i in 0..100000 {
//             vec.push(i);
//     }
//     let mut u = MyStruct::default();
//     u.init = ();
//     assert_eq!(u.init==()  && u.uninit ==MyStruct::default().uninit, true );
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// 2. Validity (2)
// union MyUninit {
//     init: (),
//     uninit: [char; 1],
// }

// #[derive(Default)]
// struct MyStruct {
//     init: (),
//     uninit: [char; 1],
// }

// fn main() {
//     let start = Instant::now();
//         let mut vec: Vec<u32> = Vec::new();
//         for i in 0..100000 {
//                 vec.push(i);
//         }
//     let _b = unsafe { MyUninit { init: () }.uninit};

//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//     for i in 0..100000 {
//             vec.push(i);
//     }
//     let mut u = MyStruct::default();
//     u.init = ();
//     assert_eq!(u.init==()  && u.uninit ==MyStruct::default().uninit, true );
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// 3. validity (3) union 
// union MyUninit {
//     init: (),
//     uninit: [fn(); 1],
// }

// #[derive(Default)]
// struct MyStruct {
//     init: (),
//     uninit: [char; 1],
// }


// fn main() {
//     let start = Instant::now();
//         let mut vec: Vec<u32> = Vec::new();
//         for i in 0..100000 {
//                 vec.push(i);
//         }
//     let _b = unsafe { MyUninit { init: () }.uninit};

//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//     for i in 0..100000 {
//             vec.push(i);
//     }
//     let mut u = MyStruct::default();
//     u.init = ();
//     assert_eq!(u.init==()  && u.uninit ==MyStruct::default().uninit, true );
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// 4. statck_borrowed (union) illegal
// use std::mem;

// union HiddenRef {
//     r: &'static i32,
// }

// fn main() {
//     let start = Instant::now();
//     let mut x: i32 = 15;
//     let xref1 = &mut x;
//     let xref1_sneaky: HiddenRef = unsafe {
//         mem::transmute_copy(&xref1)
//     };
//     let xref2 = &mut *xref1;
//     callee(xref1_sneaky);
//     let _val = *xref2;
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn callee(xref1: HiddenRef) {
//     let _val = unsafe {
//         *xref1.r
//     };
// }


// 5.
// fn demo_box_advanced_unique(mut our: Box<i32>) -> i32 {
//     unknown_code_1(&*our);
//     *our = 5;
//     unknown_code_2();
//     *our
// }

// use std::ptr; 
// use std::sync::Arc;


// static mut LEAK: *mut i32 = ptr::null_mut();
// // static  LEAK: Arc<Mutex<*mut i32>> = Arc::new(Mutex::new(ptr::null_mut()));
// // lazy_static! {
// //     static  ref LEAK: Mutex<*mut i32> = Mutex::new(ptr::null_mut());
// // }

// fn unknown_code_1(x: &i32) {
//     unsafe {
//         LEAK = x as *const _ as *mut _;
//     }
// }

// fn unknown_code_2() {
//     unsafe {
//         *LEAK = 7;
//         //~[stack]^ ERROR: /write access .* tag does not exist in the borrow stack/
//         //~[tree]| ERROR: /write access through .* is forbidden/
//     }
// }

// fn main() {
//     demo_box_advanced_unique(Box::new(0));
// }

// This should fail even without validation
//@compile-flags: -Zmiri-disable-validation -Zmiri-permissive-provenance

// static mut LEAK: usize = 0;

// fn fill(v: &mut i32) {
//     unsafe {
//         LEAK = v as *mut _ as usize;
//     }
// }

// fn evil() {
//     let _ = unsafe { &mut *(LEAK as *mut i32) }; //~ ERROR: is a dangling pointer
// }

// fn main() {
//     let _y;
//     {
//         let mut x = 0i32;
//         fill(&mut x);
//         _y = x;
//     }
//     // Now we use a pointer to `x` which is no longer in scope, and thus dead (even though the
//     // `main` stack frame still exists). We even try going through a `usize` for extra sneakiness!
//     evil();
// }

// We want to control preemption here. Stacked borrows interferes by having its own accesses.
//@compile-flags: -Zmiri-preemption-rate=0 -Zmiri-disable-stacked-borrows
// use std::sync::atomic::{fence, AtomicUsize, Ordering};
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let start = Instant::now();
//     // static mut V: u32 = 0;
//     static V:Mutex<u32> = Mutex::new(0);
//     let a = Arc::new(AtomicUsize::default());
//     let b = a.clone();
//     thread::spawn(move || {
//         // unsafe { V = 1 }
//         let mut t = V.lock().unwrap();
//         *t  = 1 ;
//         b.store(1, Ordering::SeqCst);
//     });
//     thread::sleep(Duration::from_millis(1));
//     fence(Ordering::SeqCst);
//     // Imagine the other thread's actions happening here.
//     // assert_eq!(a.load(Ordering::Relaxed), 1);
//     let mut t2 = V.lock().unwrap();
//     *t2 = 2;
//     // unsafe { V = 2 } //~ERROR: Data race detected between (1) non-atomic write on thread `<unnamed>` and (2) non-atomic write on thread `main`
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }


// static mut LEAK: usize = 0;
// static LEAK: Mutex<usize> = Mutex::new(0);

// fn fill(v: &mut i32) {
//         // LEAK = v as *mut _ as usize;
//         let t = v as *mut _ as usize;
//         let mut q = LEAK.lock().unwrap();
//         *q = t;
// }

// fn evil() {
//     // let _ = unsafe { 
//     //     &mut *(LEAK as *mut i32) 
//     // }; //~ ERROR: is a dangling pointer
//     // let t = LEAK as *mut i32;
//     let mut t = LEAK.lock().unwrap();
//     let q = *t as *mut i32;
//     let _ = unsafe {
//         &mut *q
//     };
// }

// fn main() {
//     let _y;
//     {
//         let mut x = 0i32;
//         fill(&mut x);
//         _y = x;
//     }
//     // Now we use a pointer to `x` which is no longer in scope, and thus dead (even though the
//     // `main` stack frame still exists). We even try going through a `usize` for extra sneakiness!
//     evil();
// }

// A zero-sized drop type -- the retagging of `fn drop` itself won't
// do anything (since it is zero-sized); we are entirely relying on the retagging
// in `drop_in_place` here.
// static mut PTR: *mut u8 = 0 as *mut _;
// // static  PTR:Mutex<u8> = Mutex::new(0);

// fn fun1(x: &mut u8) {
//     unsafe {
//         PTR = x;
//     }
//     // let t = PTR.lock().unwrap();
//     // let mut q = *t as *mut u8;
//     // q = x;
// }

// fn fun2() {
//     // Now we use a pointer we are not allowed to use
//     let _x = unsafe { *PTR }; //~ ERROR: /read access .* tag does not exist in the borrow stack/
//     // let t = PTR.lock().unwrap();
//     // let _x = *t;
// }

// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//         for i in 0..100000 {
//                 vec.push(i);
//         }
//     let mut val = 0;
//     let val = &mut val;
//     fun1(val);
//     *val = 2; // this invalidates any raw ptrs `fun1` might have created.
//     fun2(); // if they now use a raw ptr they break our reference
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }







// #![allow(unreachable_code)]
// #![feature(never_type)]
// #![allow(deprecated, invalid_value)]
// // use std::any::type_name;
// // use std::mem::discriminant;


// #![feature(intrinsics)]

// use std::time::Instant;
// // Directly call intrinsic to avoid debug assertions in libstd
// extern "rust-intrinsic" {
//     fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
// }

// fn main() {
//     let start = Instant::now();
//     let mut data = [0u16; 8];
//     let ptr = (&mut data[0] as *mut u16 as *mut u8).wrapping_add(1) as *mut u16;
//     // Even copying 0 elements to something unaligned should error
//     unsafe {
//         copy_nonoverlapping(&data[5], ptr, 0); //~ ERROR: accessing memory with alignment 1, but alignment 2 is required
//     }
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// #[repr(transparent)]
// struct HasDrop(u8);

// impl Drop for HasDrop {
//     fn drop(&mut self) {}
// }

// #[repr(C, align(2))]
// struct PartialDrop {
//     a: HasDrop,
//     b: u8,
// }

// //@error-in-other-file: /required 2 byte alignment/
// fn main() {
//     let start = Instant::now();
//     unsafe {
//         // Create an unaligned pointer
//         let mut x = [0_u16; 2];
//         let p = core::ptr::addr_of_mut!(x).cast::<u8>();
//         let p = p.add(1);
//         let p = p.cast::<PartialDrop>();

//         core::ptr::drop_in_place(p);
//     }
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }


// fn main() {
//     let start = Instant::now();
//     let mut vec: Vec<u32> = Vec::new();
//     for i in 0..100000 {
//         vec.push(i);
//     }
//     println!("Vector length: {}",vec.len());
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let p = {
//         let b = Box::new(42);
//         &*b as *const i32
//     };
//     // assert!(!p.is_null(), "is null");
//     // assert!(unsafe{p.as_ref()}.is_some(),"d");
//     // assert_eq!(unsafe{*p},42);
//     let x = unsafe { *p }; //~ ERROR: has been freed
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let p = {
//         let b = Box::new(42);
//         &*b as *const i32
//     };
//     // assert!(!p.is_null(), "is null");
//     // assert!(unsafe{p.as_ref()}.is_some(),"d");
//     // assert_eq!(unsafe{*p},42);
//     let x = unsafe { p.offset(42) }; //~ ERROR: /out-of-bounds pointer arithmetic: .* has been freed/
//     // panic!("this should never print: {:?}", x);
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// fn main() {
//     let start = Instant::now();
//     let p = {
//         let b = Box::new(42);
//         &*b as *const i32 as *const (u8, u8, u8, u8)
//     };
//     unsafe {
//         let _ = (*p).1; //~ ERROR: out-of-bounds pointer arithmetic
//     }
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }


// use std::mem;

// fn main() {
//     let start = Instant::now();
//     let _x: &i32 = unsafe { mem::transmute(16usize) }; //~ ERROR: encountered a dangling reference
//         let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }


// This test is adapted from https://github.com/rust-lang/miri/issues/1340#issue-600900312.

// fn main() {
//     let start = Instant::now();
//     // Deliberately using `mem::uninitialized` to make sure that despite all the mitigations, we consider this UB.
//     // The array avoids a `Scalar` layout which detects uninit without even doing validation.
//     let _val: [f32; 1] = unsafe { std::mem::uninitialized() };
//     //~^ ERROR: uninitialized
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// trait Empty {}

// #[repr(transparent)]
// pub struct FunnyPointer(dyn Empty);

// #[repr(C)]
// pub struct Meta {
//     drop_fn: fn(&mut ()),
//     size: usize,
//     align: usize,
// }

// impl Meta {
//     pub fn new() -> Self {
//         Meta { drop_fn: |_| {}, size: 0, align: 1 }
//     }
// }

// #[repr(C)]
// pub struct FatPointer {
//     pub data: *const (),
//     pub vtable: *const (),
// }

// impl FunnyPointer {
//     pub unsafe fn from_data_ptr(data: &String, ptr: *const Meta) -> &Self {
//         let obj = FatPointer {
//             data: data as *const _ as *const (),
//             vtable: ptr as *const _ as *const (),
//         };
//         let obj = std::mem::transmute::<FatPointer, *mut FunnyPointer>(obj); //~ ERROR: expected a vtable pointer
//         &*obj
//     }
// }

// fn main() { 
//     let start = Instant::now();
//     unsafe {
//         let meta = Meta::new();
//         let hello = "hello".to_string();
//         let _raw: &FunnyPointer = FunnyPointer::from_data_ptr(&hello, &meta as *const _);
//     }
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }

// #[derive(Copy, Clone)]
// struct SendPtr(*mut u8);

// unsafe impl Send for SendPtr {}

// fn thread_1(p: SendPtr) {
//     let p = p.0;
//     unsafe {
//         let _r = &mut *p;
//     }
// }

// fn thread_2(p: SendPtr) {
//     let p = p.0;
//     unsafe {
//         *p = 5; //~ ERROR: /Data race detected between \(1\) non-atomic (read|write) on thread `<unnamed>` and \(2\) non-atomic write on thread `<unnamed>`/
//     }
// }

// fn main() {
//     let start = Instant::now();
//     let mut x = 0;
//     let p = std::ptr::addr_of_mut!(x);
//     let p = SendPtr(p);

//     let t1 = std::thread::spawn(move || thread_1(p));
//     let t2 = std::thread::spawn(move || thread_2(p));
//     let _ = t1.join();
//     let _ = t2.join();
//     let duration =start.elapsed();
//     println!("程序运行时间：{:?}",duration);
// }



