#![no_std]
#![no_main]
#![feature(asm_const)]

use milkv_rs::{timer, uart};

pub mod entry;
pub mod panic;
pub mod prelude;

const STACK_SIZE: usize = 4;
type Stack = [u64; STACK_SIZE];
#[no_mangle]
static mut SECOND_CORE_STACK: Stack = [0; STACK_SIZE];

core::arch::global_asm!(
    r#"
    .globl  _os_start

    .option norvc
    .section .text.entry,"ax",@progbits
    .globl _os_start

    _os_start:

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
extern "C" fn second_core_main() -> !{
    unsafe { vga::run_vga(0x90000000) }
}