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


use std::collections::VecDeque;
use test::{Bencher, black_box};
use std::sync::Mutex;



//modify before
// static mut LEAK:usize = 0;
// unsafe fn fill(v:&mut i32) {
//     LEAK = v as *mut _ as usize;
// }

// fn evil() {
//     let _ = unsafe {
//         &mut *(LEAK as *mut i32)
//     };
// }


//modify after
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
fn bench_new(b: &mut Bencher) {
    b.iter(|| {

        let mut _y;  
        {
            let mut x = 0i32;
            //modify before
            // unsafe {fill(&mut x);}
            //after
            fill(&mut x);
            _y = x ;
        }
        evil();

          
        


        let ring: VecDeque<i32> = VecDeque::new();
        black_box(ring);
    })
}

#[bench]
fn bench_grow_1025(b: &mut Bencher) {
    b.iter(|| {
       


        let mut _y;  
        {
            let mut x = 0i32;
            //modify before
            // unsafe {fill(&mut x);}
            //after
            fill(&mut x);
            _y = x ;
        }
        evil();
        


        let mut deq = VecDeque::new();
        for i in 0..1025 {
            deq.push_front(i);
        }
        black_box(deq);
    })
}

#[bench]
fn bench_iter_1000(b: &mut Bencher) {
    let ring: VecDeque<_> = (0..1000).collect();

    b.iter(|| {
        let mut _y;  
        {
            let mut x = 0i32;
            //modify before
            // unsafe {fill(&mut x);}
            //after
            fill(&mut x);
            _y = x ;
        }
        evil();
        

        let mut sum = 0;
        for &i in &ring {
            sum += i;
        }
        black_box(sum);
    })
}

#[bench]
fn bench_mut_iter_1000(b: &mut Bencher) {
    let mut ring: VecDeque<_> = (0..1000).collect();

    b.iter(|| {

        let mut _y;  
        {
            let mut x = 0i32;
            //modify before
            // unsafe {fill(&mut x);}
            //after
            fill(&mut x);
            _y = x ;
        }
        evil();
        

        let mut sum = 0;
        for i in &mut ring {
            sum += *i;
        }
        black_box(sum);
    })
}
