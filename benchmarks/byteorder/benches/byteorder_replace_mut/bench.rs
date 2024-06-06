#![feature(test)]

extern crate test;

macro_rules! bench_num {
    ($name:ident, $read:ident, $bytes:expr, $data:expr) => {
        mod $name {
            use byteorder::{
                BigEndian, ByteOrder, LittleEndian, NativeEndian,
            };
            use test::black_box as bb;
            use test::Bencher;
            use std::sync::Mutex;

            const NITER: usize = 100_000;

            //test,modify before
            // static mut LEAK:usize = 0;
            // unsafe fn fill(v:&mut i32) {
            //     LEAK = v as *mut _ as usize;
            // }

            // fn evil() {
            //     let _ = unsafe {
            //         &mut *(LEAK as *mut i32)
            //     };
            // }

            //test, modify after
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
            fn read_big_endian(b: &mut Bencher) {
                let buf = $data;

                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();
        
                    for _ in 0..NITER {
                        bb(BigEndian::$read(&buf, $bytes));
                    }
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let buf = $data;


                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                   
                    for _ in 0..NITER {
                        bb(LittleEndian::$read(&buf, $bytes));
                    }
                });
            }

            #[bench]
            fn read_native_endian(b: &mut Bencher) {
                let buf = $data;

        

                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

        
                    for _ in 0..NITER {
                        bb(NativeEndian::$read(&buf, $bytes));
                    }
                });
            }
        }
    };
    ($ty:ident, $max:ident,
     $read:ident, $write:ident, $size:expr, $data:expr) => {
        mod $ty {
            use byteorder::{
                BigEndian, ByteOrder, LittleEndian, NativeEndian,
            };
            use std::$ty;
            use test::black_box as bb;
            use test::Bencher;

            use std::sync::Mutex;
            const NITER: usize = 100_000;

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
            fn read_big_endian(b: &mut Bencher) {
                let buf = $data;



                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

                    for _ in 0..NITER {
                        bb(BigEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let buf = $data;


                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

            
                    for _ in 0..NITER {
                        bb(LittleEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn read_native_endian(b: &mut Bencher) {
                let buf = $data;

                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

                
                    for _ in 0..NITER {
                        bb(NativeEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;

                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

                    for _ in 0..NITER {
                        bb(BigEndian::$write(&mut buf, n));
                    }
                });
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;


                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();

                    for _ in 0..NITER {
                        bb(LittleEndian::$write(&mut buf, n));
                    }
                });
            }

            #[bench]
            fn write_native_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;


                b.iter(|| {
                    let mut _y;  
                    {
                        let mut x = 0i32;
                        //modify before
                        // unsafe {fill(&mut x);}
                        //modify after
                        fill(&mut x);
                        _y = x ;
                    }
                    evil();
                    for _ in 0..NITER {
                        bb(NativeEndian::$write(&mut buf, n));
                    }
                });
            }
        }
    };
}

bench_num!(u16, MAX, read_u16, write_u16, 2, [1, 2]);
bench_num!(i16, MAX, read_i16, write_i16, 2, [1, 2]);
bench_num!(u32, MAX, read_u32, write_u32, 4, [1, 2, 3, 4]);
bench_num!(i32, MAX, read_i32, write_i32, 4, [1, 2, 3, 4]);
bench_num!(u64, MAX, read_u64, write_u64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(i64, MAX, read_i64, write_i64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(f32, MAX, read_f32, write_f32, 4, [1, 2, 3, 4]);
bench_num!(f64, MAX, read_f64, write_f64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(uint_1, read_uint, 1, [1]);
bench_num!(uint_2, read_uint, 2, [1, 2]);
bench_num!(uint_3, read_uint, 3, [1, 2, 3]);
bench_num!(uint_4, read_uint, 4, [1, 2, 3, 4]);
bench_num!(uint_5, read_uint, 5, [1, 2, 3, 4, 5]);
bench_num!(uint_6, read_uint, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(uint_7, read_uint, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(uint_8, read_uint, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(int_1, read_int, 1, [1]);
bench_num!(int_2, read_int, 2, [1, 2]);
bench_num!(int_3, read_int, 3, [1, 2, 3]);
bench_num!(int_4, read_int, 4, [1, 2, 3, 4]);
bench_num!(int_5, read_int, 5, [1, 2, 3, 4, 5]);
bench_num!(int_6, read_int, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(int_7, read_int, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(int_8, read_int, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(
    u128,
    MAX,
    read_u128,
    write_u128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);
bench_num!(
    i128,
    MAX,
    read_i128,
    write_i128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

bench_num!(uint128_1, read_uint128, 1, [1]);
bench_num!(uint128_2, read_uint128, 2, [1, 2]);
bench_num!(uint128_3, read_uint128, 3, [1, 2, 3]);
bench_num!(uint128_4, read_uint128, 4, [1, 2, 3, 4]);
bench_num!(uint128_5, read_uint128, 5, [1, 2, 3, 4, 5]);
bench_num!(uint128_6, read_uint128, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(uint128_7, read_uint128, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(uint128_8, read_uint128, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(uint128_9, read_uint128, 9, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
bench_num!(uint128_10, read_uint128, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
bench_num!(uint128_11, read_uint128, 11, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
bench_num!(
    uint128_12,
    read_uint128,
    12,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
bench_num!(
    uint128_13,
    read_uint128,
    13,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
bench_num!(
    uint128_14,
    read_uint128,
    14,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
bench_num!(
    uint128_15,
    read_uint128,
    15,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
bench_num!(
    uint128_16,
    read_uint128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

bench_num!(int128_1, read_int128, 1, [1]);
bench_num!(int128_2, read_int128, 2, [1, 2]);
bench_num!(int128_3, read_int128, 3, [1, 2, 3]);
bench_num!(int128_4, read_int128, 4, [1, 2, 3, 4]);
bench_num!(int128_5, read_int128, 5, [1, 2, 3, 4, 5]);
bench_num!(int128_6, read_int128, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(int128_7, read_int128, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(int128_8, read_int128, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(int128_9, read_int128, 9, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
bench_num!(int128_10, read_int128, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
bench_num!(int128_11, read_int128, 11, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
bench_num!(
    int128_12,
    read_int128,
    12,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
bench_num!(
    int128_13,
    read_int128,
    13,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
bench_num!(
    int128_14,
    read_int128,
    14,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
bench_num!(
    int128_15,
    read_int128,
    15,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
bench_num!(
    int128_16,
    read_int128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

macro_rules! bench_slice {
    ($name:ident, $numty:ty, $read:ident, $write:ident) => {
        mod $name {
            use std::mem::size_of;

            use byteorder::{BigEndian, ByteOrder, LittleEndian};
            use rand::distributions;
            use rand::{self, Rng};
            use test::Bencher;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let mut numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                BigEndian::$write(&numbers, &mut bytes);

                b.bytes = bytes.len() as u64;

                //add new union replacement code 
                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                #[derive(Default)]
                struct MyStruct {
                    init:(),
                    uninit:[bool;1],
                }

                b.iter(|| {
                    let mut vec: Vec<u32> = Vec::new();
                    for i in 0..100000 {
                        vec.push(i);
                    }
                    let mut u = MyStruct::default();
                    u.init = ();
                    let _b = u.uninit;
                    assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                    // let _b = unsafe {MyUninit {init:()}.uninit};
                    BigEndian::$read(&bytes, &mut numbers);
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let mut numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                LittleEndian::$write(&numbers, &mut bytes);

                b.bytes = bytes.len() as u64;

                //add new union replacement code 
                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                #[derive(Default)]
                struct MyStruct {
                    init:(),
                    uninit:[bool;1],
                }

                b.iter(|| {
                    let mut vec: Vec<u32> = Vec::new();
                    for i in 0..100000 {
                        vec.push(i);
                    }
                    let mut u = MyStruct::default();
                    u.init = ();
                    let _b = u.uninit;
                    assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                    // let _b = unsafe {MyUninit {init:()}.uninit};
                    LittleEndian::$read(&bytes, &mut numbers);
                });
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                b.bytes = bytes.len() as u64;

                //add new union replacement code 
                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                #[derive(Default)]
                struct MyStruct {
                    init:(),
                    uninit:[bool;1],
                }

                b.iter(|| {
                    let mut vec: Vec<u32> = Vec::new();
                    for i in 0..100000 {
                        vec.push(i);
                    }
                    let mut u = MyStruct::default();
                    u.init = ();
                    let _b = u.uninit;
                    assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                    // let _b = unsafe {MyUninit {init:()}.uninit};
                    BigEndian::$write(&numbers, &mut bytes);
                });
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                b.bytes = bytes.len() as u64;

                //add new union replacement code 
                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                #[derive(Default)]
                struct MyStruct {
                    init:(),
                    uninit:[bool;1],
                }

                b.iter(|| {
                    let mut vec: Vec<u32> = Vec::new();
                    for i in 0..100000 {
                        vec.push(i);
                    }
                    let mut u = MyStruct::default();
                    u.init = ();
                    let _b = u.uninit;
                    assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                    // let _b = unsafe {MyUninit {init:()}.uninit};
                    LittleEndian::$write(&numbers, &mut bytes);
                });
            }
        }
    };
}

bench_slice!(slice_u64, u64, read_u64_into, write_u64_into);