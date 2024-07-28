#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(c_variadic)]

use milkv_rs::{timer, uart};



pub mod entry;
pub mod interrupt_vector;
pub mod io;
pub mod panic;
pub mod prelude;
pub mod ray;
pub mod uart_sys;
pub mod vga;

use milkv_rs::*;

pub unsafe fn run_second_core() {
    let addr: usize;
    core::arch::asm!(
        "la {0}, _second_core_start",
        out(reg) addr
    );
    platform::reset_c906l_to_addr(addr)
}

#[no_mangle]
pub extern "C" fn os_main() {
    io::print("Entered second stage\n");
    io::print("Starting second core\n");
    unsafe { run_second_core() }

    io::print("Initializing video buffer\n");
    unsafe {
        vga::init_vga();
    }

    unsafe {
        io::print("Enabling interrupts\n");
        init_interrupts();
        io::print("Interrupts enabled\n");
    }

    // unsafe {
    //     io::SOUT = |data| {
    //         uart::print_bytes(data);
    //         // vga::print(data);
    //     }
    // }

    io::print("Doing the funny stuff\n");

    // unsafe{
    //     mipi_test();
    // }
    // timer::mdelay(1500);

    // let mut buf = [0; 512];
    // loop {
    //     let buf = read(&mut buf);
    //     unsafe{
    //         vga::print(buf);
    //     }
    // }

    ray::RayTrace::default().start_ray();
    loop {}
}

extern "C" {
    #[link_name = "start_ray"]
    fn start_ray();

}

#[no_mangle]
pub extern "C" fn draw_pix(x: core::ffi::c_int, y: core::ffi::c_int, r: u8, g: u8, b: u8) {
    unsafe {
        (vga::FRAME_BUF as *mut u8)
            .add(x as usize + y as usize * vga::WIDTH as usize)
            .write_volatile(vga::Color::new(r, g, b).0);
    }
}

#[no_mangle]
pub extern "C" fn flush() {
    vga::flush_frame(vga::FRAME_BUF as usize)
}

#[no_mangle]
pub unsafe extern "C" fn delay(time: core::ffi::c_int) {
    timer::mdelay(time as u64);
}

#[no_mangle]
pub unsafe extern "C" fn printf(str: *const core::ffi::c_char, mut args: ...) {
    let str = core::ffi::CStr::from_ptr(str as *const i8);
    let bytes = str.to_bytes();

    struct WriteO;
    impl core::fmt::Write for WriteO {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            unsafe {
                vga::print(s.as_bytes());
            }
            Ok(())
        }
    }
    use core::fmt::Write;

    for (i, part) in bytes.split(|c| *c == b'%').enumerate() {
        if i == 0 {
            vga::print(part);
            continue;
        }
        if let Some(part) = part.strip_prefix(b"d") {
            _ = write!(WriteO, "{}", args.arg::<core::ffi::c_int>() as i32);
            vga::print(part);
        } else if let Some(part) = part.strip_prefix(b"p") {
            _ = write!(WriteO, "{}", args.arg::<*const core::ffi::c_void>() as i32);
            vga::print(part);
        } else if let Some(part) = part.strip_prefix(b"s") {
            vga::print(
                core::ffi::CStr::from_ptr(args.arg::<*const core::ffi::c_char>()).to_bytes(),
            );
            vga::print(part);
        } else if let Some(part) = part.strip_prefix(b"c") {
            vga::print(&[args.arg::<core::ffi::c_char>() as u8]);
            vga::print(part);
        } else if part.starts_with(b"%") {
            vga::print(part);
        }
    }
}

#[no_mangle]
unsafe extern "C" fn puts(str: *const core::ffi::c_char) -> core::ffi::c_int {
    unsafe {
        let str = core::ffi::CStr::from_ptr(str as *const i8);
        let bytes = str.to_bytes();
        vga::print(bytes);
    }
    0
}

#[inline(always)]
pub fn read(buf: &mut [u8]) -> &mut [u8] {
    unsafe {
        let fd = 0usize;
        let size;
        core::arch::asm!(
            "ecall",
            in("a7") 63,
            inout("a0") fd => size,
            in("a1") buf.as_ptr(),
            in("a2") buf.len(),
        );
        &mut buf[..size]
    }
}

