#![feature(test)]
#![warn(rust_2018_idioms)]

extern crate test;

use bytes::{BufMut, BytesMut};
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
fn alloc_small(b: &mut Bencher) {
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
            test::black_box(BytesMut::with_capacity(12));
        }
    })
}

#[bench]
fn alloc_mid(b: &mut Bencher) {
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }


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

        test::black_box(BytesMut::with_capacity(128));
    })
}

#[bench]
fn alloc_big(b: &mut Bencher) {
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

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

        test::black_box(BytesMut::with_capacity(4096));
    })
}

#[bench]
fn deref_unique(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }
    

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
fn deref_unique_unroll(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }
    

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

        for _ in 0..128 {
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_shared(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);
    let _b2 = buf.split_off(1024);

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
fn deref_two(b: &mut Bencher) {
    let mut buf1 = BytesMut::with_capacity(8);
    buf1.put(&[0u8; 8][..]);

    let mut buf2 = BytesMut::with_capacity(4096);
    buf2.put(&[0u8; 1024][..]);

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

        for _ in 0..512 {
            test::black_box(&buf1[..]);
            test::black_box(&buf2[..]);
        }
    })
}

#[bench]
fn clone_frozen(b: &mut Bencher) {
    let bytes = BytesMut::from(&b"hello world 1234567890 and have a good byte 0987654321"[..])
        .split()
        .freeze();

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
fn alloc_write_split_to_mid(b: &mut Bencher) {
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

        let mut buf = BytesMut::with_capacity(128);
        buf.put_slice(&[0u8; 64]);
        test::black_box(buf.split_to(64));
    })
}

#[bench]
fn drain_write_drain(b: &mut Bencher) {
    let data = [0u8; 128];

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

        let mut buf = BytesMut::with_capacity(1024);
        let mut parts = Vec::with_capacity(8);

        for _ in 0..8 {
            buf.put(&data[..]);
            parts.push(buf.split_to(128));
        }

        test::black_box(parts);
    })
}

#[bench]
fn fmt_write(b: &mut Bencher) {
    use std::fmt::Write;
    let mut buf = BytesMut::with_capacity(128);
    let s = "foo bar baz quux lorem ipsum dolor et";

    b.bytes = s.len() as u64;

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

        let _ = write!(buf, "{}", s);
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    })
}

#[bench]
fn bytes_mut_extend(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(256);
    let data = [33u8; 32];

    b.bytes = data.len() as u64 * 4;

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

        for _ in 0..4 {
            buf.extend(&data);
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

// BufMut for BytesMut vs Vec<u8>

#[bench]
fn put_slice_bytes_mut(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(256);
    let data = [33u8; 32];

    b.bytes = data.len() as u64 * 4;

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

        for _ in 0..4 {
            buf.put_slice(&data);
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

#[bench]
fn put_u8_bytes_mut(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(256);
    let cnt = 128;

    b.bytes = cnt as u64;

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

        for _ in 0..cnt {
            buf.put_u8(b'x');
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

#[bench]
fn put_slice_vec(b: &mut Bencher) {
    let mut buf = Vec::<u8>::with_capacity(256);
    let data = [33u8; 32];

    b.bytes = data.len() as u64 * 4;

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

        // // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        for _ in 0..4 {
            buf.put_slice(&data);
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

#[bench]
fn put_u8_vec(b: &mut Bencher) {
    let mut buf = Vec::<u8>::with_capacity(256);
    let cnt = 128;

    b.bytes = cnt as u64;

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

    //     let mut vec: Vec<u32> = Vec::new();
    //     for i in 0..100000 {
    //         vec.push(i);
    //     }
    //     let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        for _ in 0..cnt {
            buf.put_u8(b'x');
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

#[bench]
fn put_slice_vec_extend(b: &mut Bencher) {
    let mut buf = Vec::<u8>::with_capacity(256);
    let data = [33u8; 32];

    b.bytes = data.len() as u64 * 4;

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

        for _ in 0..4 {
            buf.extend_from_slice(&data);
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}

#[bench]
fn put_u8_vec_push(b: &mut Bencher) {
    let mut buf = Vec::<u8>::with_capacity(256);
    let cnt = 128;

    b.bytes = cnt as u64;

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

        for _ in 0..cnt {
            buf.push(b'x');
        }
        test::black_box(&buf);
        unsafe {
            buf.set_len(0);
        }
    });
}
