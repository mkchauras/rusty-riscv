.section .text._entry
.global _start

_start:
	# Setup stack
	la sp, ____stack_start____
	# Setup Trap Handler
	la t0, ____mtrap_location____
	csrw mtvec, t0
	jal riscv_main
	.word 0x00000000
	jal .

.section .text._mtrap_vec
.global _mtrap_vec

_mtrap_vec:
	csrr a0, mcause
	csrr a1, mtval
	csrr a2, mepc
	jal mtrap_handler
	csrr t0, mepc
	addi t0, t0, 0x04
	csrw mepc, t0
	mret
