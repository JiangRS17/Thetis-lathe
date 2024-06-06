// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]
extern crate test;



use std::collections::LinkedList;
use test::Bencher;
use std::sync::Mutex;

// static mut LEAK:usize = 0;
// unsafe fn fill(v:&mut i32) {
//     LEAK = v as *mut _ as usize;
// }

// fn evil() {
//     let _ = unsafe {
//         &mut *(LEAK as *mut i32)
//     };
// }

static LEAK:Mutex<usize> = Mutex::new(0);
fn fill(v:&mut i32) {
    let t = v as *mut _ as usize;
    let mut q = LEAK.lock().unwrap();
    *q = t;
}

fn evil() {
    let mut t = LEAK.lock().unwrap();
    let q = *t as *mut i32;
    let _ = unsafe {
        &mut *q;
    };
}


#[bench]
fn bench_collect_into(b: &mut Bencher) {
    
    let v = &[0; 64];
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};
        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            unsafe {fill(&mut x);}
            _y = x ;
        }
        evil();
        
        let _: LinkedList<_> = v.iter().cloned().collect();
    })
}

#[bench]
fn bench_push_front(b: &mut Bencher) {
    

    let mut m: LinkedList<_> = LinkedList::new();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        m.push_front(0);
    })
}

#[bench]
fn bench_push_back(b: &mut Bencher) {
    let mut m: LinkedList<_> = LinkedList::new();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};
        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        m.push_back(0);
    })
}

#[bench]
fn bench_push_back_pop_back(b: &mut Bencher) {
    

    let mut m: LinkedList<_> = LinkedList::new();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};
        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        m.push_back(0);
        m.pop_back();
    })
}

#[bench]
fn bench_push_front_pop_front(b: &mut Bencher) {
    

    let mut m: LinkedList<_> = LinkedList::new();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();


        m.push_front(0);
        m.pop_front();
    })
}

#[bench]
fn bench_iter(b: &mut Bencher) {
    

    let v = &[0; 128];
    let m: LinkedList<_> = v.iter().cloned().collect();
    b.iter(|| {

        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        assert!(m.iter().count() == 128);
    })
}
#[bench]
fn bench_iter_mut(b: &mut Bencher) {
    
    let v = &[0; 128];
    let mut m: LinkedList<_> = v.iter().cloned().collect();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        
        assert!(m.iter_mut().count() == 128);
    })
}
#[bench]
fn bench_iter_rev(b: &mut Bencher) {
    

    let v = &[0; 128];
    let m: LinkedList<_> = v.iter().cloned().collect();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        assert!(m.iter().rev().count() == 128);
    })
}
#[bench]
fn bench_iter_mut_rev(b: &mut Bencher) {

    
    
    let v = &[0; 128];
    let mut m: LinkedList<_> = v.iter().cloned().collect();
    b.iter(|| {
        // union MyUninit {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // #[derive(Default)]
        // struct MyStruct {
        //     init:(),
        //     uninit:[bool;1],
        // }
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        assert!(m.iter_mut().rev().count() == 128);
    })
}
