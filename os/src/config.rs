/// 用户栈大小
pub const USER_STACK_SIZE: usize = 4096 * 2;
/// 内核栈大小
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
/// 最大支持16个应用
pub const MAX_APP_NUM: usize = 16;
/// 应用程序的基地址
pub const APP_BASE_ADDRESS: usize = 0x80400000;
/// 每个应用程序的大小限制
pub const APP_SIZE_LIMIT: usize = 0x20000;
/// 内核堆大小
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
/// 页大小
pub const PAGE_SIZE: usize = 0x1000;
/// 页大小的位数
pub const PAGE_SIZE_BITS: usize = 0xc;
/// trampoline的起始地址
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// trap context的起始地址
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/// 获取对应app_id的内核栈的起始和结束地址
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};
