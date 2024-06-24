#![no_std]
#![no_main]
#![no_builtins]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(cfg_target_has_atomic)]
#![feature(asm_const)]

pub mod mem;
pub mod uart;
pub mod prelude;
pub mod timer;
pub mod mmio;
pub use prelude::*;
use uart::print_b;

#[allow(non_upper_case_globals)]
pub const mxstatus: u32 = 0x7C0;
#[allow(non_upper_case_globals)]
pub const mhcr: u32 = 0x7C1;
#[allow(non_upper_case_globals)]
pub const mcor: u32 = 0x7C2;
#[allow(non_upper_case_globals)]
pub const mccr2: u32 = 0x7C3;
#[allow(non_upper_case_globals)]
pub const mcer2: u32 = 0x7C4;
#[allow(non_upper_case_globals)]
pub const mhint: u32 = 0x7C5;
#[allow(non_upper_case_globals)]
pub const mrmr: u32 = 0x7C6;
#[allow(non_upper_case_globals)]
pub const mrvbr: u32 = 0x7C7;

core::arch::global_asm!(
    r#"
    .globl  bl_entrypoint

    .option norvc
    .section .text.entry,"ax",@progbits
    .globl bl_entrypoint
  bl_entrypoint:
    #auipc a0,0x0
    #auipc a0,0x0

    li x1, 0
    li x2, 0
    li x3, 0
    li x4, 0
    li x5, 0
    li x6, 0
    li x7, 0
    li x8, 0
    li x9, 0
    # li x10, 0
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
    li x3, 0x7f
    csrs {mhcr}, x3
  
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
  
    // enable data_cache_prefetch, amr
    li x3, 0x610c
    csrs {mhint}, x3 #mhint
  
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
  
  
    la a0,bl_entrypoint
    call bl_rust_main

    j die
  
  
    .balign 4
  trap_vector:
  die:
    j panic_handler
    j die

    .bss
    .align 12
    .section .rodata
    .align 7
    "#,

    mxstatus = const mxstatus,
    mcor = const mcor,
    mhcr = const mhcr,
    mhint = const mhint,
);

fn print_u64(val: u64){
  for val in val.to_be_bytes(){
    for x in [val >> 4, val & 0xf]{
      match x{
        0..=9 => print_b(x + b'0'),
        10..=15 => print_b(x + b'A' - 10),
        _ => {}
      }
    }
  }
}

fn print_u8(val: u8){
  for x in [val >> 4, val & 0xf]{
    match x{
      0..=9 => print_b(x + b'0'),
      10..=15 => print_b(x + b'A' - 10),
      _ => {}
    }
  }
}

#[no_mangle]
pub extern "C" fn bl_rust_main(mut start: u64) {

    timer::mdelay(250);
    unsafe {
        uart::console_init();
    }
    timer::mdelay(250);
  
    loop {

      uart::print("Start Addr: 0x");
      print_u64(start);
      uart::print("\n");

      uart::print("Timer Val: 0x");
      print_u64(timer::get_timer_value());
      uart::print("\n");
      // continue;

      uart::print("                  ");
      for v in 0..16{
        uart::print(" ");
        print_u8(v);
      }
      uart::print("\n");
      for v in (0..256).step_by(16) {
        let start = (start  & !(16 - 1)) + v;
        uart::print("0x");
        print_u64(start);
        for v in 0..16{
          let curr = start + v;
          uart::print(" ");
          print_u8(unsafe{(curr as *const u8).read_volatile()});
        };
        uart::print(" ");
        for v in 0..16{
          let curr = start + v;
          match unsafe{(curr as *const u8).read_volatile()}{
            val if val.is_ascii_graphic() => print_b(val),
            _ => print_b(b'.')
          }
        }
        uart::print("\n");
      }

      uart::print("\n");
      uart::print("\n");
      uart::print("\n");

      uart::flush();

      timer::mdelay(1000);

      println!("test, {start}");

      start += 256;
    }
}

#[panic_handler]
pub fn rust_panic_handler(_info: &core::panic::PanicInfo) -> ! {
  uart::print("Rust panic... resetting\n");
  unsafe{ reset() }
}

#[no_mangle]
pub extern "C" fn panic_handler() -> !{
  uart::print("Non rust Panic ???... resetting\n");
  unsafe{ reset() }
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> !{

  uart::flush();

  macro_rules! mmio_write_32 {
      ($ptr:expr, $val:expr) => {
          ($ptr as *mut u32).write_volatile($val)
      };
  }
  macro_rules! mmio_read_32 {
    ($ptr:expr) => {
        ($ptr as *const u32).read_volatile()
    };
}
	// enable rtc wdt reset
	mmio_write_32!(0x050260E0, 0x0001); //enable rtc_core wathdog reset enable
	mmio_write_32!(0x050260C8, 0x0001); //enable rtc_core power cycle   enable

  	// mmio_write_32(0x05025018,0x00FFFFFF); //Mercury rtcsys_rstn_src_sel
	mmio_write_32!(0x050250AC, 0x00000000); //cv181x rtcsys_rstn_src_sel
	mmio_write_32!(0x05025004, 0x0000AB18);
	mmio_write_32!(0x05025008, 0x00400040); //enable rtc_ctrl wathdog reset enable

  mmio_write_32!(0x03010004, 0x00000066); //config watch dog 166ms
	mmio_write_32!(0x0301001c, 0x00000020);
	mmio_write_32!(0x0301000c, 0x00000076);
	mmio_write_32!(0x03010000, 0x00000011);

  // wait pmu state to ON
  while mmio_read_32!(0x050260D4) != 0x00000003 {}
  mmio_write_32!(0x05025008, 0x00080008);

  core::hint::unreachable_unchecked()
}
