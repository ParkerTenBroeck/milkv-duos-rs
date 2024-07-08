#![no_std]
#![feature(asm_const)]
#![feature(c_size_t)]
#![allow(clippy::missing_safety_doc)]

pub mod csr;
pub mod ddr;
pub mod gpio;
pub mod mmio;
pub mod pinmux;
pub mod plic;
pub mod security;
pub mod timer;
pub mod uart;
pub mod system;
pub mod watchdog;
pub mod interrupt;
pub mod rom_api;
pub mod platform;
pub mod rtc;
pub mod mmap;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    crate::timer::mdelay(500);
    crate::uart::flush();

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
