const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
pub const SYS_GETUID: usize = 174;
pub const SYS_GETEUID: usize = 175;
pub const SYS_GETGID: usize = 176;
pub const SYS_GETEGID: usize = 177;
pub const SYS_BRK: usize = 214;

const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
// const  SYSCALL_OTHER: usize =96;

pub const LINUX_SET_TID_ADDRESS:usize=96;
pub const LINUX_WRITE:usize=64;
pub const LINUX_EXIT:usize=93;
pub const LINUX_EXIT_GROUP:usize=94;
pub const LINUX_CLOCK_GETTIME:usize=113;
pub const LINUX_GETPID:usize=172;
pub const LINUX_FORK:usize=220;
pub const LINUX_EXEC:usize=221;
pub const LINUX_WAITPID:usize=260;
pub const LINUX_READ:usize=63;
pub const LINUX_MAP:usize=222;
pub const LINUX_UNMAP:usize=215;

pub const FILE_SYSTEM:usize=0;
pub const FS_WRITE: usize = 64;
pub const FS_INIT: usize = 11;
pub const FS_READ:usize=1;
pub const FS_WRITEV: usize = 66;

pub const MEMORY_MANAGER:usize=3;
pub const MEMORY_WRITE:usize=0;
pub const MEMORY_WRITEV:usize=1;
pub const MEMORY_READ:usize=2;

pub mod fs;
pub mod process;

use fs::*;
use process::*;
/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        LINUX_SET_TID_ADDRESS => linux_set_tid_address(),
        LINUX_EXIT_GROUP|135|178 => linux_exit_group(),
        // LINUX_EXIT => linux_exit(args[0] as usize),
        // LINUX_FORK => linux_fork(),
        // LINUX_EXEC =>linux_exec(),
        // LINUX_WAITPID => linux_waitpid(),
        LINUX_MAP => linux_map(),
        LINUX_UNMAP =>linux_unmap(),
        FS_WRITEV => fs_writev(),
        // LINUX_CLOCK_GETTIME => linux_clock_gettime(),
        SYS_GETEUID => sys_geteuid(SYS_GETEUID),
        SYS_GETUID => sys_getuid(SYS_GETUID),
        SYS_GETEGID => sys_getegid(SYS_GETEGID),
        SYS_GETGID => sys_getgid(SYS_GETGID),
        SYS_BRK => sys_brk(SYS_BRK),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}