use milkv_rs::csr::*;

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
    la t0, mtrap_vector
    csrw mtvec, t0
    // csrw stvec, t0
  
    # set mxstatus to init value
    li x3, 0xc0638000
    csrw {mxstatus}, x3
  
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
