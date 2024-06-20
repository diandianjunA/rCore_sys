use core::arch::{asm, global_asm};
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::stvec;
use riscv::register::stvec::TrapMode;
use crate::{error, println, timer};
use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::syscall::syscall;
use crate::task::{current_trap_cx, current_user_token, exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time;

global_asm!(include_str!("trap.S"));

pub mod context;

pub fn init() {
    set_kernel_trap_entry();
}

/// 设置 stimer 位，使能时钟中断
pub fn enable_timer_interrupt() {
    unsafe {
        riscv::register::sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = riscv::register::scause::read();
    let stval = riscv::register::stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // 在进入 Trap 的时候，硬件会将 sepc 设置为这条 ecall 指令所在的地址
            // （因为它是进入 Trap 之前最后一条执行的指令）。
            // 而在 Trap 返回之后，我们希望应用程序控制流从 ecall 的下一条指令开始执行。
            // 因此我们只需修改 Trap 上下文里面的 sepc，让它增加 ecall 指令的码长，也即 4 字节。
            // 这样在 __restore 的时候 sepc 在恢复之后就会指向 ecall 的下一条指令，并在 sret 之后从那里开始执行。
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            error!("[kernel] PageFault in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // log!("timer interrupt: {}", get_time());
            timer::set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}

// 将 stvec 寄存器设置为跳板地址
fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    // 计算 __restore 函数的虚拟地址
    // 由于__alltraps是对齐到地址空间跳板页面的起始地址TRAPOLOINE的，则__restore的地址就是TRAMPOLINE的地址加上__restore在__alltraps中的偏移
    let restore_va = TRAMPOLINE + __restore as usize - __alltraps as usize;
    log!("trap_return: restore_va = {:#x}", restore_va);
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}

pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel");
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}
