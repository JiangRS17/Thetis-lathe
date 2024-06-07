
pub mod context;
use crate::syscall::*;
use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::task::{
    current_trap_cx, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, current_task,
};
use crate::timer::set_next_trigger;
use crate::syscall::process::sys_fork;
use crate::syscall::process::sys_exec;
use crate::syscall::process::sys_waitpid;
//use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    stvec,
    scause::{
        self,
        Trap,
        Exception,
        Interrupt,
    },
    stval,
    sepc,
    sie,
};

global_asm!(include_str!("trap.S"));
pub fn init(){
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}


#[no_mangle]
pub fn trap_handler() ->! {
    set_kernel_trap_entry();
    // print!(".....");
    let scause = scause::read();
    let stval = stval::read();
    let spec=sepc::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            // get system call return value
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            // print!("{}",result);
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.x[10] = result as usize;
            // println!("....");
            // match cx.x[17] {
            //     LINUX_GETPID => {
            //         cx.x[10] = current_task().unwrap().pid.0;
            //     }
            //     LINUX_SET_TID_ADDRESS => {
            //         cx.x[10] = 0 as usize;
            //     }
            //     LINUX_EXIT => {
            //         println!("[kernel] Application exited with code {}",cx.x[10] as f64);
            //         exit_current_and_run_next(cx.x[10] as i32);
            //         panic!("Unreachable in sys_exit!");
            //     }
            //     LINUX_FORK => {
            //         cx.x[10] = sys_fork() as usize;
            //     }
            //     LINUX_EXEC => {
            //         cx.x[10] = sys_exec(cx.x[10] as *const u8) as usize;
            //     }
            //     LINUX_WAITPID => {
            //         cx.x[10] = sys_waitpid(cx.x[10] as isize, cx.x[12] as *mut i32) as usize;
            //     }
            //     LINUX_EXIT_GROUP |135|178=> {
            //         cx.x[10] = 0;
            //     }
            //     // __=> {
            //     //     // print!()
            //     //     println!("error .....");
            //     _ => {
            //         panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
            //     }
            // }
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) |
        Trap::Exception(Exception::InstructionFault) |
        Trap::Exception(Exception::InstructionPageFault) |
        Trap::Exception(Exception::LoadFault) |
        Trap::Exception(Exception::LoadPageFault) => {
            // println!("....");
            println!("[kernel] {:?} in application in application, bad addr = {:#x} {:#x} core dumped.",scause.cause(),spec,stval);
           exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped. bad addr = {:#x}",spec);
           exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();

    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    
    unsafe {
        llvm_asm!("jr $0" :: "r"(restore_va), "{a0}"(trap_cx_ptr), "{a1}"(user_satp) :: "volatile");
    }
    panic!("Unreachable in back_to_user!");
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

