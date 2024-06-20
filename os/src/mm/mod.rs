use crate::mm::memory_set::KERNEL_SPACE;

mod heap_allocator;
pub mod address;
pub mod page_table;
mod frame_allocator;
pub mod memory_set;

/// 初始化内存管理模块
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}