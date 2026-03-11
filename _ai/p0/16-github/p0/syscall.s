	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_test_array                     ; -- Begin function test_array
	.p2align	2
_test_array:                            ; @test_array
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
	adrp	x0, l_str.0@PAGE
	add	x0, x0, l_str.0@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #256]
	ldr	x8, [sp, #256]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #200]
	mov	w8, #3                          ; =0x3
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #264]
	ldr	x8, [sp, #264]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	mov	x0, #0                          ; =0x0
	bl	_rt_imm
	mov	x8, x0
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	str	x8, [sp, #248]
	ldr	x1, [sp, #248]
	bl	_p0_array
	str	x0, [sp, #144]
	ldr	x8, [sp, #144]
	str	x8, [sp, #240]
	adrp	x0, l_str.1@PAGE
	add	x0, x0, l_str.1@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #168]
	ldr	x8, [sp, #168]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #240]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #184]
	adrp	x0, l_str.2@PAGE
	add	x0, x0, l_str.2@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #280]
	ldr	x8, [sp, #280]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #240]
	bl	_p0_len
	str	x0, [sp, #152]
	ldr	x8, [sp, #152]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #320]
	ldr	x8, [sp, #240]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	mov	w8, #99                         ; =0x63
	mov	x0, x8
	bl	_rt_imm
	mov	x8, x0
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	str	x8, [sp, #352]
	ldr	x1, [sp, #352]
	bl	_p0_push
	str	x0, [sp, #360]
	ldr	x8, [sp, #240]
	str	x8, [sp, #64]                   ; 8-byte Folded Spill
	mov	w8, #100                        ; =0x64
	mov	x0, x8
	bl	_rt_imm
	mov	x8, x0
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	str	x8, [sp, #160]
	ldr	x1, [sp, #160]
	bl	_p0_push
	str	x0, [sp, #312]
	adrp	x0, l_str.3@PAGE
	add	x0, x0, l_str.3@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #304]
	ldr	x8, [sp, #304]
	str	x8, [sp, #72]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #240]
	str	x8, [sp, #80]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #72]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #80]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #288]
	adrp	x0, l_str.4@PAGE
	add	x0, x0, l_str.4@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #344]
	ldr	x8, [sp, #344]
	str	x8, [sp, #88]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #240]
	bl	_p0_len
	str	x0, [sp, #232]
	ldr	x8, [sp, #232]
	str	x8, [sp, #96]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #88]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #96]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #336]
	ldr	x0, [sp, #240]
	bl	_p0_pop
	str	x0, [sp, #296]
	ldr	x8, [sp, #296]
	str	x8, [sp, #224]
	adrp	x0, l_str.5@PAGE
	add	x0, x0, l_str.5@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #272]
	ldr	x8, [sp, #272]
	str	x8, [sp, #104]                  ; 8-byte Folded Spill
	ldr	x8, [sp, #224]
	str	x8, [sp, #112]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #104]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #112]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #328]
	adrp	x0, l_str.6@PAGE
	add	x0, x0, l_str.6@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #216]
	ldr	x8, [sp, #216]
	str	x8, [sp, #120]                  ; 8-byte Folded Spill
	ldr	x8, [sp, #240]
	str	x8, [sp, #128]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #120]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #128]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #208]
	adrp	x0, l_str.7@PAGE
	add	x0, x0, l_str.7@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #192]
	ldr	x8, [sp, #192]
	str	x8, [sp, #136]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #136]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #176]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #384]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #368]            ; 16-byte Folded Reload
	add	sp, sp, #400
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_test_dict                      ; -- Begin function test_dict
	.p2align	2
_test_dict:                             ; @test_dict
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #448
	stp	x28, x27, [sp, #416]            ; 16-byte Folded Spill
	stp	x29, x30, [sp, #432]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 448
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	adrp	x0, l_str.8@PAGE
	add	x0, x0, l_str.8@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #136]
	ldr	x8, [sp, #136]
	str	x8, [sp]                        ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp]                        ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #280]
	bl	_rt_new_dict
	str	x0, [sp, #376]
	adrp	x0, l_str.9@PAGE
	add	x0, x0, l_str.9@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #352]
	adrp	x0, l_str.10@PAGE
	add	x0, x0, l_str.10@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #296]
	ldr	x0, [sp, #376]
	ldr	x1, [sp, #352]
	ldr	x2, [sp, #296]
	bl	_rt_set_item
	adrp	x0, l_str.11@PAGE
	add	x0, x0, l_str.11@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #200]
	mov	w8, #25                         ; =0x19
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #288]
	ldr	x0, [sp, #376]
	ldr	x1, [sp, #200]
	ldr	x2, [sp, #288]
	bl	_rt_set_item
	adrp	x0, l_str.12@PAGE
	add	x0, x0, l_str.12@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #192]
	adrp	x0, l_str.13@PAGE
	add	x0, x0, l_str.13@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #384]
	ldr	x0, [sp, #376]
	ldr	x1, [sp, #192]
	ldr	x2, [sp, #384]
	bl	_rt_set_item
	ldr	x8, [sp, #376]
	str	x8, [sp, #264]
	adrp	x0, l_str.14@PAGE
	add	x0, x0, l_str.14@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #144]
	ldr	x8, [sp, #144]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	ldr	x8, [sp, #264]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #152]
	adrp	x0, l_str.15@PAGE
	add	x0, x0, l_str.15@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #256]
	ldr	x8, [sp, #256]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #264]
	bl	_p0_len
	str	x0, [sp, #408]
	ldr	x8, [sp, #408]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #248]
	ldr	x0, [sp, #264]
	bl	_p0_keys
	str	x0, [sp, #392]
	ldr	x8, [sp, #392]
	str	x8, [sp, #224]
	adrp	x0, l_str.16@PAGE
	add	x0, x0, l_str.16@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #368]
	ldr	x8, [sp, #368]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #224]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #304]
	ldr	x8, [sp, #264]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	adrp	x0, l_str.17@PAGE
	add	x0, x0, l_str.17@PAGEOFF
	bl	_rt_load_str
	mov	x8, x0
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	str	x8, [sp, #360]
	ldr	x1, [sp, #360]
	bl	_p0_has_key
	str	x0, [sp, #208]
	ldr	x8, [sp, #208]
	str	x8, [sp, #216]
	ldr	x8, [sp, #264]
	str	x8, [sp, #64]                   ; 8-byte Folded Spill
	adrp	x0, l_str.18@PAGE
	add	x0, x0, l_str.18@PAGEOFF
	bl	_rt_load_str
	mov	x8, x0
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	str	x8, [sp, #320]
	ldr	x1, [sp, #320]
	bl	_p0_has_key
	str	x0, [sp, #400]
	ldr	x8, [sp, #400]
	str	x8, [sp, #344]
	adrp	x0, l_str.19@PAGE
	add	x0, x0, l_str.19@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #336]
	ldr	x8, [sp, #336]
	str	x8, [sp, #72]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #216]
	str	x8, [sp, #80]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #72]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #80]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #232]
	adrp	x0, l_str.20@PAGE
	add	x0, x0, l_str.20@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #328]
	ldr	x8, [sp, #328]
	str	x8, [sp, #88]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #344]
	str	x8, [sp, #96]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #88]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #96]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #184]
	ldr	x8, [sp, #264]
	str	x8, [sp, #104]                  ; 8-byte Folded Spill
	adrp	x0, l_str.21@PAGE
	add	x0, x0, l_str.21@PAGEOFF
	bl	_rt_load_str
	mov	x8, x0
	ldr	x0, [sp, #104]                  ; 8-byte Folded Reload
	str	x8, [sp, #168]
	ldr	x1, [sp, #168]
	bl	_p0_remove
	str	x0, [sp, #176]
	adrp	x0, l_str.22@PAGE
	add	x0, x0, l_str.22@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #312]
	ldr	x8, [sp, #312]
	str	x8, [sp, #112]                  ; 8-byte Folded Spill
	ldr	x8, [sp, #264]
	str	x8, [sp, #120]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #112]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #120]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #240]
	adrp	x0, l_str.23@PAGE
	add	x0, x0, l_str.23@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #272]
	ldr	x8, [sp, #272]
	str	x8, [sp, #128]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #128]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #160]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #432]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #416]            ; 16-byte Folded Reload
	add	sp, sp, #448
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_test_type_and_cast             ; -- Begin function test_type_and_cast
	.p2align	2
_test_type_and_cast:                    ; @test_type_and_cast
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #480
	stp	x28, x27, [sp, #448]            ; 16-byte Folded Spill
	stp	x29, x30, [sp, #464]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 480
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	adrp	x0, l_str.24@PAGE
	add	x0, x0, l_str.24@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #136]
	ldr	x8, [sp, #136]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #272]
	mov	w8, #42                         ; =0x2a
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #392]
	ldr	x8, [sp, #392]
	str	x8, [sp, #240]
	adrp	x0, l_str.25@PAGE
	add	x0, x0, l_str.25@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #296]
	ldr	x8, [sp, #296]
	str	x8, [sp, #416]
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	str	x0, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_imm
	str	x0, [sp, #288]
	bl	_rt_new_arr
	str	x0, [sp, #176]
	ldr	x0, [sp, #176]
	ldr	x1, [sp, #288]
	bl	_rt_append_item
	mov	w8, #2                          ; =0x2
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #408]
	ldr	x0, [sp, #176]
	ldr	x1, [sp, #408]
	bl	_rt_append_item
	ldr	x8, [sp, #176]
	str	x8, [sp, #344]
	bl	_rt_new_dict
	str	x0, [sp, #368]
	adrp	x0, l_str.26@PAGE
	add	x0, x0, l_str.26@PAGEOFF
	bl	_rt_load_str
	mov	x8, x0
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	str	x8, [sp, #232]
	bl	_rt_imm
	str	x0, [sp, #376]
	ldr	x0, [sp, #368]
	ldr	x1, [sp, #232]
	ldr	x2, [sp, #376]
	bl	_rt_set_item
	ldr	x8, [sp, #368]
	str	x8, [sp, #184]
	adrp	x0, l_str.27@PAGE
	add	x0, x0, l_str.27@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #168]
	ldr	x8, [sp, #168]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #240]
	bl	_p0_typeof
	str	x0, [sp, #384]
	ldr	x8, [sp, #384]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #432]
	adrp	x0, l_str.28@PAGE
	add	x0, x0, l_str.28@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #352]
	ldr	x8, [sp, #352]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #416]
	bl	_p0_typeof
	str	x0, [sp, #264]
	ldr	x8, [sp, #264]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #224]
	adrp	x0, l_str.29@PAGE
	add	x0, x0, l_str.29@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #304]
	ldr	x8, [sp, #304]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #344]
	bl	_p0_typeof
	str	x0, [sp, #400]
	ldr	x8, [sp, #400]
	str	x8, [sp, #64]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #64]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #424]
	adrp	x0, l_str.30@PAGE
	add	x0, x0, l_str.30@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #144]
	ldr	x8, [sp, #144]
	str	x8, [sp, #72]                   ; 8-byte Folded Spill
	ldr	x0, [sp, #184]
	bl	_p0_typeof
	str	x0, [sp, #160]
	ldr	x8, [sp, #160]
	str	x8, [sp, #80]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #72]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #80]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #216]
	ldr	x0, [sp, #416]
	bl	_p0_int
	str	x0, [sp, #440]
	mov	w8, #50                         ; =0x32
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #152]
	ldr	x0, [sp, #440]
	ldr	x1, [sp, #152]
	bl	_rt_add
	str	x0, [sp, #312]
	ldr	x8, [sp, #312]
	str	x8, [sp, #280]
	adrp	x0, l_str.31@PAGE
	add	x0, x0, l_str.31@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #208]
	ldr	x8, [sp, #208]
	str	x8, [sp, #88]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #280]
	str	x8, [sp, #96]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #88]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #96]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #256]
	ldr	x0, [sp, #240]
	bl	_p0_str
	str	x0, [sp, #336]
	adrp	x0, l_str.32@PAGE
	add	x0, x0, l_str.32@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #248]
	ldr	x0, [sp, #336]
	ldr	x1, [sp, #248]
	bl	_rt_add
	str	x0, [sp, #128]
	ldr	x8, [sp, #128]
	str	x8, [sp, #200]
	adrp	x0, l_str.33@PAGE
	add	x0, x0, l_str.33@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #328]
	ldr	x8, [sp, #328]
	str	x8, [sp, #104]                  ; 8-byte Folded Spill
	ldr	x8, [sp, #200]
	str	x8, [sp, #112]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #104]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #112]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #320]
	adrp	x0, l_str.34@PAGE
	add	x0, x0, l_str.34@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #192]
	ldr	x8, [sp, #192]
	str	x8, [sp, #120]                  ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #120]                  ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #360]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #464]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #448]            ; 16-byte Folded Reload
	add	sp, sp, #480
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_test_char_conversion           ; -- Begin function test_char_conversion
	.p2align	2
_test_char_conversion:                  ; @test_char_conversion
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #192
	stp	x29, x30, [sp, #176]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 192
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x0, l_str.35@PAGE
	add	x0, x0, l_str.35@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #80]
	ldr	x8, [sp, #80]
	str	x8, [sp]                        ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp]                        ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #72]
	adrp	x0, l_str.36@PAGE
	add	x0, x0, l_str.36@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #152]
	ldr	x8, [sp, #152]
	str	x8, [sp, #104]
	ldr	x0, [sp, #104]
	bl	_p0_ord
	str	x0, [sp, #160]
	ldr	x8, [sp, #160]
	str	x8, [sp, #128]
	adrp	x0, l_str.37@PAGE
	add	x0, x0, l_str.37@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #88]
	ldr	x8, [sp, #88]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	ldr	x8, [sp, #128]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #120]
	mov	w8, #1                          ; =0x1
	mov	x0, x8
	bl	_rt_imm
	str	x0, [sp, #96]
	ldr	x0, [sp, #128]
	ldr	x1, [sp, #96]
	bl	_rt_add
	str	x0, [sp, #144]
	ldr	x0, [sp, #144]
	bl	_p0_chr
	str	x0, [sp, #64]
	ldr	x8, [sp, #64]
	str	x8, [sp, #48]
	adrp	x0, l_str.38@PAGE
	add	x0, x0, l_str.38@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #136]
	ldr	x8, [sp, #136]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #48]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #56]
	adrp	x0, l_str.39@PAGE
	add	x0, x0, l_str.39@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #168]
	ldr	x8, [sp, #168]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #112]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #176]            ; 16-byte Folded Reload
	add	sp, sp, #192
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_test_system                    ; -- Begin function test_system
	.p2align	2
_test_system:                           ; @test_system
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #160
	stp	x29, x30, [sp, #144]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 160
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x0, l_str.40@PAGE
	add	x0, x0, l_str.40@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #88]
	ldr	x8, [sp, #88]
	str	x8, [sp]                        ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp]                        ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #80]
	bl	_p0_time
	str	x0, [sp, #48]
	ldr	x8, [sp, #48]
	str	x8, [sp, #120]
	adrp	x0, l_str.41@PAGE
	add	x0, x0, l_str.41@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #112]
	ldr	x8, [sp, #112]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	ldr	x8, [sp, #120]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #72]
	bl	_p0_random
	str	x0, [sp, #136]
	ldr	x8, [sp, #136]
	str	x8, [sp, #96]
	adrp	x0, l_str.42@PAGE
	add	x0, x0, l_str.42@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #128]
	ldr	x8, [sp, #128]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #96]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #104]
	adrp	x0, l_str.43@PAGE
	add	x0, x0, l_str.43@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #64]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #144]            ; 16-byte Folded Reload
	add	sp, sp, #160
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	___p0_main                      ; -- Begin function __p0_main
	.p2align	2
___p0_main:                             ; @__p0_main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #272
	stp	x28, x27, [sp, #240]            ; 16-byte Folded Spill
	stp	x29, x30, [sp, #256]            ; 16-byte Folded Spill
	.cfi_def_cfa_offset 272
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	adrp	x0, l_str.44@PAGE
	add	x0, x0, l_str.44@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #152]
	ldr	x8, [sp, #152]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #208]
	adrp	x0, l_str.45@PAGE
	add	x0, x0, l_str.45@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #96]
	ldr	x8, [sp, #96]
	str	x8, [sp, #16]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #16]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #168]
	bl	_test_array
	str	x0, [sp, #176]
	bl	_test_dict
	str	x0, [sp, #104]
	bl	_test_type_and_cast
	str	x0, [sp, #128]
	bl	_test_char_conversion
	str	x0, [sp, #64]
	bl	_test_system
	str	x0, [sp, #184]
	adrp	x0, l_str.46@PAGE
	add	x0, x0, l_str.46@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #192]
	ldr	x8, [sp, #192]
	str	x8, [sp, #24]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #24]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #144]
	adrp	x0, l_str.47@PAGE
	add	x0, x0, l_str.47@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #160]
	ldr	x0, [sp, #160]
	bl	_p0_input
	str	x0, [sp, #224]
	ldr	x8, [sp, #224]
	str	x8, [sp, #216]
	adrp	x0, l_str.48@PAGE
	add	x0, x0, l_str.48@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #88]
	ldr	x8, [sp, #88]
	str	x8, [sp, #32]                   ; 8-byte Folded Spill
	ldr	x8, [sp, #216]
	str	x8, [sp, #40]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #32]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	ldr	x0, [sp, #40]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #72]
	adrp	x0, l_str.49@PAGE
	add	x0, x0, l_str.49@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #200]
	ldr	x8, [sp, #200]
	str	x8, [sp, #48]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #48]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #136]
	mov	x0, #0                          ; =0x0
	bl	_rt_imm
	str	x0, [sp, #232]
	ldr	x0, [sp, #232]
	bl	_p0_exit
	str	x0, [sp, #120]
	adrp	x0, l_str.50@PAGE
	add	x0, x0, l_str.50@PAGEOFF
	bl	_rt_load_str
	str	x0, [sp, #80]
	ldr	x8, [sp, #80]
	str	x8, [sp, #56]                   ; 8-byte Folded Spill
	bl	_rt_print_begin
	ldr	x0, [sp, #56]                   ; 8-byte Folded Reload
	bl	_rt_print_arg
	bl	_rt_print_end
	str	x0, [sp, #112]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #256]            ; 16-byte Folded Reload
	ldp	x28, x27, [sp, #240]            ; 16-byte Folded Reload
	add	sp, sp, #272
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	___init__                       ; -- Begin function __init__
	.p2align	2
___init__:                              ; @__init__
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	___p0_main
	str	x0, [sp, #8]
	mov	x0, #0                          ; =0x0
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
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
	.asciz	"--- 1. \346\270\254\350\251\246\351\231\243\345\210\227\347\233\270\351\227\234 (array, len, push, pop) ---"

	.p2align	4, 0x0                          ; @str.1
l_str.1:
	.asciz	"\345\210\235\345\247\213\345\214\226 array(3, 0):"

	.p2align	4, 0x0                          ; @str.2
l_str.2:
	.asciz	"\351\231\243\345\210\227\351\225\267\345\272\246 len(arr):"

l_str.3:                                ; @str.3
	.asciz	"push \345\205\251\346\254\241\345\276\214:"

	.p2align	4, 0x0                          ; @str.4
l_str.4:
	.asciz	"\346\226\260\351\231\243\345\210\227\351\225\267\345\272\246:"

	.p2align	4, 0x0                          ; @str.5
l_str.5:
	.asciz	"pop() \345\275\210\345\207\272\347\232\204\345\200\274:"

	.p2align	4, 0x0                          ; @str.6
l_str.6:
	.asciz	"pop \345\276\214\347\232\204\351\231\243\345\210\227:"

l_str.7:                                ; @str.7
	.space	1

	.p2align	4, 0x0                          ; @str.8
l_str.8:
	.asciz	"--- 2. \346\270\254\350\251\246\345\255\227\345\205\270\347\233\270\351\227\234 (keys, has_key, remove) ---"

l_str.9:                                ; @str.9
	.asciz	"name"

l_str.10:                               ; @str.10
	.asciz	"Alice"

l_str.11:                               ; @str.11
	.asciz	"age"

l_str.12:                               ; @str.12
	.asciz	"city"

l_str.13:                               ; @str.13
	.asciz	"Taipei"

l_str.14:                               ; @str.14
	.asciz	"\345\210\235\345\247\213\345\255\227\345\205\270:"

	.p2align	4, 0x0                          ; @str.15
l_str.15:
	.asciz	"\345\255\227\345\205\270\351\225\267\345\272\246 len(d):"

	.p2align	4, 0x0                          ; @str.16
l_str.16:
	.asciz	"\346\211\200\346\234\211\347\232\204\351\215\265 keys(d):"

l_str.17:                               ; @str.17
	.asciz	"name"

l_str.18:                               ; @str.18
	.asciz	"job"

	.p2align	4, 0x0                          ; @str.19
l_str.19:
	.asciz	"\346\230\257\345\220\246\345\214\205\345\220\253 'name'?"

	.p2align	4, 0x0                          ; @str.20
l_str.20:
	.asciz	"\346\230\257\345\220\246\345\214\205\345\220\253 'job'?"

l_str.21:                               ; @str.21
	.asciz	"age"

	.p2align	4, 0x0                          ; @str.22
l_str.22:
	.asciz	"remove(d, 'age') \344\271\213\345\276\214\347\232\204\345\255\227\345\205\270:"

l_str.23:                               ; @str.23
	.space	1

	.p2align	4, 0x0                          ; @str.24
l_str.24:
	.asciz	"--- 3. \346\270\254\350\251\246\345\236\213\345\210\245\350\210\207\350\275\211\346\217\233 (typeof, int, str) ---"

l_str.25:                               ; @str.25
	.asciz	"100"

l_str.26:                               ; @str.26
	.asciz	"k"

l_str.27:                               ; @str.27
	.asciz	"typeof(42)  :"

l_str.28:                               ; @str.28
	.asciz	"typeof('100'):"

l_str.29:                               ; @str.29
	.asciz	"typeof([1]) :"

l_str.30:                               ; @str.30
	.asciz	"typeof({k}) :"

	.p2align	4, 0x0                          ; @str.31
l_str.31:
	.asciz	"int('100') + 50 ="

	.p2align	4, 0x0                          ; @str.32
l_str.32:
	.asciz	" \346\230\257\344\270\200\345\200\213\345\255\227\344\270\262"

	.p2align	4, 0x0                          ; @str.33
l_str.33:
	.asciz	"str(42) \344\270\262\346\216\245\347\265\220\346\236\234:"

l_str.34:                               ; @str.34
	.space	1

	.p2align	4, 0x0                          ; @str.35
l_str.35:
	.asciz	"--- 4. \346\270\254\350\251\246\345\255\227\345\205\203\350\210\207 ASCII \350\275\211\346\217\233 (ord, chr) ---"

l_str.36:                               ; @str.36
	.asciz	"A"

	.p2align	4, 0x0                          ; @str.37
l_str.37:
	.asciz	"ord('A') \347\232\204 ASCII \347\242\274 ="

	.p2align	4, 0x0                          ; @str.38
l_str.38:
	.asciz	"chr(66) \351\202\204\345\216\237\347\232\204\345\255\227\345\205\203 ="

l_str.39:                               ; @str.39
	.space	1

	.p2align	4, 0x0                          ; @str.40
l_str.40:
	.asciz	"--- 5. \346\270\254\350\251\246\347\263\273\347\265\261\347\213\200\346\205\213 (time, random) ---"

	.p2align	4, 0x0                          ; @str.41
l_str.41:
	.asciz	"\347\225\266\345\211\215\346\231\202\351\226\223\346\210\263 time():"

	.p2align	4, 0x0                          ; @str.42
l_str.42:
	.asciz	"\347\224\242\347\224\237\347\232\204\344\272\202\346\225\270 random():"

l_str.43:                               ; @str.43
	.space	1

	.p2align	4, 0x0                          ; @str.44
l_str.44:
	.asciz	">>> \351\226\213\345\247\213\345\237\267\350\241\214\347\263\273\347\265\261\345\207\275\346\225\270\346\270\254\350\251\246 <<<"

l_str.45:                               ; @str.45
	.space	1

	.p2align	4, 0x0                          ; @str.46
l_str.46:
	.asciz	"--- 6. \346\270\254\350\251\246 I/O \350\210\207\345\274\267\345\210\266\347\265\202\346\255\242 (input, exit) ---"

	.p2align	4, 0x0                          ; @str.47
l_str.47:
	.asciz	"\350\253\213\350\274\270\345\205\245\344\273\273\346\204\217\346\226\207\345\255\227 (\346\210\226\347\233\264\346\216\245\346\214\211 Enter \347\271\274\347\272\214): "

	.p2align	4, 0x0                          ; @str.48
l_str.48:
	.asciz	"\344\275\240\345\211\233\346\211\215\350\274\270\345\205\245\347\232\204\346\230\257:"

	.p2align	4, 0x0                          ; @str.49
l_str.49:
	.asciz	"\346\272\226\345\202\231\345\221\274\345\217\253 exit(0) \347\265\220\346\235\237\350\231\233\346\223\254\346\251\237..."

	.p2align	4, 0x0                          ; @str.50
l_str.50:
	.asciz	"\351\200\231\350\241\214\346\260\270\351\201\240\344\270\215\346\207\211\350\251\262\350\242\253\345\215\260\345\207\272\344\276\206\357\274\201\345\233\240\347\202\272 VM \345\267\262\347\266\223\347\265\202\346\255\242\344\272\206\357\274\201"

.subsections_via_symbols
