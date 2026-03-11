	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_factorial                      ; -- Begin function factorial
	.p2align	2
_factorial:                             ; @factorial
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [sp, #8]
	subs	x8, x8, #0
	cset	w8, eq
	tbnz	w8, #0, LBB0_2
	b	LBB0_1
LBB0_1:                                 ; %L1
	ldr	x8, [sp, #8]
	str	x8, [sp]                        ; 8-byte Folded Spill
	ldr	x8, [sp, #8]
	subs	x0, x8, #1
	bl	_factorial
	ldr	x8, [sp]                        ; 8-byte Folded Reload
	mul	x0, x8, x0
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
LBB0_2:                                 ; %L2
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_greet                          ; -- Begin function greet
	.p2align	2
_greet:                                 ; @greet
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 48
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #24]
	adrp	x8, l_.str.1@PAGE
	add	x8, x8, l_.str.1@PAGEOFF
	str	x8, [sp, #16]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	_concat
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	bl	_print
	ldr	x0, [sp, #8]
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #5                          ; =0x5
	mov	x0, x8
	bl	_factorial
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	bl	_to_str
	mov	x1, x0
	adrp	x0, l_.str.2@PAGE
	add	x0, x0, l_.str.2@PAGEOFF
	bl	_concat
	str	x0, [sp]
	ldr	x0, [sp]
	bl	_print
	ldr	x8, [sp, #8]
	subs	x8, x8, #0
	cset	w8, eq
	tbnz	w8, #0, LBB2_2
	b	LBB2_1
LBB2_1:                                 ; %L1
	adrp	x0, l_.str.3@PAGE
	add	x0, x0, l_.str.3@PAGEOFF
	bl	_greet
	b	LBB2_3
LBB2_2:                                 ; %L2
	b	LBB2_3
LBB2_3:                                 ; %L3
	ldr	x0, [sp, #8]
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__cstring,cstring_literals
l_.str.1:                               ; @.str.1
	.asciz	"Hello, "

l_.str.2:                               ; @.str.2
	.asciz	"result="

l_.str.3:                               ; @.str.3
	.asciz	"Master!"

.subsections_via_symbols
