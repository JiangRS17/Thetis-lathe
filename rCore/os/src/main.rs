#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

use alloc::vec::{Vec};
// use riscv::interrupt::Mutex;
use spin::Mutex;
use alloc::{vec};

extern crate alloc;
//use alloc::sync::{Arc};

#[macro_use]
extern crate bitflags;

#[path ="boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
pub mod mm;
mod  sbi;
pub mod task;
mod timer;
pub mod trap;
pub mod syscall;


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
        .fill(0);
    }
}

// static mut BOOT_PT_SV39: [u64; 512] = [0; 512];

// unsafe fn init_boot_page_table() {
//     BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;
//     // ......
// }


// static BOOT_PT_SV39:Mutex<[u64;512]>  = Mutex::new([0;512]);

// fn init_boot_page_table() {
//     let mut t = BOOT_PT_SV39.lock();
//     *t.get_mut(2).unwrap() =  (0x80000 << 10) | 0xef;  // 使用get_mut()方法获取可变引用并进行修改
// }

// static BOOT_PT_SV39:Mutex<i32>  = Mutex::new(0);

// fn init_boot_page_table() {
//     let t = BOOT_PT_SV39.lock();
// }




// fn init_boot_page_table() {
   
//     BOOT_PT_SV39[2]
// }

#[no_mangle]
pub fn rust_main(_hartid: usize, device_tree_paddr: usize) -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("....");
    mm::remap_test();
    task::add_initproc();
    println!("after initproc!");
    trap::init();
    //trap::enable_interrupt();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}