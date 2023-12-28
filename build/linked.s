	.text
	.file	"llvm-link"
	.globl	print                           # -- Begin function print
	.p2align	4, 0x90
	.type	print,@function
print:                                  # @print
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	write
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	print, .Lfunc_end0-print
	.cfi_endproc
                                        # -- End function
	.globl	fib                             # -- Begin function fib
	.p2align	4, 0x90
	.type	fib,@function
fib:                                    # @fib
# %bb.0:                                # %entry
	pushq	%r14
	pushq	%rbx
	pushq	%rax
	xorl	%r14d, %r14d
	cmpq	$2, %rdi
	jl	.LBB1_3
# %bb.1:                                # %entry.else.preheader
	movq	%rdi, %rbx
	.p2align	4, 0x90
.LBB1_2:                                # %entry.else
                                        # =>This Inner Loop Header: Depth=1
	leaq	-1(%rbx), %rdi
	callq	fib@PLT
	leaq	-2(%rbx), %rdi
	addq	%rax, %r14
	cmpq	$3, %rbx
	movq	%rdi, %rbx
	ja	.LBB1_2
.LBB1_3:                                # %_ret
	addq	%rdi, %r14
	movq	%r14, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	retq
.Lfunc_end1:
	.size	fib, .Lfunc_end1-fib
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$40, %edi
	callq	fib@PLT
	movq	%rax, %rdi
	callq	print@PLT
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	main, .Lfunc_end2-main
	.cfi_endproc
                                        # -- End function
	.globl	write                           # -- Begin function write
	.p2align	4, 0x90
	.type	write,@function
write:                                  # @write
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rsi
	movl	$.L.str, %edi
	xorl	%eax, %eax
	callq	printf
	cltq
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	write, .Lfunc_end3-write
	.cfi_endproc
                                        # -- End function
	.type	.L.str,@object                  # @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"%ld\n"
	.size	.L.str, 5

	.ident	"Ubuntu clang version 12.0.1-19ubuntu3"
	.section	".note.GNU-stack","",@progbits
