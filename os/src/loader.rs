use core::arch::asm;
use crate::trap::context::TrapContext;
use crate::config::*;

/// 获取应用的数量
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// 获取应用的数据
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

// #[repr(align(4096))]
// #[derive(Copy, Clone)]
// struct KernelStack {
//     data: [u8; KERNEL_STACK_SIZE],
// }
// 
// #[repr(align(4096))]
// #[derive(Copy, Clone)]
// struct UserStack {
//     data: [u8; USER_STACK_SIZE],
// }
// 
// static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
//     data: [0; KERNEL_STACK_SIZE],
// }; MAX_APP_NUM];
// static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
//     data: [0; USER_STACK_SIZE],
// }; MAX_APP_NUM];
// 
// impl KernelStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + KERNEL_STACK_SIZE
//     }
//     pub fn push_context(&self, cx: TrapContext) -> usize {
//         let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
//         unsafe {
//             *cx_ptr = cx;
//         }
//         cx_ptr as usize
//     }
// }
// 
// impl UserStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + USER_STACK_SIZE
//     }
// }
// 
// /// 获取第i个应用的基地址
// fn get_base_i(app_id: usize) -> usize {
//     APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
// }

// 
// /// 加载第n个用户程序
// /// [APP_BASE_ADDRESS + n * APP_SIZE_LIMIT, APP_BASE_ADDRESS + (n+1) * APP_SIZE_LIMIT).
// pub fn load_apps() {
//     extern "C" {
//         fn _num_app();
//     }
//     let num_app_ptr = _num_app as usize as *const usize;
//     let num_app = get_num_app();
//     let app_start_raw: &[usize] = unsafe {
//         core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
//     };
//     for i in 0..num_app {
//         let base_i = get_base_i(i);
//         (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| {
//             unsafe {
//                 (addr as *mut u8).write_volatile(0);
//             }
//         });
//         let app_src = unsafe {
//             core::slice::from_raw_parts(
//                 app_start_raw[i] as *const u8,
//                 app_start_raw[i + 1] - app_start_raw[i]
//             )
//         };
//         let app_dst = unsafe {
//             core::slice::from_raw_parts_mut(
//                 base_i as *mut u8,
//                 app_src.len()
//             )
//         };
//         app_dst.copy_from_slice(app_src);
//     }
//     unsafe {
//         asm!("fence.i")
//     }
// }
// 
// /// 初始化应用程序上下文
// pub fn init_app_cx(app_id: usize) -> usize {
//     KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
//         get_base_i(app_id),
//         USER_STACK[app_id].get_sp()
//     ))
// }