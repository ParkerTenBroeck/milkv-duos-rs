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
        sd x1, 1 * 8( sp )
        sd x5, 2 * 8( sp )
        sd x6, 3 * 8( sp )
        sd x7, 4 * 8( sp )
        sd x8, 5 * 8( sp )
        sd x9, 6 * 8( sp )
        sd x10, 7 * 8( sp )
        sd x11, 8 * 8( sp )
        sd x12, 9 * 8( sp )
        sd x13, 10 * 8( sp )
        sd x14, 11 * 8( sp )
        sd x15, 12 * 8( sp )
        sd x16, 13 * 8( sp )
        sd x17, 14 * 8( sp )
        sd x18, 15 * 8( sp )
        sd x19, 16 * 8( sp )
        sd x20, 17 * 8( sp )
        sd x21, 18 * 8( sp )
        sd x22, 19 * 8( sp )
        sd x23, 20 * 8( sp )
        sd x24, 21 * 8( sp )
        sd x25, 22 * 8( sp )
        sd x26, 23 * 8( sp )
        sd x27, 24 * 8( sp )
        sd x28, 25 * 8( sp )
        sd x29, 26 * 8( sp )
        sd x30, 27 * 8( sp )
        sd x31, 28 * 8( sp )

        csrr t0, mstatus
        sd t0, 29 * 8( sp )

        csrr a0, mcause
        csrr a1, mepc
        csrr a2, mtval

        # test if asynchronous
        srli a2, a0, 64 - 1		/* MSB of mcause is 1 if handing an asynchronous interrupt - shift to LSB to clear other bits. */
        beq a2, x0, handle_synchronous		/* Branch past interrupt handing if not asynchronous. */
        	

    handle_asynchronous:
        sd a1, 0( sp )
        jal trap_handler
        j return

    handle_synchronous:
        addi a1, a1, 4
        sd a1, 0( sp )
        jal trap_handler


    return:

        ld t0, 0(sp)
        csrw mepc, t0

        ld t0, 29 * 8(sp)
        csrw mstatus, t0

        
        ld x1, 1 * 8( sp )
        ld x5, 2 * 8( sp )
        ld x6, 3 * 8( sp )
        ld x7, 4 * 8( sp )
        ld x8, 5 * 8( sp )
        ld x9, 6 * 8( sp )
        ld x10, 7 * 8( sp )
        ld x11, 8 * 8( sp )
        ld x12, 9 * 8( sp )
        ld x13, 10 * 8( sp )
        ld x14, 11 * 8( sp )
        ld x15, 12 * 8( sp )
        ld x16, 13 * 8( sp )
        ld x17, 14 * 8( sp )
        ld x18, 15 * 8( sp )
        ld x19, 16 * 8( sp )
        ld x20, 17 * 8( sp )
        ld x21, 18 * 8( sp )
        ld x22, 19 * 8( sp )
        ld x23, 20 * 8( sp )
        ld x24, 21 * 8( sp )
        ld x25, 22 * 8( sp )
        ld x26, 23 * 8( sp )
        ld x27, 24 * 8( sp )
        ld x28, 25 * 8( sp )
        ld x29, 26 * 8( sp )
        ld x30, 27 * 8( sp )
        ld x31, 28 * 8( sp ) 
        addi sp, sp, 8 * 29

        mret
    "#
);

/* RISC-V */
pub const PLIC_BASE: u32 = 0x70000000;


// pub const CLINT_MTIME(cnt): u32 = asm volatile("csrr %0, time\n" : "=r"(cnt) :: "memory");

/* PLIC */
pub const PLIC_PRIORITY0: u32 = (PLIC_BASE + 0x0);
pub const PLIC_PRIORITY1: u32 = (PLIC_BASE + 0x4);
pub const PLIC_PRIORITY2: u32 = (PLIC_BASE + 0x8);
pub const PLIC_PRIORITY3: u32 = (PLIC_BASE + 0xc);
pub const PLIC_PRIORITY4: u32 = (PLIC_BASE + 0x10);

pub const PLIC_PENDING1: u32 = (PLIC_BASE + 0x1000);
pub const PLIC_PENDING2: u32 = (PLIC_BASE + 0x1004);
pub const PLIC_PENDING3: u32 = (PLIC_BASE + 0x1008);
pub const PLIC_PENDING4: u32 = (PLIC_BASE + 0x100C);

pub const PLIC_ENABLE1: u32 = (PLIC_BASE + 0x2000);
pub const PLIC_ENABLE2: u32 = (PLIC_BASE + 0x2004);
pub const PLIC_ENABLE3: u32 = (PLIC_BASE + 0x2008);
pub const PLIC_ENABLE4: u32 = (PLIC_BASE + 0x200C);

pub const PLIC_THRESHOLD: u32 = (PLIC_BASE + 0x200000);
pub const PLIC_CLAIM: u32 = (PLIC_BASE + 0x200004);