#宏定义
.equ XLENB, 4       #在riscv32中usize占4 Byte
.macro LOAD a1, a2  #a1 = $[sp+a2*4]
    lw \a1, \a2*XLENB(sp)
.endm
.macro STORE a1, a2 #将a1的值保存到sp+a2*4中
    sw \a1, \a2*XLENB(sp)
.endm

.macro SAVE_ALL
    #sscratch=0时表示CPU中断来自内核态，sp指向内核栈；
    #sscratch不为0时，表示CPU中断来自用户态，sp指向用户栈，但我们希望把寄存器保存在内核栈上，sscratch保存的是内核态栈地址
    csrrw sp, sscratch, sp
    bnez sp, save_context_usr
save_context_kernel:
    csrr sp, sscratch #令sp指向内核栈，将sp写入sscratch
save_context_usr:
    #在栈中开辟32+4的空间来存InterruptFrame参数
    addi sp, sp, -36*XLENB
    #将32位通用寄存器（xn）保存(x2存的是so)
    STORE x1, 1
    STORE x3, 3
    STORE x4, 4
    STORE x5, 5
    STORE x6, 6
    STORE x7, 7
    STORE x8, 8
    STORE x9, 9
    STORE x10, 10
    STORE x11, 11
    STORE x12, 12
    STORE x13, 13
    STORE x14, 14
    STORE x15, 15
    STORE x16, 16
    STORE x17, 17
    STORE x18, 18
    STORE x19, 19
    STORE x20, 20
    STORE x21, 21
    STORE x22, 22
    STORE x23, 23
    STORE x24, 24
    STORE x25, 25
    STORE x26, 26
    STORE x27, 27
    STORE x28, 28
    STORE x29, 29
    STORE x30, 30
    STORE x31, 31
    #将现在sscratch的值（发生中断时的sp），也就是用户栈/内核态地址写入sp+2*XLENB；
    #将sscratch的值重置为0（进入内核态后sscratch为0）
    csrrw s0, sscratch, x0
    STORE s0, 2
    csrr s1, sstatus
    STORE s1, 32
    csrr s2, stval
    STORE s2, 33
    csrr s3, scause
    STORE s3, 34
    csrr s4, sepc
    STORE s4, 35
.endm

.macro RESTORE_ALL
    #如果中断来自于内核(sstatus.SPP=1)，则返回到内核态
    #如果中断来自于用户(sstatus.SPP=0)，则返回到用户态，将sscratch的值设置为内核栈地址
    LOAD s1, 32         #s1 = sstatus
    csrw sstatus, s1    #恢复sstatus
    LOAD s2, 35         #s2 = sepc
    csrw sepc, s2       #恢复sepc
    addi s0, s1, 1<<8   #sstatus.SPP(没搞懂为什么要这么算SPP
    bnez s0, restore_context_kernel
restore_context_usr:
    #计算内核栈地址（因为之前压栈，开辟了36×4的空间），于是内核态地址就是sp+36×4
    addi s0, sp, 36*XLENB
    csrw sscratch, s0
restore_context_kernel:
    LOAD x1, 1
    LOAD x3, 3
    LOAD x4, 4
    LOAD x5, 5
    LOAD x6, 6
    LOAD x7, 7
    LOAD x8, 8
    LOAD x9, 9
    LOAD x10, 10
    LOAD x11, 11
    LOAD x12, 12
    LOAD x13, 13
    LOAD x14, 14
    LOAD x15, 15
    LOAD x16, 16
    LOAD x17, 17
    LOAD x18, 18
    LOAD x19, 19
    LOAD x20, 20
    LOAD x21, 21
    LOAD x22, 22
    LOAD x23, 23
    LOAD x24, 24
    LOAD x25, 25
    LOAD x26, 26
    LOAD x27, 27
    LOAD x28, 28
    LOAD x29, 29
    LOAD x30, 30
    LOAD x31, 31
    #x2是栈，最后将中断前栈值恢复
    LOAD x2, 2
.endm

.section .text
.global __changecontext
__changecontext:
    SAVE_ALL    #保存现场、通用寄存器
    mv a0, sp   #a0是参数寄存器，函数的参数已经在SAVEALL那一步存在栈里了
    jal handle_trap#跳转到handle_trap函数
.global __ret
__ret:
    RESTORE_ALL #恢复现场
    sret