#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod cmd;
pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;
// pub mod vga;
pub mod io;

use platform::fip_param1;
pub use prelude::*;

// unsafe fn setup_dl_flag() {
// 	let v = milkv_rs::rom_api::p_rom_api_get_boot_src();
//     use platform::*;

// 	match v {
//         boot_src::BOOT_SRC_UART =>
// 		    mmio_write_32!(BOOT_SOURCE_FLAG_ADDR, MAGIC_NUM_UART_DL),
//         boot_src::BOOT_SRC_SD =>
// 		    mmio_write_32!(BOOT_SOURCE_FLAG_ADDR, MAGIC_NUM_SD_DL),
//         boot_src::BOOT_SRC_USB =>
// 		    mmio_write_32!(BOOT_SOURCE_FLAG_ADDR, MAGIC_NUM_USB_DL),
//         _ =>
//             mmio_write_32!(BOOT_SOURCE_FLAG_ADDR, v as u32),
// 	}
// }

// unsafe fn switch_rtc_mode_1st_stage() {

//     use rtc::*;

// 	let mut read_data: u32;
// 	let mut write_data: u32;
// 	let rtc_mode: u32;

// // #ifdef CV181X_SUPPORT_SUSPEND_RESUME
// // 	void (*warmboot_entry)(void) = get_warmboot_entry();

// // 	if (warmboot_entry == (void *)BL31_WARMBOOT_ENTRY)
// // 		return;
// // #endif

// 	// reg_rtc_mode = rtc_ctrl0[10]
// 	read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0);
// 	rtc_mode = (read_data >> 10) & 0x1;
// 	if rtc_mode == 0x1 {
// 		io::print("By pass rtc mode switch\n");
// 		return;
// 	}

// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0_UNLOCKKEY, 0xAB18);
// 	read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0);

// 	// reg_clk32k_cg_en = rtc_ctrl0[11] -> 0
// 	write_data = 0x08000000 | (read_data & 0xfffff7ff);
// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0, write_data);

// 	//cg_en_out_clk_32k = rtc_ctrl_status0[25]
// 	read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0_STATUS0);
// 	while (read_data & 0x02000000) != 0x00 {
// 		read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0_STATUS0);
//     }

// 	read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0);
// 	//r eg_rtc_mode = rtc_ctrl0[10];
// 	write_data = 0x04000000 | (read_data & 0xfffffbff) | (0x1 << 10);
// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0, write_data);

// 	// DA_SOC_READY = 1
// 	mmio_write_32!(RTC_MACRO_BASE + 0x8C, 0x01);
// 	// DA_SOC_READY = 0
// 	mmio_write_32!(RTC_MACRO_BASE + 0x8C, 0x0);

// 	timer::udelay(200); // delay ~200us

// 	read_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0);
// 	// reg_clk32k_cg_en = rtc_ctrl0[11] -> 1
// 	write_data = 0x0C000000 | (read_data & 0xffffffff) | (0x1 << 11);
// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0, write_data); //rtc_ctrl0
// }

// unsafe fn set_rtc_en_registers() {
// 	let mut write_data: u32;
// 	let mut read_data: u32;
//     use rtc::*;

// 	read_data = mmio_read_32!(REG_RTC_BASE + RTC_ST_ON_REASON);
// 	// io::print("st_on_reason=%x\n", read_data);
// 	read_data = mmio_read_32!(REG_RTC_BASE + RTC_ST_OFF_REASON);
// 	// io::print("st_off_reason=%x\n", read_data);

// 	mmio_write_32!(REG_RTC_BASE + RTC_EN_SHDN_REQ, 0x01);
// 	while mmio_read_32!(REG_RTC_BASE + RTC_EN_SHDN_REQ) != 0x01{}

// 	mmio_write_32!(REG_RTC_BASE + RTC_EN_WARM_RST_REQ, 0x01);
// 	while mmio_read_32!(REG_RTC_BASE + RTC_EN_WARM_RST_REQ) != 0x01{}

// 	mmio_write_32!(REG_RTC_BASE + RTC_EN_PWR_CYC_REQ, 0x01);
// 	while mmio_read_32!(REG_RTC_BASE + RTC_EN_PWR_CYC_REQ) != 0x01{}

// 	mmio_write_32!(REG_RTC_BASE + RTC_EN_WDT_RST_REQ, 0x01);
// 	while mmio_read_32!(REG_RTC_BASE + RTC_EN_WDT_RST_REQ) != 0x01{}

//     unsafe fn mmio_setbits_32(addr: u32, set: u32) {
//         mmio_write_32!(addr, mmio_read_32!(addr) | set);
//     }

//     unsafe fn mmio_clrbits_32(addr: u32, clear: u32) {
//         mmio_write_32!(addr, mmio_read_32!(addr) & !clear);
//     }

