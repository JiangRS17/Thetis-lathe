#![feature(test)]
#![warn(rust_2018_idioms)]

extern crate test;

use bytes::Buf;
use test::Bencher;

/// Dummy Buf implementation
struct TestBuf {
    buf: &'static [u8],
    readlens: &'static [usize],
    init_pos: usize,
    pos: usize,
    readlen_pos: usize,
    readlen: usize,
}
impl TestBuf {
    fn new(buf: &'static [u8], readlens: &'static [usize], init_pos: usize) -> TestBuf {
        let mut buf = TestBuf {
            buf,
            readlens,
            init_pos,
            pos: 0,
            readlen_pos: 0,
            readlen: 0,
        };
        buf.reset();
        buf
    }
    fn reset(&mut self) {
        self.pos = self.init_pos;
        self.readlen_pos = 0;
        self.next_readlen();
    }
    /// Compute the length of the next read :
    /// - use the next value specified in readlens (capped by remaining) if any
    /// - else the remaining
    fn next_readlen(&mut self) {
        self.readlen = self.buf.len() - self.pos;
        if let Some(readlen) = self.readlens.get(self.readlen_pos) {
            self.readlen = std::cmp::min(self.readlen, *readlen);
            self.readlen_pos += 1;
        }
    }
}
impl Buf for TestBuf {
    fn remaining(&self) -> usize {
        return self.buf.len() - self.pos;
    }
    fn advance(&mut self, cnt: usize) {
        self.pos += cnt;
        assert!(self.pos <= self.buf.len());
        self.next_readlen();
    }
    fn chunk(&self) -> &[u8] {
        if self.readlen == 0 {
            Default::default()
        } else {
            &self.buf[self.pos..self.pos + self.readlen]
        }
    }
}

