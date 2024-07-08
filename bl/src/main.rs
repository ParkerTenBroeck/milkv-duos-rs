#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;
pub mod cmd;

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
// 		uart::print("By pass rtc mode switch\n");
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
// 	// uart::print("st_on_reason=%x\n", read_data);
// 	read_data = mmio_read_32!(REG_RTC_BASE + RTC_ST_OFF_REASON);
// 	// uart::print("st_off_reason=%x\n", read_data);

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

unsafe fn vga() -> !{
    mmio_write_32!(0x130 + 0x03002000, 0x1);
    gpio::set_gpio_direction(mmio::GPIO1, 13, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 14, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 15, gpio::Direction::Output);

    let mut val = 0;
    // let mut start = timer::get_mtimer();
    let ptr = core::ptr::addr_of_mut!((*mmio::GPIO1).dr);
    loop{
        val += 1 << 13;
        // let val = (val & 0b111) << 13;
        ptr.write_volatile(val);
        // timer::mdelay(100);
        // let mut end = timer::get_mtimer();
        // while end.wrapping_sub(start) < 99{
        //     end = timer::get_mtimer();
        // }
        // start = end;
        // idelay(1);
    }
}

fn idelay(v: u64){
    for _ in 0..v{
        unsafe{
            core::arch::asm!("nop")
        }
    }
}

#[no_mangle]
pub extern "C" fn bl_rust_main() {
    timer::mdelay(250);
    unsafe {

        uart::console_init();

    }
    timer::mdelay(250);
    uart::print("\n\n\nBooted into firmware\nInitialized uart to 115200\n");

    unsafe {
        if let Err(_) = security::efuse::lock_efuse() {
            reset();
        } else {
            uart::print("Locked efuse\n");
        }
    }

    // unsafe{
    //     vga();
    // }
 

    // unsafe {
    //     setup_dl_flag()
    // }
    // uart::print("setup dl flag\n");


    // unsafe {
    //     switch_rtc_mode_1st_stage()
    // }
    // uart::print("setup first stage rtc mode\n");

    // unsafe {
    //     set_rtc_en_registers()
    // }
    // uart::print("enabled rtc registers\n");

  
    unsafe {
        ddr::init_ddr();
    }
    uart::print("DDR initialized\n");

    unsafe {
        // set pinmux to enable output of LED pin
        mmio_write_32!(0x03001074, 0x3);
        gpio::set_gpio_direction(mmio::GPIO0, 29, gpio::Direction::Output);
    }
    uart::print("Connfigured pinmux(LED pin 29)\n");

    uart::print("Enabling interrupts\n");
    unsafe {
        csr::enable_timer_interrupt();
        csr::enable_interrupts();
        // trigger an interrupt NOW
        timer::set_timercmp(timer::get_mtimer());


        // plic is seen as a single external interrupt source
        csr::enable_external_interrupt();
        // all enabled interrupts allowed
        plic::mint_threshhold(0);

        //--------------- timer 0 initialization ----------------------
        interrupt_vector::add_plic_handler(interrupt::TIMER0, ||{
            gpio::set_gpio(mmio::GPIO0, 29, !gpio::read_gpio(mmio::GPIO0, 29));
            timer::mm::clear_int(mmio::TIMER0);
        });
        
        // timer 0 interrupt number
        plic::set_priority(interrupt::TIMER0, 1);
        plic::enable_m_interrupt(interrupt::TIMER0);

        // initialize timer0
        timer::mm::set_mode(mmio::TIMER0, timer::mm::TimerMode::Count);
        // quarter second
        timer::mm::set_load_value(mmio::TIMER0, timer::SYS_COUNTER_FREQ_IN_SECOND as u32 / 4);
        timer::mm::set_enabled(mmio::TIMER0, true);
        //-------------------------------------


        plic::set_priority(interrupt::UART0, 1);
        plic::enable_m_interrupt(interrupt::UART0);
    }
    uart::print("Interrupts enabled\n");


    uart::print("Starting console\n");

    cmd::run();
}


