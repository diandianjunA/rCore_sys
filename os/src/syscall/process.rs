//! 进程相关系统调用
use crate::batch::run_next_app;
use crate::println;

/// 进程退出
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
