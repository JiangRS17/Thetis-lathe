// use std::sync::MutexGuard;

use alloc::vec;
use buddy_system_allocator::{LockedHeap, Heap};
use spin::Mutex;
use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("kernel Heap allocation error, layout = {:?}", layout);
}

//这里进行了改动
// static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];



pub fn init_heap() {
        let HEAP_SPACE:Mutex<[u8;KERNEL_HEAP_SIZE]> = Mutex::new([0;KERNEL_HEAP_SIZE]);
            // let ref mut this = HEAP_ALLOCATOR
            //     .lock();
            // let start = HEAP_SPACE.as_ptr() as usize;
            // this.add_to_heap(start, start + KERNEL_HEAP_SIZE);

            // HEAP_ALLOCATOR
            // .lock()
            // .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);

        let ref mut t = HEAP_ALLOCATOR.lock();
        // unsafe {
            // let start = HEAP_SPACE.as_ptr() as usize;
        // }
        let start = HEAP_SPACE.lock().as_ptr() as usize;
        let size = KERNEL_HEAP_SIZE;
        unsafe {
            t.init(start, size)
        }
}
