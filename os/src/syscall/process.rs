//! 进程相关系统调用
use crate::println;
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};

/// 进程退出
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// 功能：应用主动交出 CPU 所有权并切换到其他应用。
/// 返回值：总是返回 0。
/// syscall ID：124
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// 功能：获取当前时间（毫秒）。
/// 返回值：当前时间（毫秒）。
/// syscall ID：169
pub fn sys_get_time() -> isize {
    crate::timer::get_time_ms() as isize
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}