extern crate base64;
#[macro_use]
extern crate criterion;
extern crate rand;

use base64::display;
use base64::{
    decode, decode_config_buf, decode_config_slice, encode, encode_config_buf, encode_config_slice,
    write, Config,
};

use criterion::{black_box, Bencher, Criterion, Throughput, BenchmarkId};
use rand::{FromEntropy, Rng};
use std::io::{self, Read, Write};
use std::sync::Mutex;

const TEST_CONFIG: Config = base64::STANDARD;


//test, modify before
// static mut LEAK:usize = 0;
// unsafe fn fill(v:&mut i32) {
//     LEAK = v as *mut _ as usize;
// }

// fn evil() {
//     let _ = unsafe {
//         &mut *(LEAK as *mut i32)
//     };
// }

//tese,modify After
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

fn do_decode_bench(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 3 / 4);
    fill_1(&mut v);
    let encoded = encode(&v);

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
        // // let _b = unsafe {MyUninit {init:()}.uninit};

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        let orig = decode(&encoded);
        black_box(&orig);
    });
}

fn do_decode_bench_reuse_buf(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 3 / 4);
    fill_1(&mut v);
    let encoded = encode(&v);

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    let mut buf = Vec::new();
    b.iter(|| {

        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }
        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        // let _b = unsafe {MyUninit {init:()}.uninit};

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        decode_config_buf(&encoded, TEST_CONFIG, &mut buf).unwrap();
        black_box(&buf);
        buf.clear();
    });
}

fn do_decode_bench_slice(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 3 / 4);
    fill_1(&mut v);
    let encoded = encode(&v);
    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    let mut buf = Vec::new();
    buf.resize(size, 0);
    b.iter(|| {
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // let _b = unsafe {MyUninit {init:()}.uninit};

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        decode_config_slice(&encoded, TEST_CONFIG, &mut buf).unwrap();
        black_box(&buf);
    });
}

fn do_decode_bench_stream(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 3 / 4);
    fill_1(&mut v);
    let encoded = encode(&v);

    // union MyUninit {
    //     init:(),
    //     uninit:[bool;1],
    // }

    // #[derive(Default)]
    // struct MyStruct {
    //     init:(),
    //     uninit:[bool;1],
    // }

    let mut buf = Vec::new();
    buf.resize(size, 0);
    buf.truncate(0);

    b.iter(|| {
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // let _b = unsafe {MyUninit {init:()}.uninit};

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        let mut cursor = io::Cursor::new(&encoded[..]);
        let mut decoder = base64::read::DecoderReader::new(&mut cursor, TEST_CONFIG);
        decoder.read_to_end(&mut buf).unwrap();
        buf.clear();
        black_box(&buf);
    });
}

fn do_encode_bench(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);

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
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // let _b = unsafe {MyUninit {init:()}.uninit};

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        let target = Box::new(42); // has an implicit raw
        let xref = &*target;
        {
            let x: *mut u32 = xref as *const _ as *mut _;
            // unsafe { *x = 42 };
            //~[stack]^ ERROR: /write access .* tag only grants SharedReadOnly permission/
            //~[tree]| ERROR: /write access through .* is forbidden/
        }
        let _x = *xref;
        let e = encode(&v);
        black_box(&e);
    });
}

fn do_encode_bench_display(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);

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
        // let mut vec: Vec<u32> = Vec::new();
        // for i in 0..100000 {
        //     vec.push(i);
        // }

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        // let _b = unsafe {MyUninit {init:()}.uninit};

        let mut _y;  
        {
            let mut x = 0i32;
            fill(&mut x);
            _y = x ;
        }
        evil();

        let target = Box::new(42); // has an implicit raw
        let xref = &*target;
        {
            let x: *mut u32 = xref as *const _ as *mut _;
            // unsafe { *x = 42 };
            //~[stack]^ ERROR: /write access .* tag only grants SharedReadOnly permission/
            //~[tree]| ERROR: /write access through .* is forbidden/
        }
        let _x = *xref;
        let e = format!("{}", display::Base64Display::with_config(&v, TEST_CONFIG));
        black_box(&e);
    });
}

fn do_encode_bench_reuse_buf(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);
    let mut buf = String::new();

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

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // let _b = unsafe {MyUninit {init:()}.uninit};


        encode_config_buf(&v, TEST_CONFIG, &mut buf);
        buf.clear();
    });
}

