	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_get_first_score                ; -- Begin function get_first_score
	.p2align	2
_get_first_score:                       ; @get_first_score
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 64
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #16]
	adrp	x0, l_str.0@PAGE
	add	x0, x0, l_str.0@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	_rt_get_item
	str	x0, [sp, #40]
	mov	x0, #0                          ; =0x0
	bl	_rt_imm
	str	x0, [sp, #8]
	ldr	x0, [sp, #40]
	ldr	x1, [sp, #8]
	bl	_rt_get_item
	str	x0, [sp, #32]
	ldr	x0, [sp, #32]
	ldp	x29, x30, [sp, #48]             ; 16-byte Folded Reload
	add	sp, sp, #64
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	___init__                       ; -- Begin function __init__
	.p2align	2
___init__:                              ; @__init__
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #400
	stp	x28, x27, [sp, #368]            ; 16-byte Folded Spill
	stp	x29, x30, [sp, #384]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 400
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	adrp	x0, l_str.1@PAGE
	add	x0, x0, l_str.1@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #80]
	ldr	x8, [sp, #80]
	str	x8, [sp]                        ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp]                        ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #192]
	bl	_rt_new_dict
	str	x0, [sp, #280]
	adrp	x0, l_str.2@PAGE
	add	x0, x0, l_str.2@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #312]
	adrp	x0, l_str.3@PAGE
	add	x0, x0, l_str.3@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #208]
	ldr	x0, [sp, #280]
	ldr	x1, [sp, #312]
	ldr	x2, [sp, #208]
	bl	_rt_set_item
	adrp	x0, l_str.4@PAGE
	add	x0, x0, l_str.4@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #128]
	mov	w8, #25                         ; =0x19
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #160]
	ldr	x0, [sp, #280]
	ldr	x1, [sp, #128]
	ldr	x2, [sp, #160]
	bl	_rt_set_item
	adrp	x0, l_str.5@PAGE
	add	x0, x0, l_str.5@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #104]
	mov	w8, #100                        ; =0x64
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #184]
	bl	_rt_new_arr
	str	x0, [sp, #344]
	ldr	x0, [sp, #344]
	ldr	x1, [sp, #184]
	bl	_rt_append_item
	mov	w8, #95                         ; =0x5f
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #120]
	ldr	x0, [sp, #344]
	ldr	x1, [sp, #120]
	bl	_rt_append_item
	mov	w8, #80                         ; =0x50
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #352]
	ldr	x0, [sp, #344]
	ldr	x1, [sp, #352]
	bl	_rt_append_item
	ldr	x0, [sp, #280]
	ldr	x1, [sp, #104]
	ldr	x2, [sp, #344]
	bl	_rt_set_item
	adrp	x0, l_str.6@PAGE
	add	x0, x0, l_str.6@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #152]
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #176]
	ldr	x0, [sp, #280]
	ldr	x1, [sp, #152]
	ldr	x2, [sp, #176]
	bl	_rt_set_item
	ldr	x8, [sp, #280]
	str	x8, [sp, #296]
	adrp	x0, l_str.7@PAGE
	add	x0, x0, l_str.7@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #360]
	ldr	x8, [sp, #360]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	adrp	x0, l_str.8@PAGE
	add	x0, x0, l_str.8@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #96]
	ldr	x0, [sp, #296]
	ldr	x1, [sp, #96]
	bl	_rt_get_item
	str	x0, [sp, #240]
	ldr	x8, [sp, #240]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #200]
	adrp	x0, l_str.9@PAGE
	add	x0, x0, l_str.9@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #336]
	ldr	x8, [sp, #336]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #296]
	bl	_get_first_score
	str	x0, [sp, #144]
	ldr	x8, [sp, #144]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #88]
	adrp	x0, l_str.10@PAGE
	add	x0, x0, l_str.10@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #168]
	ldr	x8, [sp, #168]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #272]
	adrp	x0, l_str.11@PAGE
	add	x0, x0, l_str.11@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #264]
	mov	w8, #2                          ; =0x2
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #304]
	mov	w8, #99                         ; =0x63
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #256]
	ldr	x0, [sp, #296]
	ldr	x1, [sp, #264]
	bl	_rt_get_item
	str	x0, [sp, #232]
	ldr	x0, [sp, #232]
	ldr	x1, [sp, #304]
	ldr	x2, [sp, #256]
	bl	_rt_set_item
	adrp	x0, l_str.12@PAGE
	add	x0, x0, l_str.12@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #288]
	adrp	x0, l_str.13@PAGE
	add	x0, x0, l_str.13@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #320]
	ldr	x0, [sp, #296]
	ldr	x1, [sp, #288]
	ldr	x2, [sp, #320]
	bl	_rt_set_item
	adrp	x0, l_str.14@PAGE
	add	x0, x0, l_str.14@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #224]
	ldr	x8, [sp, #224]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	adrp	x0, l_str.15@PAGE
	add	x0, x0, l_str.15@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #216]
	ldr	x0, [sp, #296]
	ldr	x1, [sp, #216]
	bl	_rt_get_item
	str	x0, [sp, #136]
	ldr	x8, [sp, #136]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #112]
	adrp	x0, l_str.16@PAGE
	add	x0, x0, l_str.16@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #328]
	ldr	x8, [sp, #328]
	str	x8, [sp, #64]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #296]
	str	x8, [sp, #72]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #72]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #248]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #384]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #368]            ; 16-byte Folded Reload
	add	sp, sp, #400
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
	.asciz	"scores"

	.p2align	4, 0x0                          ; @str.1
l_str.1:
	.asciz	"=== \345\273\272\347\253\213\350\244\207\351\233\234\347\232\204 JSON \350\263\207\346\226\231\347\265\220\346\247\213 ==="

l_str.2:                                ; @str.2
	.asciz	"name"

l_str.3:                                ; @str.3
	.asciz	"Alice"

l_str.4:                                ; @str.4
	.asciz	"age"

l_str.5:                                ; @str.5
	.asciz	"scores"

l_str.6:                                ; @str.6
	.asciz	"is_active"

	.p2align	4, 0x0                          ; @str.7
l_str.7:
	.asciz	"\344\275\277\347\224\250\350\200\205\345\247\223\345\220\215:"

l_str.8:                                ; @str.8
	.asciz	"name"

	.p2align	4, 0x0                          ; @str.9
l_str.9:
	.asciz	"\347\254\254\344\270\200\347\255\206\346\210\220\347\270\276\346\230\257:"

	.p2align	4, 0x0                          ; @str.10
l_str.10:
	.asciz	"=== \351\226\213\345\247\213\344\277\256\346\224\271\350\263\207\346\226\231 ==="

l_str.11:                               ; @str.11
	.asciz	"scores"

l_str.12:                               ; @str.12
	.asciz	"email"

	.p2align	4, 0x0                          ; @str.13
l_str.13:
	.asciz	"alice@example.com"

	.p2align	4, 0x0                          ; @str.14
l_str.14:
	.asciz	"\344\277\256\346\224\271\345\276\214\347\232\204\346\210\220\347\270\276\351\231\243\345\210\227:"

l_str.15:                               ; @str.15
	.asciz	"scores"

	.p2align	4, 0x0                          ; @str.16
l_str.16:
	.asciz	"\344\277\256\346\224\271\345\276\214\347\232\204\344\275\277\347\224\250\350\200\205\347\211\251\344\273\266:"

.subsections_via_symbols
