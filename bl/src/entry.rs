use crate::csr_reg::*;

core::arch::global_asm!(
    r#"
    .globl  bl_entrypoint

    .option norvc
    .section .text.entry,"ax",@progbits
    .globl bl_entrypoint
  bl_entrypoint:

    li x1, 0
    li x2, 0
    li x3, 0
    li x4, 0
    li x5, 0
    li x6, 0
    li x7, 0
    li x8, 0
    li x9, 0
    li x10, 0
    li x11, 0
    li x12, 0
    li x13, 0
    li x14, 0
    li x15, 0
    li x16, 0
    li x17, 0
    li x18, 0
    li x19, 0
    li x20, 0
    li x21, 0
    li x22, 0
    li x23, 0
    li x24, 0
    li x25, 0
    li x26, 0
    li x27, 0
    li x28, 0
    li x29, 0
    li x30, 0
    li x31, 0
  
    csrw mscratch, x0
  
    # write mtvec and make sure it sticks
    la t0, trap_vector
    csrw mtvec, t0
    // csrw stvec, t0
  
    # set mxstatus to init value
    li x3, 0xc0638000
    csrw {mxstatus}, x3
  
    # set plic_ctrl = 1
    li x3, 0x701FFFFC # plic_base + 0x1FFFFC
    li x4, 1
    sw x4 , 0(x3)
  
    // invalidate all memory for BTB,BHT,DCACHE,ICACHE
    li x3, 0x30013
    csrs {mcor}, x3
    
    // enable ICACHE,DCACHE,BHT,BTB,RAS,WA
    #li x3, 0x7f
    #csrs {mhcr}, x3
  
    # invalid I-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x11
    csrs {mcor}, x3

    # enable I-cache
    #li x3, 0x1
    #csrs {mhcr}, x3
    
    # invalid D-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x12
    csrs {mcor}, x3

    # enable D-cache
    #li x3, 0x2
    #csrs {mhcr}, x3
  
    // enable data_cache_prefetch, amr
    #li x3, 0x610c
    #csrs {mhint}, x3 #mhint
  
    # enable fp
    li x3, 0x1 << 13
    csrs mstatus, x3
  
    la sp, __STACKS_END__
  
    la a3, __BSS_START__
    la a4, __BSS_END__
    sub a4, a4, a3
  
  bss_clear:
    sd x0, 0(a3)
    addi a3, a3, 8
    addi a4, a4, -8
    bnez a4, bss_clear
  
    call bl_rust_main

    j die
  
  
    .balign 4
  die:
    ebreak
    j die
    "#,

    mxstatus = const mxstatus,
    mcor = const mcor,
    mhcr = const mhcr,
    mhint = const mhint,
);


core::arch::global_asm!(
    r#"
    .globl  trap_vector

    .section .text.trap_vector,"ax",@progbits
    .globl trap_vector

    .balign 4
    trap_vector:
        # j trap_handler
        addi sp, sp, -8 * 29
        sw x1, 1 * 8( sp )
        sw x5, 2 * 8( sp )
        sw x6, 3 * 8( sp )
        sw x7, 4 * 8( sp )
        sw x8, 5 * 8( sp )
        sw x9, 6 * 8( sp )
        sw x10, 7 * 8( sp )
        sw x11, 8 * 8( sp )
        sw x12, 9 * 8( sp )
        sw x13, 10 * 8( sp )
        sw x14, 11 * 8( sp )
        sw x15, 12 * 8( sp )
        sw x16, 13 * 8( sp )
        sw x17, 14 * 8( sp )
        sw x18, 15 * 8( sp )
        sw x19, 16 * 8( sp )
        sw x20, 17 * 8( sp )
        sw x21, 18 * 8( sp )
        sw x22, 19 * 8( sp )
        sw x23, 20 * 8( sp )
        sw x24, 21 * 8( sp )
        sw x25, 22 * 8( sp )
        sw x26, 23 * 8( sp )
        sw x27, 24 * 8( sp )
        sw x28, 25 * 8( sp )
        sw x29, 26 * 8( sp )
        sw x30, 27 * 8( sp )
        sw x31, 28 * 8( sp )

        csrr t0, mstatus
        sw t0, 29 * 8( sp )

        csrr a0, mcause
        csrr a1, mepc

        # test if asynchronous
        srli a2, a0, 64 - 1		/* MSB of mcause is 1 if handing an asynchronous interrupt - shift to LSB to clear other bits. */
        beq a2, x0, handle_synchronous		/* Branch past interrupt handing if not asynchronous. */
        	

    handle_asynchronous:
        sw a1, 0( sp )
        jal trap_handler
        j return

    handle_synchronous:
        addi a1, a1, 4
        sw a1, 0( sp )
        jal trap_handler


    return:

        lw t0, 0(sp)
        csrw mepc, t0

        lw t0, 29 * 8(sp)
        csrw mstatus, t0

        
        lw x1, 1 * 8( sp )
        lw x5, 2 * 8( sp )
        lw x6, 3 * 8( sp )
        lw x7, 4 * 8( sp )
        lw x8, 5 * 8( sp )
        lw x9, 6 * 8( sp )
        lw x10, 7 * 8( sp )
        lw x11, 8 * 8( sp )
        lw x12, 9 * 8( sp )
        lw x13, 10 * 8( sp )
        lw x14, 11 * 8( sp )
        lw x15, 12 * 8( sp )
        lw x16, 13 * 8( sp )
        lw x17, 14 * 8( sp )
        lw x18, 15 * 8( sp )
        lw x19, 16 * 8( sp )
        lw x20, 17 * 8( sp )
        lw x21, 18 * 8( sp )
        lw x22, 19 * 8( sp )
        lw x23, 20 * 8( sp )
        lw x24, 21 * 8( sp )
        lw x25, 22 * 8( sp )
        lw x26, 23 * 8( sp )
        lw x27, 24 * 8( sp )
        lw x28, 25 * 8( sp )
        lw x29, 26 * 8( sp )
        lw x30, 27 * 8( sp )
        lw x31, 28 * 8( sp ) 
        addi sp, sp, 8 * 29

        mret
    "#
);