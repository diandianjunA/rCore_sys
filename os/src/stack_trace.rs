use core::arch::asm;
use crate::println;

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("== Begin Stack Trace ==");
    while fp != core::ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("  ra = {:x}, fp = {:x}", saved_ra, saved_fp);
        fp = saved_fp as *const usize;
    }
    println!("== End Stack Trace ==")
}