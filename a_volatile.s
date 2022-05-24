	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 3
	.globl	_wait_while_0                   ; -- Begin function wait_while_0
	.p2align	2
_wait_while_0:                          ; @wait_while_0
	.cfi_startproc
; %bb.0:
LBB0_1:                                 ; =>This Inner Loop Header: Depth=1
	ldr	w8, [x0]
	cbz	w8, LBB0_1
; %bb.2:
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