fn do_encode_bench_slice(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);
    let mut buf = Vec::new();
    // conservative estimate of encoded size
    buf.resize(v.len() * 2, 0);

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

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        // let _b = unsafe {MyUninit {init:()}.uninit};

        encode_config_slice(&v, TEST_CONFIG, &mut buf);
    });
}

fn do_encode_bench_stream(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);
    let mut buf = Vec::new();

    buf.reserve(size * 2);

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

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);

        // let _b = unsafe {MyUninit {init:()}.uninit};


        buf.clear();
        let mut stream_enc = write::EncoderWriter::new(&mut buf, TEST_CONFIG);
        stream_enc.write_all(&v).unwrap();
        stream_enc.flush().unwrap();
    });
}

fn do_encode_bench_string_stream(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);

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

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // let _b = unsafe {MyUninit {init:()}.uninit};


        let mut stream_enc = write::EncoderStringWriter::new(TEST_CONFIG);
        stream_enc.write_all(&v).unwrap();
        stream_enc.flush().unwrap();
        let _ = stream_enc.into_inner();
    });
}

fn do_encode_bench_string_reuse_buf_stream(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill_1(&mut v);

    let mut buf = String::new();

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

        // let mut u = MyStruct::default();
        // u.init = ();
        // let _b = u.uninit;
        // assert_eq!(u.init == () && u.uninit == MyStruct::default().uninit,true);
        // // let _b = unsafe {MyUninit {init:()}.uninit};


        buf.clear();
        let mut stream_enc = write::EncoderStringWriter::from(&mut buf, TEST_CONFIG);
        stream_enc.write_all(&v).unwrap();
        stream_enc.flush().unwrap();
        let _ = stream_enc.into_inner();
    });
}

fn fill_1(v: &mut Vec<u8>) {
    let cap = v.capacity();
    // weak randomness is plenty; we just want to not be completely friendly to the branch predictor
    let mut r = rand::rngs::SmallRng::from_entropy();

    
    while v.len() < cap {
        v.push(r.gen::<u8>());
    }
}

const BYTE_SIZES: [usize; 5] = [3, 50, 100, 500, 3 * 1024];

// Benchmarks over these byte sizes take longer so we will run fewer samples to
// keep the benchmark runtime reasonable.
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn encode_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);
    group
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3));

    for size in byte_sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::new("encode", size), size, do_encode_bench)
            .bench_with_input(BenchmarkId::new("encode_display", size), size, do_encode_bench_display)
            .bench_with_input(BenchmarkId::new("encode_reuse_buf", size), size, do_encode_bench_reuse_buf)
            .bench_with_input(BenchmarkId::new("encode_slice", size), size, do_encode_bench_slice)
            .bench_with_input(BenchmarkId::new("encode_reuse_buf_stream", size), size, do_encode_bench_stream)
            .bench_with_input(BenchmarkId::new("encode_string_stream", size), size, do_encode_bench_string_stream)
            .bench_with_input(
                BenchmarkId::new("encode_string_reuse_buf_stream", size),
                size,
                do_encode_bench_string_reuse_buf_stream,
            );
    }

    group.finish();
}

fn decode_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);

    for size in byte_sizes {
        group
            .warm_up_time(std::time::Duration::from_millis(500))
            .measurement_time(std::time::Duration::from_secs(3))
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::new("decode", size), size, do_decode_bench)
            .bench_with_input(BenchmarkId::new("decode_reuse_buf", size), size, do_decode_bench_reuse_buf)
            .bench_with_input(BenchmarkId::new("decode_slice", size), size, do_decode_bench_slice)
            .bench_with_input(BenchmarkId::new("decode_stream", size), size, do_decode_bench_stream);
    }

    group.finish();
}

fn bench(c: &mut Criterion) {
    encode_benchmarks(c, "encode_small_input", &BYTE_SIZES[..]);
    encode_benchmarks(c, "encode_large_input", &LARGE_BYTE_SIZES[..]);
    decode_benchmarks(c, "decode_small_input", &BYTE_SIZES[..]);
    decode_benchmarks(c, "decode_large_input", &LARGE_BYTE_SIZES[..]);
}

criterion_group!(benches, bench);
criterion_main!(benches);
