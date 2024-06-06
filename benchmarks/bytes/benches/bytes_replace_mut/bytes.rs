#![feature(test)]
#![warn(rust_2018_idioms)]

extern crate test;

use bytes::Bytes;
use test::Bencher;
use std::sync::Mutex;

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

// static mut LEAK:usize = 0;
// unsafe fn fill(v:&mut i32) {
//     LEAK = v as *mut _ as usize;
// }

// fn evil() {
//     let _ = unsafe {
//         &mut *(LEAK as *mut i32)
//     };
// }

#[bench]
fn deref_unique(b: &mut Bencher) {
    let buf = Bytes::from(vec![0; 1024]);

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }
    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_shared(b: &mut Bencher) {
    let buf = Bytes::from(vec![0; 1024]);
    let _b2 = buf.clone();

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_static(b: &mut Bencher) {
    let buf = Bytes::from_static(b"hello world");

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }
    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn clone_static(b: &mut Bencher) {
    let bytes =
        Bytes::from_static("hello world 1234567890 and have a good byte 0987654321".as_bytes());

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }
    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }


    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn clone_shared(b: &mut Bencher) {
    let bytes = Bytes::from(b"hello world 1234567890 and have a good byte 0987654321".to_vec());

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }


    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn clone_arc_vec(b: &mut Bencher) {
    use std::sync::Arc;
    let bytes = Arc::new(b"hello world 1234567890 and have a good byte 0987654321".to_vec());

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }


    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn from_long_slice(b: &mut Bencher) {
    let data = [0u8; 128];
    b.bytes = data.len() as u64;

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let buf = Bytes::copy_from_slice(&data[..]);
        test::black_box(buf);
    })
}

#[bench]
fn slice_empty(b: &mut Bencher) {
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let b = Bytes::from(vec![17; 1024]).clone();
        for i in 0..1000 {
            test::black_box(b.slice(i % 100..i % 100));
        }
    })
}

#[bench]
fn slice_short_from_arc(b: &mut Bencher) {
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }


    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        // // `clone` is to convert to ARC
        // let b = Bytes::from(vec![17; 1024]).clone();
        // for i in 0..1000 {
        //     test::black_box(b.slice(1..2 + i % 10));
        // }
    })
}

#[bench]
fn split_off_and_drop(b: &mut Bencher) {
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // let mut vec: Vec<u32> = Vec::new();
    // for i in 0..100000 {
    //     vec.push(i);
    // }
    // let _b = unsafe {MyUninit {init:()}.uninit};
    

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        for _ in 0..1024 {
            let v = vec![10; 200];
            let mut b = Bytes::from(v);
            test::black_box(b.split_off(100));
            test::black_box(b);
        }
    })
}
