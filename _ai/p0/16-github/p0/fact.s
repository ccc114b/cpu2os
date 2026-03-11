	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_fact                           ; -- Begin function fact
	.p2align	2
_fact:                                  ; @fact
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #80
	stp	x29, x30, [sp, #64]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 80
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #32]
	mov	w8, #2                          ; =0x2
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #48]
	ldr	x0, [sp, #32]
	ldr	x1, [sp, #48]
	bl	_rt_cmp_lt
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	bl	_rt_is_truthy
	tbz	w0, #0, LBB0_2
	b	LBB0_1
LBB0_1:                                 ; %fallthrough.1
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #56]
	ldr	x0, [sp, #56]
	ldp	x29, x30, [sp, #64]             ; 16-byte Folded Reload
	add	sp, sp, #80
	ret
LBB0_2:                                 ; %L1
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp]
	ldr	x0, [sp, #32]
	ldr	x1, [sp]
	bl	_rt_sub
	str	x0, [sp, #40]
	ldr	x0, [sp, #40]
	bl	_fact
	str	x0, [sp, #24]
	ldr	x0, [sp, #32]
	ldr	x1, [sp, #24]
	bl	_rt_mul
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	ldp	x29, x30, [sp, #64]             ; 16-byte Folded Reload
	add	sp, sp, #80
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	___init__                       ; -- Begin function __init__
	.p2align	2
___init__:                              ; @__init__
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #144
	stp	x29, x30, [sp, #128]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 144
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x0, l_str.0@PAGE
	add	x0, x0, l_str.0@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #88]
	ldr	x8, [sp, #88]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	mov	w8, #5                          ; =0x5
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #64]
	ldr	x0, [sp, #64]
	bl	_fact
	str	x0, [sp, #72]
	ldr	x8, [sp, #72]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #120]
	adrp	x0, l_str.1@PAGE
	add	x0, x0, l_str.1@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #104]
	ldr	x8, [sp, #104]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	mov	w8, #10                         ; =0xa
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #80]
	mov	w8, #20                         ; =0x14
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #96]
	ldr	x0, [sp, #80]
	ldr	x1, [sp, #96]
	bl	_rt_add
	str	x0, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	adrp	x0, l_str.2@PAGE
	add	x0, x0, l_str.2@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #48]
	ldr	x8, [sp, #48]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #112]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #128]            ; 16-byte Folded Reload
	add	sp, sp, #144
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	___init__
	mov	w0, #0                          ; =0x0
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__cstring,cstring_literals
l_str.0:                                ; @str.0
	.asciz	"fact(5) ="

l_str.1:                                ; @str.1
	.asciz	"10 + 20 ="

l_str.2:                                ; @str.2
	.asciz	", \345\276\210\347\260\241\345\226\256\357\274\201"

.subsections_via_symbols
