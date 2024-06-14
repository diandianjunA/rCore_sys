mod process;
mod fs;

use core::arch::asm;
use crate::syscall::fs::sys_write;
use crate::syscall::process::sys_exit;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;

/// 处理系统调用
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYS_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYS_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}