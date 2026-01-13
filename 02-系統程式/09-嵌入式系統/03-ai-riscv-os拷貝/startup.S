/*
 * RISC-V 64-bit 啟動程式碼
 */

.section .text.init
.global _start
.type _start, @function

_start:
    # 關閉所有中斷
    csrw mie, zero
    csrw mip, zero

    # 設定 global pointer (gp)
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    # 設定堆疊指標 (sp)
    la sp, __stack_top

    # 清空 BSS 段
    la t0, __bss_start
    la t1, __bss_end
bss_clear:
    bge t0, t1, bss_done
    sd zero, 0(t0)
    addi t0, t0, 8
    j bss_clear
bss_done:

    # 設定 trap vector (機器模式)
    la t0, trap_vector
    csrw mtvec, t0

    # 啟用 timer 中斷
    li t0, 0x80          # MIE.MTIE bit
    csrw mie, t0
    
    # 設定機器模式中斷使能
    li t0, 0x8           # MSTATUS.MIE bit
    csrw mstatus, t0

    # 初始化 timer (for QEMU virt)
    call init_timer

    # 跳到 C 的 main 函數
    call main

    # 如果 main 返回，進入無窮迴圈
halt:
    wfi
    j halt

# Trap 處理程式
.align 4
trap_vector:
    # 保存上下文 (64-bit)
    addi sp, sp, -256
    sd ra, 0(sp)
    sd t0, 8(sp)
    sd t1, 16(sp)
    sd t2, 24(sp)
    sd t3, 32(sp)
    sd t4, 40(sp)
    sd t5, 48(sp)
    sd t6, 56(sp)
    sd a0, 64(sp)
    sd a1, 72(sp)
    sd a2, 80(sp)
    sd a3, 88(sp)
    sd a4, 96(sp)
    sd a5, 104(sp)
    sd a6, 112(sp)
    sd a7, 120(sp)
    sd s0, 128(sp)
    sd s1, 136(sp)
    sd s2, 144(sp)
    sd s3, 152(sp)
    sd s4, 160(sp)
    sd s5, 168(sp)
    sd s6, 176(sp)
    sd s7, 184(sp)
    sd s8, 192(sp)
    sd s9, 200(sp)
    sd s10, 208(sp)
    sd s11, 216(sp)

    # 檢查是否為 timer 中斷
    csrr a0, mcause
    li t0, 0x8000000000000007    # Machine timer interrupt (64-bit)
    beq a0, t0, timer_handler

    # 其他中斷/異常
    j trap_exit

timer_handler:
    # 呼叫 C 的 timer 處理函數
    call timer_interrupt_handler
    
    # 重新設定 timer
    call reset_timer

trap_exit:
    # 恢復上下文 (64-bit)
    ld ra, 0(sp)
    ld t0, 8(sp)
    ld t1, 16(sp)
    ld t2, 24(sp)
    ld t3, 32(sp)
    ld t4, 40(sp)
    ld t5, 48(sp)
    ld t6, 56(sp)
    ld a0, 64(sp)
    ld a1, 72(sp)
    ld a2, 80(sp)
    ld a3, 88(sp)
    ld a4, 96(sp)
    ld a5, 104(sp)
    ld a6, 112(sp)
    ld a7, 120(sp)
    ld s0, 128(sp)
    ld s1, 136(sp)
    ld s2, 144(sp)
    ld s3, 152(sp)
    ld s4, 160(sp)
    ld s5, 168(sp)
    ld s6, 176(sp)
    ld s7, 184(sp)
    ld s8, 192(sp)
    ld s9, 200(sp)
    ld s10, 208(sp)
    ld s11, 216(sp)
    addi sp, sp, 256

    mret

# Timer 初始化 (QEMU virt - CLINT)
.global init_timer
init_timer:
    # CLINT 基址: 0x2000000
    # mtime: 0x200bff8
    # mtimecmp: 0x2004000
    
    li t0, 0x2004000     # mtimecmp 位址
    li t1, 0x200bff8     # mtime 位址
    ld t2, 0(t1)         # 讀取當前時間 (64-bit)
    li t3, 1000000       # 設定 1ms 間隔 (假設 1MHz)
    add t2, t2, t3
    sd t2, 0(t0)         # 設定比較值 (64-bit)
    ret

# 重設 Timer
.global reset_timer
reset_timer:
    li t0, 0x2004000
    li t1, 0x200bff8
    ld t2, 0(t1)
    li t3, 1000000
    add t2, t2, t3
    sd t2, 0(t0)
    ret