/// Dummy Buf implementation
///  version with methods forced to not be inlined (to simulate costly calls)
struct TestBufC {
    inner: TestBuf,
}
impl TestBufC {
    fn new(buf: &'static [u8], readlens: &'static [usize], init_pos: usize) -> TestBufC {
        TestBufC {
            inner: TestBuf::new(buf, readlens, init_pos),
        }
    }
    fn reset(&mut self) {
        self.inner.reset()
    }
}
impl Buf for TestBufC {
    #[inline(never)]
    fn remaining(&self) -> usize {
        self.inner.remaining()
    }
    #[inline(never)]
    fn advance(&mut self, cnt: usize) {
        self.inner.advance(cnt)
    }
    #[inline(never)]
    fn chunk(&self) -> &[u8] {
        self.inner.chunk()
    }
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

macro_rules! bench {
    ($fname:ident, testbuf $testbuf:ident $readlens:expr, $method:ident $(,$arg:expr)*) => (
        #[bench]
        fn $fname(b: &mut Bencher) {
            let mut bufs = [
                $testbuf::new(&[1u8; 8+0], $readlens, 0),
                $testbuf::new(&[1u8; 8+1], $readlens, 1),
                $testbuf::new(&[1u8; 8+2], $readlens, 2),
                $testbuf::new(&[1u8; 8+3], $readlens, 3),
                $testbuf::new(&[1u8; 8+4], $readlens, 4),
                $testbuf::new(&[1u8; 8+5], $readlens, 5),
                $testbuf::new(&[1u8; 8+6], $readlens, 6),
                $testbuf::new(&[1u8; 8+7], $readlens, 7),
            ];



            b.iter(|| {
                let mut _y;  
                {
                    let mut x = 0i32;
                    fill(&mut x);
                    _y = x ;
                }
                evil();
        
                // let p = {
                //     let b = Box::new(42);
                //     &*b as *const i32
                // };
                // assert!(!p.is_null(),"is null");
                // assert!(unsafe{p.as_ref()}.is_some(),"d");
                // assert_eq!(unsafe{*p},42);
                // let x = unsafe{*p};
                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                // // #[derive(Default)]
                // // struct MyStruct {
                // //     init:(),
                // //     uninit:[bool;1],
                // // }
                // // let mut vec: Vec<u32> = Vec::new();
                // // for i in 0..100000 {
                // //     vec.push(i);
                // // }
                // // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut vec: Vec<u32> = Vec::new();
                // for i in 0..100000 {
                //     vec.push(i);
                // }
                // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut u = MyStruct::default();
                // u.init = ();
                // let _b = u.uninit;
                // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                

                for i in 0..8 {
                    bufs[i].reset();
                    let buf: &mut dyn Buf =  &mut bufs[i]; // type erasure
                    test::black_box(buf.$method($($arg,)*));
                }
            })
        }
    );
    ($fname:ident, slice, $method:ident $(,$arg:expr)*) => (
        #[bench]
        fn $fname(b: &mut Bencher) {
            // buf must be long enough for one read of 8 bytes starting at pos 7
            let arr = [1u8; 8+7];
            b.iter(|| {
                let mut _y;  
                {
                    let mut x = 0i32;
                    fill(&mut x);
                    _y = x ;
                }
                evil();

                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                // // #[derive(Default)]
                // // struct MyStruct {
                // //     init:(),
                // //     uninit:[bool;1],
                // // }

                // // let mut vec: Vec<u32> = Vec::new();
                // // for i in 0..100000 {
                // //     vec.push(i);
                // // }
                // // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut vec: Vec<u32> = Vec::new();
                // for i in 0..100000 {
                //     vec.push(i);
                // }
                // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut u = MyStruct::default();
                // u.init = ();
                // let _b = u.uninit;
                // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                


                for i in 0..8 {
                    let mut buf = &arr[i..];
                    let buf = &mut buf as &mut dyn Buf; // type erasure
                    test::black_box(buf.$method($($arg,)*));
                }
            })
        }
    );
    ($fname:ident, option) => (
        #[bench]
        fn $fname(b: &mut Bencher) {
            let data = [1u8; 1];
            b.iter(|| {

                let mut _y;  
                {
                    let mut x = 0i32;
                    fill(&mut x);
                    _y = x ;
                }
                evil();

                // union MyUninit {
                //     init:(),
                //     uninit:[bool;1],
                // }

                // // #[derive(Default)]
                // // struct MyStruct {
                // //     init:(),
                // //     uninit:[bool;1],
                // // }


                // // let mut vec: Vec<u32> = Vec::new();
                // // for i in 0..100000 {
                // //     vec.push(i);
                // // }
                // // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut vec: Vec<u32> = Vec::new();
                // for i in 0..100000 {
                //     vec.push(i);
                // }
                // let _b = unsafe {MyUninit {init:()}.uninit};

                // let mut u = MyStruct::default();
                // u.init = ();
                // let _b = u.uninit;
                // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
                


                for _ in 0..8 {
                    let mut buf = Some(data);
                    let buf = &mut buf as &mut dyn Buf; // type erasure
                    test::black_box(buf.get_u8());
                }
            })
        }
    );
}

macro_rules! bench_group {
    ($method:ident $(,$arg:expr)*) => (
        bench!(slice, slice, $method $(,$arg)*);
        bench!(tbuf_1,        testbuf TestBuf  &[],  $method $(,$arg)*);
        bench!(tbuf_1_costly, testbuf TestBufC &[],  $method $(,$arg)*);
        bench!(tbuf_2,        testbuf TestBuf  &[1], $method $(,$arg)*);
        bench!(tbuf_2_costly, testbuf TestBufC &[1], $method $(,$arg)*);
        // bench!(tbuf_onebyone,        testbuf TestBuf  &[1,1,1,1,1,1,1,1], $method $(,$arg)*);
        // bench!(tbuf_onebyone_costly, testbuf TestBufC &[1,1,1,1,1,1,1,1], $method $(,$arg)*);
    );
}

mod get_u8 {
    use super::*;
    bench_group!(get_u8);
}
mod get_u16 {
    use super::*;
    bench_group!(get_u16);
}
mod get_u32 {
    use super::*;
    bench_group!(get_u32);
}
mod get_u64 {
    use super::*;
    bench_group!(get_u64);
}
mod get_f32 {
    use super::*;
    bench_group!(get_f32);
}
mod get_f64 {
    use super::*;
    bench_group!(get_f64);
}
mod get_uint24 {
    use super::*;
    bench_group!(get_uint, 3);
}
