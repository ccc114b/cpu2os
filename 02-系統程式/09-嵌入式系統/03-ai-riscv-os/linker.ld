/*
 * RISC-V Linker Script
 * 適用於 QEMU virt machine
 */

OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    . = 0x80000000;

    /* 程式碼段 */
    .text : {
        *(.text.init)
        *(.text*)
        . = ALIGN(4);
    } > RAM

    /* 唯讀資料段 */
    .rodata : {
        *(.rodata*)
        . = ALIGN(4);
    } > RAM

    /* Small data 段 */
    .sdata : {
        __SDATA_BEGIN__ = .;
        *(.sdata .sdata.* .gnu.linkonce.s.*)
        . = ALIGN(8);
    } > RAM

    /* 資料段 */
    .data : {
        *(.data*)
        . = ALIGN(8);
    } > RAM
    
    /* Global pointer：指向 .sdata 的中間位置 */
    __global_pointer$ = __SDATA_BEGIN__ + 0x800;

    /* Small BSS 段 */
    .sbss : {
        *(.sbss .sbss.* .gnu.linkonce.sb.*)
        *(.scommon)
        . = ALIGN(8);
    } > RAM

    /* BSS 段 */
    .bss : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    } > RAM

    /* 堆疊 */
    . = ALIGN(16);
    . += 0x10000;  /* 64KB 堆疊 */
    __stack_top = .;

    /* 堆積 */
    . = ALIGN(16);
    __heap_start = .;
    . += 0x100000;  /* 1MB 堆積 */
    __heap_end = .;
}