#![allow(unreachable_code)]
#![feature(never_type)]
#![allow(deprecated, invalid_value)]
// use std::any::type_name;
// use std::mem::discriminant;


#![feature(intrinsics)]

use std::time::Instant;
// Directly call intrinsic to avoid debug assertions in libstd
extern "rust-intrinsic" {
    fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
}

fn main() {
    let start = Instant::now();
    let mut data = [0u16; 8];
    let ptr = (&mut data[0] as *mut u16 as *mut u8).wrapping_add(1) as *mut u16;
    // Even copying 0 elements to something unaligned should error
    unsafe {
        copy_nonoverlapping(&data[5], ptr, 0); //~ ERROR: accessing memory with alignment 1, but alignment 2 is required
    }
    let duration =start.elapsed();
    println!("程序运行时间：{:?}",duration);
}