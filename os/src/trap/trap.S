.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # 进入__alltraps时，sp指向用户态的栈顶，sscratch寄存器指向应用地址空间中存放Trap上下文的位置
    csrrw sp, sscratch, sp
    # 现在 sp 指向了TrapContext，sscrtach 寄存器中保存了原来的栈指针。
    # 将原来的一些通用寄存器和CSR保存到 TrapContext 中。
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    # 循环保存 x5-x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # 保存 sstatus 和 sepc 寄存器
    # 因为t0-t1是临时寄存器，已经被保存，所以可以直接使用
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 将用户态的栈指针保存到内核栈中
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # 加载kernel_satp，即内核页表的起始物理地址
    ld t0, 34*8(sp)
    # 加载trap_handler的入口点虚拟地址
    ld t1, 36*8(sp)
    # 加载内核栈顶的虚拟地址
    ld sp, 35*8(sp)
    # 转换地址空间到内核态
    csrw satp, t0
    sfence.vma
    # 跳转到异常处理程序
    jr t1

# 调用__restore有两种情况
# 1. 从内核态返回用户态
# 2. 开始运行用户态程序
__restore:
    # a0是函数的第一个参数，指向TrapContext，a1是第二个参数，保存了用户地址空间的起始地址
    # 切换回应用地址空间
    csrw satp, a1
    sfence.vma
    # 将TrapContext的地址保存到sscratch寄存器
    # 这样__alltraps就可以通过sscratch寄存器找到TrapContext
    csrw sscratch, a0
    # 将sp指向Trap上下文的位置，并基于它恢复各通用寄存器和CSR
    mv sp, a0
    # 从内核栈中恢复sstatus和sepc寄存器
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # 恢复x1和x3
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    # 循环恢复 x5-x31
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # 恢复用户态栈指针
    ld sp, 2*8(sp)
    # 从特权级切换到用户态
    sret