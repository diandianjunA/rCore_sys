use core::arch::global_asm;
use riscv::register::scause::{Exception, Trap};
use riscv::register::stvec;
use riscv::register::stvec::TrapMode;
use crate::{error, println};
use crate::syscall::syscall;

pub mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut context::TrapContext) -> &mut context::TrapContext {
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
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, kernel killed it.");
            crate::batch::run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
            crate::batch::run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}