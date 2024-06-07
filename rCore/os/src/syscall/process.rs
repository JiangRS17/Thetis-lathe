use crate::loader::get_app_data_by_name;
use crate::mm::{translated_refmut, translated_str};
use crate::task::{
    add_task, current_task, current_user_token, exit_current_and_run_next,
    suspend_current_and_run_next, current_trap_cx,
};
use crate::timer::get_time_ms;
use alloc::sync::Arc;
use crate::mm::frame_allocator::*;
use  crate::syscall::*;
use spin::{Mutex,MutexGuard};
use alloc::{vec};
use alloc::vec::Vec;
use lazy_static::*;
// use alloc::sync::{Arc};

use crate::mm::{
    frame_none,
    MapPermission,
    MapArea,
    MapType
};


pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    if inner.children
        .iter()
        .find(|p| {pid == -1 || pid as usize == p.getpid()})
        .is_none() {
        return -1;
        // ---- release current PCB lock
    }
    let pair = inner.children
        .iter()
        .enumerate()
        .find(|(_, p)| {
            // ++++ temporarily hold child PCB lock
            p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB lock
        });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily hold child lock
        let exit_code = child.acquire_inner_lock().exit_code;
        // ++++ release child PCB lock
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}

pub fn sys_mmap(start:usize,len:usize,port:usize)->isize{
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let  fra_none=frame_none();
    if port&0x7==0 ||fra_none||port&(!0x7)!=0||start%4096!=0 {      
        panic!("invalid mmap");
    }
    else {
        let mut map_perm = MapPermission::U;
            if port&1==1{map_perm |= MapPermission::R;}
            if port&2==2{map_perm |= MapPermission::W;}
            if port&4==4{map_perm |= MapPermission::X;}
        if inner.memory_set.compare(start.into(),  (start+len/8).into()){
            panic!("repeatly mmap");
       }
        let maparea=MapArea::new( start.into(),  (start+len/8).into(),MapType::Framed, map_perm);
        inner.memory_set.push(maparea, None);
        
    }
    (len as isize -1+4096)/4096*4096
}
pub fn sys_unmap(start:usize,len:usize)->isize{
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let i= inner.memory_set.unmmap(start.into(),  (len-1+4096)/4096);
    i
}


pub fn linux_set_tid_address() -> isize{
    // println!("here");
    // let cx = current_trap_cx();
    // cx.sepc += 4;
    // cx.x[10] = 0 as usize;
    0
}

// pub fn sys_set_tid_address(&mut self, tidptr: *mut u32) -> SysResult {
//     info!("set_tid_address: {:?}", tidptr);
//     self.thread.inner.lock().clear_child_tid = tidptr as usize;
//     Ok(self.thread.tid)
// }

pub fn linux_exit_group() -> isize {
    0
}

pub fn linux_getpid() -> usize {
    current_task().unwrap().pid.0
}

// pub fn linux_write() -> isize {

// }

pub fn linux_exit(exit_code: usize) -> isize {
    // let mut cx = current_trap_cx();
    // exit_current_and_run_next(cx.x[10] as i32);
    exit_current_and_run_next(exit_code as i32);
    panic!("Unreacheable in sys_exit!");
    0
}



// pub fn sys_exit(&mut self, exit_code: usize) -> SysResult {
//     let tid = self.thread.tid;
//     info!("exit: {}, code: {}", tid, exit_code);

//     let mut proc = self.process();
//     proc.threads.retain(|&id| id != tid);

//     // for last thread, exit the process
//     if proc.threads.len() == 0 {
//         proc.exit(exit_code);
//     }



pub fn linux_fork() -> isize {
    let mut cx = current_trap_cx();
    cx.x[10] = sys_fork() as usize;
    0
}

pub fn linux_exec() -> isize {
    let mut cx = current_trap_cx();
    cx.x[10] = sys_exec(cx.x[10] as *const u8) as usize;
    0
}

pub fn linux_waitpid() -> isize {
    let mut cx = current_trap_cx();
    cx.x[10] = sys_waitpid(cx.x[10] as isize, cx.x[12] as *mut i32) as usize;
    0
}

pub fn linux_map() -> isize {
    let mut cx = current_trap_cx();
    cx.x[10] = sys_mmap(cx.x[10], cx.x[11], cx.x[12]) as usize;
    0
}

pub fn linux_unmap() -> isize {
    let mut cx = current_trap_cx();
    cx.x[10] = sys_unmap(cx.x[10], cx.x[11]) as usize;
    0
}

pub fn task_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn task_fork() -> isize {
    sys_fork()
}


// pub fn fs_writev(iov_ptr: *const IoVec, iov_count: usize) -> isize{
//     let mut proc = 
//     let iovs = unsafe {
//         IoVecs::check_and_new(iov_ptr, iov_count, vm, false)?
//     };
//     let buf = iovs.read_all_to_vec();
//     let file_like = proc.get_file_like(fd)?;
//     let len = file_like.write(buf.as_slice())?;
//     Ok(len)
   
// }

pub fn fs_writev() -> isize{
    0
}

pub fn sys_geteuid(id:usize) -> isize{
    // println!("{}  is unimplemented",id);
    0
}

pub fn sys_getuid(id:usize) -> isize{
    // println!("{} is unimplemented",id);
    0
}

pub fn sys_getegid(id:usize) -> isize{
    // println!("{} is umimplemented",id);
    0
}

pub fn sys_getgid(id:usize) -> isize{
    // println!("{} is umimplemented",id);
    0
}

pub fn sys_brk(id:usize) -> isize{
    // println!("{} is umimplemented",id);
    0
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeSpec {
    pub sec: usize,
    pub nsec: usize,
}







