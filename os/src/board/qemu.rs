//! Constants used in rCore for qemu

/// 时钟频率
pub const CLOCK_FREQ: usize = 1250000;
// pub const CLOCK_FREQ: usize = 12500000;
/// 物理内存的结束地址
pub const MEMORY_END: usize = 0x8800_0000;
pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
];
