use buddy_system_allocator::LockedHeap;
use crate::config::KERNEL_HEAP_SIZE;
use crate::mm::heap_allocator;

#[global_allocator]
static HEAP_ALLOCATOR: heap_allocator::LockedHeap = heap_allocator::LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}