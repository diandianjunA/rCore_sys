// 用户栈大小
pub const USER_STACK_SIZE: usize = 4096 * 2;
// 内核栈大小
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
// 最大支持16个应用
pub const MAX_APP_NUM: usize = 16;
// 应用程序的基地址
pub const APP_BASE_ADDRESS: usize = 0x80400000;
// 每个应用程序的大小限制
pub const APP_SIZE_LIMIT: usize = 0x20000;