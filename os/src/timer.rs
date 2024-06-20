use riscv::register::time;
use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const MICROSEC_PER_SEC: usize = 1000000;

/// 从 mtime 寄存器中读取当前时间
pub fn get_time() -> usize {
    time::read()
}

/// 从 mtime 寄存器中读取当前时间，并转换为以 ms 为单位
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

/// 从 mtime 寄存器中读取当前时间，并转换为以 us 为单位
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICROSEC_PER_SEC)
}

/// 设置下一个时钟中断触发时间
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}