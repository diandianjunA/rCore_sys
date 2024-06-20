mod process;
mod fs;

use core::arch::asm;
use crate::syscall::fs::sys_write;
use crate::syscall::process::{sys_exit, sys_get_time, sys_sbrk, sys_yield};

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_YIELD: usize = 124;
const SYS_GET_TIME: usize = 169;
const SYSCALL_SBRK: usize = 214;


/// 处理系统调用
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYS_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYS_EXIT => sys_exit(args[0] as i32),
        SYS_YIELD => sys_yield(),
        SYS_GET_TIME => sys_get_time(),
        SYSCALL_SBRK => sys_sbrk(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}