.section .text._entry
.global _start

_start:
	# Setup stack
	la sp, ____stack_start____
	j riscv_main
	jal .
