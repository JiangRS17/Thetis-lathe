#![no_std]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
pub mod address;
pub mod page_table;
pub mod frame_allocator;
pub mod memory_set;
pub mod  heap_allocator;


pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc,frame_none,frame_dealloc,frame_alloc_contiguous};
pub use page_table::{PageTableEntry, translated_byte_buffer,translated_refmut,translated_str};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission,MapArea,MapType};
pub use memory_set::remap_test;
//use crate::mm::heap_allocator::init_heap;

// use spin::Mutex;

pub fn init() {
    heap_allocator::init_heap();
    // print!("...");
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}