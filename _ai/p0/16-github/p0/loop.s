	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	___init__                       ; -- Begin function __init__
	.p2align	2
___init__:                              ; @__init__
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #384
	stp	x28, x27, [sp, #352]            ; 16-byte Folded Spill
	stp	x29, x30, [sp, #368]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 384
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	adrp	x0, l_str.0@PAGE
	add	x0, x0, l_str.0@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #112]
	ldr	x8, [sp, #112]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #312]
	mov	x0, #0                          ; =0x0
	str	x0, [sp, #64]                   ; 8-byte Folded Spill
	bl	_rt_imm
	str	x0, [sp, #344]
	bl	_rt_new_arr
	str	x0, [sp, #200]
	ldr	x0, [sp, #200]
	ldr	x1, [sp, #344]
	bl	_rt_append_item
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_imm
	str	x0, [sp, #336]
	ldr	x0, [sp, #200]
	ldr	x1, [sp, #336]
	bl	_rt_append_item
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_imm
	str	x0, [sp, #136]
	ldr	x0, [sp, #200]
	ldr	x1, [sp, #136]
	bl	_rt_append_item
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_imm
	str	x0, [sp, #280]
	ldr	x0, [sp, #200]
	ldr	x1, [sp, #280]
	bl	_rt_append_item
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_imm
	str	x0, [sp, #192]
	ldr	x0, [sp, #200]
	ldr	x1, [sp, #192]
	bl	_rt_append_item
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	ldr	x8, [sp, #200]
	str	x8, [sp, #256]
	bl	_rt_imm
	str	x0, [sp, #120]
	ldr	x8, [sp, #120]
	str	x8, [sp, #248]
	b	LBB0_1
LBB0_1:                                 ; %L1
                                        ; =>This Inner Loop Header: Depth=1
	mov	w8, #5                          ; =0x5
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #104]
	ldr	x0, [sp, #248]
	ldr	x1, [sp, #104]
	bl	_rt_cmp_lt
	str	x0, [sp, #96]
	ldr	x0, [sp, #96]
	bl	_rt_is_truthy
	tbz	w0, #0, LBB0_7
	b	LBB0_2
LBB0_2:                                 ; %fallthrough.1
                                        ;   in Loop: Header=BB0_1 Depth=1
	b	LBB0_4
LBB0_3:                                 ; %L3
                                        ;   in Loop: Header=BB0_1 Depth=1
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #160]
	ldr	x0, [sp, #248]
	ldr	x1, [sp, #160]
	bl	_rt_add
	str	x0, [sp, #144]
	ldr	x8, [sp, #144]
	str	x8, [sp, #248]
	b	LBB0_1
LBB0_4:                                 ; %L2
                                        ;   in Loop: Header=BB0_1 Depth=1
	mov	w8, #2                          ; =0x2
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #208]
	ldr	x0, [sp, #248]
	ldr	x1, [sp, #208]
	bl	_rt_cmp_eq
	str	x0, [sp, #240]
	ldr	x0, [sp, #240]
	bl	_rt_is_truthy
	tbz	w0, #0, LBB0_6
	b	LBB0_5
LBB0_5:                                 ; %fallthrough.2
                                        ;   in Loop: Header=BB0_1 Depth=1
	b	LBB0_3
LBB0_6:                                 ; %L5
                                        ;   in Loop: Header=BB0_1 Depth=1
	mov	w8, #10                         ; =0xa
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #328]
	ldr	x0, [sp, #248]
	ldr	x1, [sp, #328]
	bl	_rt_mul
	str	x0, [sp, #320]
	ldr	x0, [sp, #256]
	ldr	x1, [sp, #248]
	ldr	x2, [sp, #320]
	bl	_rt_set_item
	b	LBB0_3
LBB0_7:                                 ; %L4
	adrp	x0, l_str.1@PAGE
	add	x0, x0, l_str.1@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #304]
	ldr	x8, [sp, #304]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #256]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #272]
	adrp	x0, l_str.2@PAGE
	add	x0, x0, l_str.2@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #232]
	ldr	x8, [sp, #232]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #296]
	mov	x0, #0                          ; =0x0
	bl	_rt_imm
	str	x0, [sp, #264]
	ldr	x8, [sp, #264]
	str	x8, [sp, #152]
	b	LBB0_8
LBB0_8:                                 ; %L6
                                        ; =>This Inner Loop Header: Depth=1
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #128]
	ldr	x0, [sp, #128]
	bl	_rt_is_truthy
	tbz	w0, #0, LBB0_12
	b	LBB0_9
LBB0_9:                                 ; %fallthrough.3
                                        ;   in Loop: Header=BB0_8 Depth=1
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #168]
	ldr	x0, [sp, #152]
	ldr	x1, [sp, #168]
	bl	_rt_add
	str	x0, [sp, #224]
	ldr	x8, [sp, #224]
	str	x8, [sp, #152]
	mov	w8, #3                          ; =0x3
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #176]
	ldr	x0, [sp, #152]
	ldr	x1, [sp, #176]
	bl	_rt_cmp_gt
	str	x0, [sp, #88]
	ldr	x0, [sp, #88]
	bl	_rt_is_truthy
	tbz	w0, #0, LBB0_11
	b	LBB0_10
LBB0_10:                                ; %fallthrough.4
	b	LBB0_12
LBB0_11:                                ; %L8
                                        ;   in Loop: Header=BB0_8 Depth=1
	adrp	x0, l_str.3@PAGE
	add	x0, x0, l_str.3@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #216]
	ldr	x8, [sp, #216]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	ldr	x8, [sp, #152]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	adrp	x0, l_str.4@PAGE
	add	x0, x0, l_str.4@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #288]
	ldr	x8, [sp, #288]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #184]
	b	LBB0_8
LBB0_12:                                ; %L7
	adrp	x0, l_str.5@PAGE
	add	x0, x0, l_str.5@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #80]
	ldr	x8, [sp, #80]
	str	x8, [sp]                        ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp]                        ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #72]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #368]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #352]            ; 16-byte Folded Reload
	add	sp, sp, #384
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
	.p2align	4, 0x0                          ; @str.0
l_str.0:
	.asciz	"=== 1. \346\270\254\350\251\246 For \350\277\264\345\234\210\350\210\207 Continue ==="

	.p2align	4, 0x0                          ; @str.1
l_str.1:
	.asciz	"\351\201\216\346\277\276\345\276\214\347\232\204\351\231\243\345\210\227:"

	.p2align	4, 0x0                          ; @str.2
l_str.2:
	.asciz	"=== 2. \346\270\254\350\251\246 While \350\277\264\345\234\210\350\210\207 Break ==="

l_str.3:                                ; @str.3
	.asciz	"While \345\237\267\350\241\214\347\254\254"

l_str.4:                                ; @str.4
	.asciz	"\346\254\241"

	.p2align	4, 0x0                          ; @str.5
l_str.5:
	.asciz	"\350\267\263\345\207\272\350\277\264\345\234\210\344\272\206\357\274\201"

.subsections_via_symbols
