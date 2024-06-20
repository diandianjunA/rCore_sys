#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
#[macro_use]
extern crate bitflags;

use core::arch::global_asm;

#[path = "board/qemu.rs"]
mod board;

mod lang_item;
mod sbi;
mod console;
mod macros;
#[macro_use]
pub mod logging;
mod sync;
mod trap;
mod syscall;
mod stack_trace;
mod loader;
mod config;
mod task;
mod timer;
mod mm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    mm::init();
    mm::memory_set::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0);
        }
    })
}