// 	// Set rtcsys_rst_ctrl[24] = 1; bit 24 is reg_rtcsys_reset_en
// 	mmio_setbits_32(REG_RTC_CTRL_BASE + RTC_POR_RST_CTRL, 0x1);

// 	// rtc_ctrl0_unlockkey
// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0_UNLOCKKEY, 0xAB18);

// 	// Enable hw_wdg_rst_en
// 	write_data = mmio_read_32!(REG_RTC_CTRL_BASE + RTC_CTRL0);
// 	write_data = 0xffff0000 | write_data | (0x1 << 11) | (0x01 << 6);
// 	mmio_write_32!(REG_RTC_CTRL_BASE + RTC_CTRL0, write_data);

// 	// Avoid power up again after poweroff
// 	mmio_clrbits_32(REG_RTC_BASE + RTC_EN_PWR_VBAT_DET, 1 << 2);
// }

// fn idelay(v: u64) {
//     for _ in 0..v {
//         unsafe { core::arch::asm!("nop") }
//     }
// }

// static mut DDR: u8 = 1;
// static mut EFUSE_LOCK: u8 = 1;
// static mut PLL_SPEED: u8 = 1;

#[no_mangle]
pub extern "C" fn bl_rust_main(init: usize) {
    if init == 0{
        timer::mdelay(250);
        unsafe {
            uart::console_init();
        }
        timer::mdelay(250);
        io::print("\n\n\nBooted into firmware\nInitialized uart to 115200\n");
    }

    crate::println!("{init}");

    unsafe{
        // init_interrupts();
        // csr::disable_interrupts();
        // csr::disable_external_interrupt();
        // csr::disable_timer_interrupt();
        // io::print("Disabled Interrupts\n");
    }

    unsafe {
        if init == 0{
            if let Err(_) = security::efuse::lock_efuse() {
                reset();
            } else {
                io::print("Locked efuse\n");
                // EFUSE_LOCK = 2;
            }
        }
    }

    unsafe {
        if init == 0{
            ddr::init_ddr();
            // DDR = 2;
            io::print("DDR initialized\n");
        }
    }

    unsafe {
        // set pinmux to enable output of LED pin
        mmio_write_32!(0x03001074, 0x3);
        gpio::set_gpio_direction(mmio::GPIO0, 29, gpio::Direction::Output);
    }
    io::print("Connfigured pinmux(LED pin 29)\n");

    // io::print("Loading second stage\n");
    // let dst = 0x80000000 as *mut core::ffi::c_void;
    // let off = unsafe { (*mmio::PARAM1).bl2_img_size } as usize + core::mem::size_of::<fip_param1>();
    // let size = unsafe { (*mmio::PARAM1).param2_size } as usize;
    // unsafe {
    //     rom_api::p_rom_api_load_image(dst, off as u32, size, 1);
    // }
    // io::print("\n");

    io::print("Speeding up pll\n");
    unsafe {
        if init == 0{
            milkv_rs::platform::init_pll_speed();
        }
        // PLL_SPEED = 2;
    }

    
    io::print("Starting console\n");
    cmd::run();
    io::print("Press key for early console\n");
    
    let time = timer::get_mtimer().wrapping_add(timer::SYS_COUNTER_FREQ_IN_SECOND * 2);
    while timer::get_mtimer() < time{
        if uart::has_b(){
        }            
    }
    // unsafe{
    //     core::arch::asm!("
    //     jr {0}
    // ", in(reg) dst)
    // }
}

unsafe fn init_interrupts() {
    plic::clear();

    // //--------------- timer 0 initialization ----------------------
    // interrupt_vector::add_plic_handler(interrupt::TIMER0, || {
    //     gpio::set_gpio(mmio::GPIO0, 29, !gpio::read_gpio(mmio::GPIO0, 29));
    //     timer::mm::clear_int(mmio::TIMER0);
    // });

    // // timer 0 interrupt number
    // plic::set_priority(interrupt::TIMER0, 1);
    // plic::enable_m_interrupt(interrupt::TIMER0);

    // // initialize timer0
    // timer::mm::set_mode(mmio::TIMER0, timer::mm::TimerMode::Count);
    // // quarter second
    // timer::mm::set_load_value(mmio::TIMER0, timer::SYS_COUNTER_FREQ_IN_SECOND as u32 / 4);
    // timer::mm::set_enabled(mmio::TIMER0, true);
    // //-------------------------------------


    // // all enabled interrupts allowed
    // plic::mint_threshhold(0);
    // // plic is seen as a single external interrupt source
    // csr::enable_external_interrupt();

    csr::enable_timer_interrupt();
    csr::enable_interrupts();


    // trigger an interrupt NOW
    timer::set_mtimercmp(timer::get_mtimer());
}
