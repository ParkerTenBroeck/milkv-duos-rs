use milkv_rs::csr::*;

use crate::io;

use milkv_rs::csr::*;

core::arch::global_asm!(
    r#"
    .globl  _stage0_start

    .option norvc
    .section .text.entry,"ax",@progbits
    .globl _stage0_start
    _stage0_start:

        li x10, 0
    li x1, 0
    li x2, 0
    li x3, 0
    li x4, 0
    li x5, 0
    li x6, 0
    li x7, 0
    li x8, 0
    li x9, 0
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

    csrw sscratch, x0
  
    # set {mxstatus} to init value
    li x3, 0xc0638000
    csrw {mxstatus}, x3
  
    # set plic_ctrl = 1
    li x3, 0x701FFFFC # plic_base + 0x1FFFFC
    li x4, 1
    sw x4 , 0(x3)
  
    # invalid I-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x11
    csrs {mcor}, x3
    # enable I-cache
    li x3, 0x1
    csrs {mhcr}, x3
    # invalid D-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x12
    csrs {mcor}, x3
    # enable D-cache
    li x3, 0x2
    csrs {mhcr}, x3

    # enable floating point
    li x3, 0x1 << 13
    csrs mstatus, x3

    la sp, __STACKS_END__
  
    la a3, __BSS_START__
    la a4, __BSS_END__
  
    beq a3, a4, .bss_clear_end  
    sd x0, 0(a3)
    addi a4, a4, 7
  .bss_clear:
    addi a3, a3, 8
    sd x0, 0(a3)
    ble a3, a4, .bss_clear
  .bss_clear_end:
  
    call _stage0_main
    j .die
  
    .balign 4
  .die:
    ebreak
    j .die
    "#,

    mxstatus = const mxstatus,
    mcor = const mcor,
    mhcr = const mhcr,
    // mhint = const mhint,
);
core::arch::global_asm!(
    r#"
    .globl  _second_core_start

    .option norvc
    .section .text,"ax",@progbits
    .globl _second_core_start

    _second_core_start:

    la sp, SECOND_CORE_STACK + {stack_size}

    csrw mscratch, x0
  
    # write mtvec and make sure it sticks
    # la t0, mtrap_vector
    # csrw mtvec, t0

    # set {mxstatus} to init value
    li x3, 0xc0638000
    csrw {mxstatus}, x3
  
    # set plic_ctrl = 1
    #li x3, 0x701FFFFC # plic_base + 0x1FFFFC
    #li x4, 1
    #sw x4 , 0(x3)
  
    # invalid I-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x11
    csrs {mcor}, x3
    # enable I-cache
    li x3, 0x1
    csrs {mhcr}, x3
    
    # invalid D-cache
    li x3, 0x33
    csrc {mcor}, x3
    li x3, 0x12
    csrs {mcor}, x3
    # enable D-cache
    li x3, 0x2
    csrs {mhcr}, x3

    jal second_core_main
    "#,

    mxstatus = const milkv_rs::csr::mxstatus,
    mcor = const milkv_rs::csr::mcor,
    mhcr = const milkv_rs::csr::mhcr,
    stack_size = const core::mem::size_of::<Stack>(),
    // mhint = const mhint,
);

#[no_mangle]
unsafe extern "C" fn second_core_main() -> ! {
    unsafe { vga::run_vga(crate::vga::FRAME_BUF as usize) }
}

const STACK_SIZE: usize = 256;
type Stack = [u64; STACK_SIZE];
#[no_mangle]
static mut SECOND_CORE_STACK: Stack = [0; STACK_SIZE];
