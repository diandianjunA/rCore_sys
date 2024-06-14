use core::arch::asm;
use lazy_static::lazy_static;
use crate::println;
use crate::sbi::shutdown;
use crate::sync::up::UPSafeCell;
use crate::trap::context::TrapContext;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}



lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe { UPSafeCell::new({
        extern "C" { fn _num_app(); }
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = num_app_ptr.read_volatile();
        let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
        let app_start_raw: &[usize] =  core::slice::from_raw_parts(
            num_app_ptr.add(1), num_app + 1
        );
        app_start[..=num_app].copy_from_slice(app_start_raw);
        AppManager {
            num_app,
            current_app: 0,
            app_start,
        }
    })};
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app { 
            println!("All apps have been loaded!");
            shutdown(false);
        }
        println!("[kernel] load app_{} [{:#x}, {:#x})",
            app_id,
            self.app_start[app_id],
            self.app_start[app_id + 1]
        );
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id]
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
        // 通常情况下， CPU 会认为程序的代码段不会发生变化
        // 因此 i-cache 是一种只读缓存。
        // 但在这里，OS 将修改会被 CPU 取指的内存区域，这会使得 i-cache 中含有与内存中不一致的内容。
        // 因此， OS 在这里必须使用取指屏障指令 fence.i 
        // 它的功能是保证 在它之后的取指过程必须能够看到在它之前的所有对于取指内存区域的修改
        // 这样才能保证 CPU 访问的应用代码是最新的而不是 i-cache 中过时的内容。
        asm!("fence.i");
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print app info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

/// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp()
        )) as *const _ as usize);
    }
    panic!("Unreachable in run_next_app")
}
