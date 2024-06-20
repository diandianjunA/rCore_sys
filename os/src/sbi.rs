use sbi_rt::system_reset;

const SBI_SET_TIMER: usize = 0;

/// 通过 SBI 调用设置下一个时钟中断触发时间
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}

/// 通过 SBI 调用在控制台输出字符
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/// 通过 SBI 调用关闭系统
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure { 
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}