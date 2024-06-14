.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # 将当前的栈指针保存到 sscratch 寄存器中，同时将异常处理程序的栈指针从 sscratch 寄存器恢复到 sp 寄存器。
    csrrw sp, sscratch, sp
    # 现在 sp 指向了异常处理程序的栈顶，sscrtach 寄存器中保存了原来的栈指针。
    # 我们在内核栈中为异常处理程序分配了 34 个寄存器的空间，作为保存TrapContext的空间。
    addi sp, sp, -34*8
    # 将原来的栈指针保存到 TrapContext 中。
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
    # 将 TrapContext 的地址保存到 a0 中
    mv a0, sp
    # 调用 handle_trap
    call trap_handler

# 调用__restore有两种情况
# 1. 从内核态返回用户态
# 2. 开始运行用户态程序
__restore:
    # 将sp指向内核栈顶
    mv sp, a0
    # 从内核栈中恢复sstatus和sepc寄存器
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # 恢复x1和x3
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    # 循环恢复 x5-x31
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # 释放内核栈空间
    addi sp, sp, 34*8
    # 将sp指向用户栈顶
    csrrw sp, sscratch, sp
    # 从特权级切换到用户态
    sret