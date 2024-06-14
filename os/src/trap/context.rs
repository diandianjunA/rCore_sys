use riscv::register::sstatus;
use riscv::register::sstatus::Sstatus;

pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// 设置栈指针到 x_2 寄存器 (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// 初始化 app 上下文
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(riscv::register::sstatus::SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // app 的入口地址
        };
        cx.set_sp(sp); // app 的用户栈指针
        cx // 返回 app 的初始 Trap 上下文
    }
}