#[inline(always)]
pub fn write(buf: &[u8]) {
    unsafe {
        let fd = 0usize;
        let _size: usize;
        core::arch::asm!(
            "ecall",
            in("a7") 64,
            inout("a0") fd => _size,
            in("a1") buf.as_ptr(),
            in("a2") buf.len(),
        );
    }
}

unsafe fn mipi_test() {
    // pin mux

    let val = 1;

    //CTRL_PAD_MIPI_TXM4
    (0x0300_116C as *mut u32).write_volatile(val);

    //CTRL_PAD_MIPI_TXM4
    (0x0300_1194 as *mut u32).write_volatile(val);
    //CTRL_PAD_MIPI_TXP4
    (0x0300_1198 as *mut u32).write_volatile(val);

    //CTRL_PAD_MIPI_TXM3
    (0x0300_119C as *mut u32).write_volatile(val);
    //CTRL_PAD_MIPI_TXP3
    (0x0300_11A0 as *mut u32).write_volatile(val);

    //CTRL_PAD_MIPI_TXM2
    (0x0300_11A4 as *mut u32).write_volatile(val);
    //CTRL_PAD_MIPI_TXP2
    (0x0300_11A8 as *mut u32).write_volatile(val);

    //CTRL_PAD_MIPI_TXM1
    (0x0300_11AC as *mut u32).write_volatile(val);
    //CTRL_PAD_MIPI_TXP1
    (0x0300_11B0 as *mut u32).write_volatile(val);

    //CTRL_PAD_MIPI_TXM0
    (0x0300_11B4 as *mut u32).write_volatile(val);
    //CTRL_PAD_MIPI_TXP0
    (0x0300_11B8 as *mut u32).write_volatile(val);

    #[repr(C)]
    struct MIPITx {
        dsi_mac_reg_00: u32,
        dsi_mac_reg_01: u32,
        dsi_mac_reg_02: u32,
        dsi_mac_reg_03: u32,
        dsi_mac_reg_04: u32,
        dsi_mac_reg_05: u32,
        dsi_mac_reg_06: u32,
        dsi_mac_reg_07: u32,
        dsi_mac_reg_08: u32,
        dsi_mac_reg_09: u32,
    }

    #[repr(C)]
    struct MIPITxPHY {
        reg_00: u32,
        reg_01: u32,
        reg_02: u32,
        reg_03: u32,
        reg_04: u32,
        reg_05: u32,
        _reserved0: [u32; 29],
        reg_23: u32,
        reg_24: u32,
        reg_25: u32,
        reg_26: u32,
        reg_27: u32,
        reg_28: u32,
        _reserved1: [u32; 4],
        reg_2d: u32,
    }

    let tx_reg = 0x0A08A000 as *mut MIPITx;
    let tx_phy = 0x0A0D1000 as *mut MIPITxPHY;
    // clk_lane_en + data_0_lane_en
    core::ptr::addr_of_mut!((*tx_phy).reg_00).write_volatile(0b1);

    loop {
        // core::ptr::addr_of_mut!((*tx_reg).dsi_mac_reg_00).write_volatile(0b1);
    }
}

unsafe fn init_interrupts() {
    io::print("well this is fun1\n");
    // trigger an interrupt NOW
    timer::set_timercmp(timer::get_mtimer());

    plic::clear();

    // all enabled interrupts allowed
    plic::mint_threshhold(0);

    //--------------- uart ----------------------
    uart_sys::init();
    //-------------------------------------

    //--------------- timer 0 initialization ----------------------
    interrupt_vector::add_plic_handler(interrupt::TIMER0, |_, _, _, _| {
        gpio::set_gpio(mmio::GPIO0, 29, !gpio::read_gpio(mmio::GPIO0, 29));
        timer::mm::clear_int(mmio::TIMER0);
    });

    // timer 0 interrupt number
    plic::set_priority(interrupt::TIMER0, 2);
    plic::enable_m_interrupt(interrupt::TIMER0);

    // initialize timer0
    timer::mm::set_mode(mmio::TIMER0, timer::mm::TimerMode::Count);
    // quarter second
    timer::mm::set_load_value(mmio::TIMER0, timer::SYS_COUNTER_FREQ_IN_SECOND as u32 / 4);
    timer::mm::set_enabled(mmio::TIMER0, true);
    //-------------------------------------

    csr::enable_timer_interrupt();
    csr::enable_external_interrupt();
    csr::enable_interrupts();
}
