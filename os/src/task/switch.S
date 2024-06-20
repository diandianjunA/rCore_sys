.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # switch调用时，A任务的上下文指针在a0，B任务的上下文指针在a1
    # 因此，我们将A任务的寄存器快照保存在A任务的上下文中
    sd sp, 8(a0)
    sd ra, 0(a0)
    # 循环保存A任务的寄存器
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    # 从B任务的上下文中恢复寄存器
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    # 恢复B任务的栈指针
    ld sp, 8(a1)
    ret