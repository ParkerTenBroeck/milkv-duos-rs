#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod cmd;
pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;

use csr::{disable_interrupts, enable_interrupts};
use platform::{
    fip_param1, PINMUX_BASE, PWM0_BASE, PWM_HLPERIOD0, PWM_OE, PWM_PERIOD0, PWM_START,
    REG_APLL0_CSR, REG_APLL_SSC_SYN_CTRL, REG_APLL_SSC_SYN_SET, REG_PLL_G2_CTRL,
    REG_PLL_G2_SSC_SYN_CTRL,
};
pub use prelude::*;
use timer::{odelay, udelay};

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

unsafe fn vga() {
    // mmio_write_32!(0x130 + 0x03002000, 0x1);
    // gpio::set_gpio_direction(mmio::GPIO1, 13, gpio::Direction::Output);
    // gpio::set_gpio_direction(mmio::GPIO1, 14, gpio::Direction::Output);
    // gpio::set_gpio_direction(mmio::GPIO1, 15, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 1, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 2, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 3, gpio::Direction::Output);
    gpio::set_gpio_direction(mmio::GPIO1, 15, gpio::Direction::Input);
    gpio::set_gpio_direction(mmio::GPIO1, 14, gpio::Direction::Input);

    // gpio::set_gpio_direction(mmio::GPIO2, 12, gpio::Direction::Output);
    // gpio::set_gpio_direction(mmio::GPIO2, 13, gpio::Direction::Output);
    // gpio::set_gpio_direction(mmio::GPIO2, 14, gpio::Direction::Output);

    let mut bruh_timing = 4;
    let ptr = core::ptr::addr_of_mut!((*mmio::GPIO1).dr);

    loop {
        // let div_clk_axi6 = (0x03002000 + 0x0bc) as *mut u32;
        // div_clk_axi6.write_volatile((1<<3) | (bruh_timing << 16) | (1));
        // bruh_timing -= 1;
        // bruh_timing &= 0xF;
        // if bruh_timing == 0{
        //     bruh_timing = 4;
        // }
        unsafe {
            disable_interrupts();
        }

        for _ in 0..120 {
            // let mut bool = false;

            // let start = timer::get_mtimer();
            // let start_off = 1048659 * timer::SYS_COUNTER_FREQ_IN_US / 1000;
            // while timer::get_mtimer().wrapping_sub(start) < start_off {}

            while !gpio::read_gpio(mmio::GPIO1, 14) {}
            while gpio::read_gpio(mmio::GPIO1, 14) {}
            // currently on back porch
            timer::udelay(600);
            // let start = timer::get_mtimer();
            // let start_off = (10486594 * timer::SYS_COUNTER_FREQ_IN_US) / 10000;
            // while timer::get_mtimer().wrapping_sub(start) < start_off {}

            for i in 0..480 {
                while gpio::read_gpio(mmio::GPIO1, 15) {}
                while !gpio::read_gpio(mmio::GPIO1, 15) {}
                let mut goal = timer::get_mtimer();
                goal = goal.wrapping_add(60);
                while timer::get_mtimer() < goal {}
                goal = goal.wrapping_add(26 * timer::SYS_COUNTER_FREQ_IN_US);
                // timer::udelay(1049);
                // v line 31.777557100298 us;
                // while !gpio::read_gpio(mmio::GPIO1, 15){}

                let mut pval = 0;
                while timer::get_mtimer() < goal {
                    pval += 1;
                    let pval = (pval & 0b111) << 1;
                    ptr.write_volatile(pval);
                    // ptr.write_volatile(0);
                    // ptr.write_volatile(0b111 << 1);
                    // val += 1;
                    // if bool{

                    //     ptr.write_volatile(0b111 << 1);
                    // }else{
                    //     ptr.write_volatile(0);
                    // }
                }
                ptr.write_volatile(0);
                // bool ^= true;

                // let start = timer::get_mtimer();
                // let i = 0;
                // // let start = timer::get_mtimer();
                // let line_off = (i * (317775571 + 254220456) * timer::SYS_COUNTER_FREQ_IN_US) / 10000000;

                // while timer::get_mtimer().wrapping_sub(start) < line_off {}

                // while gpio::read_gpio(mmio::GPIO1, 15){}
            }
        }
        unsafe { enable_interrupts() }
        // timer::mdelay(100);
        // let mut end = timer::get_mtimer();
        // while end.wrapping_sub(start) < 10{
        //     end = timer::get_mtimer();
        // }
        // start = end;
        // timer::udelay(1);
        // idelay(30);
    }
}

fn idelay(v: u64) {
    for _ in 0..v {
        unsafe { core::arch::asm!("nop") }
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
        interrupt_vector::add_plic_handler(interrupt::TIMER0, || {
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

    uart::print("Loading second stage\n");
    let dst = 0x80000000 as *mut core::ffi::c_void;
    let off = unsafe { (*mmio::PARAM1).bl2_img_size } as usize + core::mem::size_of::<fip_param1>();
    let size = unsafe { (*mmio::PARAM1).param2_size } as usize;
    unsafe {
        rom_api::p_rom_api_load_image(dst, off as u32, size, 1);
    }
    uart::print("\n");
    // let src = unsafe{(*mmio::PARAM1).bl2_img_size};
    // let count = 0;

    // uart::print("Copying second stage to DDR ram\n");
    // unsafe{
    //     core::ptr::copy_nonoverlapping(src, dst, count)
    // }
    // uart::print("Starting second stage\n");

    uart::print("Speeding up clock\n");
    unsafe {

        {
            // OD clk setting
            let mut value;
            let mut byp0_value;

            let pll_syn_set = [
                614400000, // set apll synthesizer  98.304 M
                610080582, // set disp synthesizer  99 M
                610080582, // set cam0 synthesizer  99 M
                //586388132, // set cam1 synthesizer  103 M
                615164587, // set cam1 synthesizer  98.18181818 M
            ];

            let pll_csr = [
                0x00208201, // set apll *16/2 (786.432 MHz)
                0x00188101, // set disp *12/1 (1188 MHz)
                // 0x00188101, // set cam0 *12/1 (1188 MHz)
                0x00308201, // set cam0 *24/2 (1188 MHz)
                //0x00148101, // set cam1 *10/1 (1030 MHz)
                0x00168101, // set cam1 *11/1 (1080 MHz)
            ];

            // NOTICE("PLLS/OD.\n");
            unsafe fn config_core_power(low_period: u32) {
                /*
                 * low_period = 0x42; // 0.90V
                 * low_period = 0x48; // 0.93V
                 * low_period = 0x4F; // 0.96V
                 * low_period = 0x58; // 1.00V
                 * low_period = 0x5C; // 1.02V
                 * low_period = 0x62; // 1.05V
                 * low_period = 0x62; // 1.05V
                 */
                mmio_write_32!(PWM0_BASE + PWM_HLPERIOD0, low_period);
                mmio_write_32!(PWM0_BASE + PWM_PERIOD0, 0x64);
                mmio_write_32!(PINMUX_BASE + 0xA4, 0x0); // set pinmux for pwm0
                mmio_write_32!(PWM0_BASE + PWM_START, 0x1); // enable bit0:pwm0
                mmio_write_32!(PWM0_BASE + PWM_OE, 0x1); // output enable bit0:pwm0
                timer::mdelay(10);
            }

            // set vddc for OD clock
            config_core_power(0x58); //1.00V

            // store byp0 value
            byp0_value = mmio_read_32!(0x03002030);

            // switch clock to xtal
            mmio_write_32!(0x03002030, 0xffffffff);
            mmio_write_32!(0x03002034, 0x0000003f);

            //set mipipll = 900MHz
            mmio_write_32!(0x03002808, 0x05488101);

            // set synthersizer clock
            mmio_write_32!(REG_PLL_G2_SSC_SYN_CTRL, 0x3F); // enable synthesizer clock enable,
                                                           // [0]: 1: MIPIMPLL(900)/1=900MHz,
                                                           //      0: MIPIMPLL(900)/2=450MHz

            for i in 0..4 {
                mmio_write_32!(REG_APLL_SSC_SYN_SET + 0x10 * i, pll_syn_set[i as usize]); // set pll_syn_set

                value = mmio_read_32!(REG_APLL_SSC_SYN_CTRL + 0x10 * i);
                value |= 1; // [0]: sw update (w1t: write one toggle)
                value &= !(1 << 4); // [4]: bypass = 0
                mmio_write_32!(REG_APLL_SSC_SYN_CTRL + 0x10 * i, value);

                mmio_write_32!(REG_APLL0_CSR + 4 * i, pll_csr[i as usize]); // set pll_csr
            }

            value = mmio_read_32!(REG_PLL_G2_CTRL);
            value = value & (!0x00011111);
            mmio_write_32!(REG_PLL_G2_CTRL, value); //clear all pll PD

            // set mpll = 1050MHz
            mmio_write_32!(0x03002908, 0x05548101);

            // set clk_sel_23: [23] clk_sel for clk_c906_0 = 1 (DIV_IN0_SRC_MUX)
            // set clk_sel_24: [24] clk_sel for clk_c906_1 = 1 (DIV_IN0_SRC_MUX)
            mmio_write_32!(0x03002020, 0x01800000);

            // set div, src_mux of clk_c906_0: [20:16]div_factor=1, [9:8]clk_src = 3 (mpll), 1050/1 = 1050MHz
            mmio_write_32!(0x03002130, 0x00010309);

            // set div, src_mux of clk_c906_1: [20:16]div_factor=1, [9:8]clk_src = 1 (a0pll), 786.432/1 = 786.432MHz
            mmio_write_32!(0x03002138, 0x00010109);


            // set tpll = 1400MHz
            mmio_write_32!(0x0300290C, 0x07708101);

            mmio_write_32!(0x03002048, 0x00020109); //clk_cpu_axi0 = DISPPLL(1188) / 2
            mmio_write_32!(0x03002054, 0x00020009); //clk_tpu = TPLL(1400) / 2 = 700MHz
            mmio_write_32!(0x03002064, 0x00080009); //clk_emmc = FPLL(1500) / 8 = 187.5MHz
            mmio_write_32!(0x03002088, 0x00080009); //clk_spi_nand = FPLL(1500) / 8 = 187.5MHz
            mmio_write_32!(0x03002098, 0x00200009); //clk_sdma_aud0 = APLL(786.432) / 32 = 24.576MHz
            mmio_write_32!(0x03002120, 0x000F0009); //clk_pwm_src = FPLL(1500) / 15 = 100MHz
            mmio_write_32!(0x030020A8, 0x00010009); //clk_uart -> clk_cam0_200 = XTAL(25) / 1 = 25MHz
            mmio_write_32!(0x030020E4, 0x00030209); //clk_axi_video_codec = CAM1PLL(1080) / 3 = 360MHz
            mmio_write_32!(0x030020EC, 0x00020109); //clk_vc_src0 = MIPIPLL(900) / 2 = 450MHz
            mmio_write_32!(0x030020C8, 0x00030009); //clk_axi_vip = MIPIPLL(900) / 3 = 300MHz
            mmio_write_32!(0x030020D0, 0x00060309); //clk_src_vip_sys_0 = FPLL(1500) / 6 = 250MHz
            mmio_write_32!(0x030020D8, 0x00040209); //clk_src_vip_sys_1 = DISPPLL(1188)/ 4 = 297MHz
            mmio_write_32!(0x03002110, 0x00020209); //clk_src_vip_sys_2 = DISPPLL(1188) / 2 = 594MHz
                                                    //mmio_write_32(0x03002140, 0x00020009); //clk_src_vip_sys_3 = MIPIPLL(900) / 2 = 450MHz
            mmio_write_32!(0x03002144, 0x00030309); //clk_src_vip_sys_4 = FPLL(1500) / 3 = 500MHz

            // set hsperi clock to PLL (FPLL) div by 5  = 300MHz
            mmio_write_32!(0x030020B8, 0x00050009); //--> CLK_AXI4

            // set rtcsys clock to PLL (FPLL) div by 5  = 300MHz
            mmio_write_32!(0x0300212C, 0x00050009); // CLK_SRC_RTC_SYS_0

            unsafe fn mmio_clrbits_32(addr: u32, clear: u32) {
                mmio_write_32!(addr, mmio_read_32!(addr) & !clear);
            }

            // disable powerdown, mipimpll_d3_pd[2] = 0
            mmio_clrbits_32(0x030028A0, 0x4);

            // disable powerdown, cam0pll_d2_pd[1]/cam0pll_d3_pd[2] = 0
            mmio_clrbits_32(0x030028AC, 0x6);

            //wait for pll stable
            timer::udelay(200);

            // switch clock to PLL from xtal except clk_axi4 & clk_spi_nand
            byp0_value &= (
                1 << 8 | //clk_spi_nand
                   1 << 19
                //clk_axi4
            );
            mmio_write_32!(0x03002030, byp0_value); // REG_CLK_BYPASS_SEL0_REG
            mmio_write_32!(0x03002034, 0x0); // REG_CLK_BYPASS_SEL1_REG
        }

                // const test: u32 = !(1<<6);
        // const v: u32 = 0x03002000 + 0x134;
        // // Clock Bypass to xtal for c906_0
        // let clk_byp_1_6 = (0x03002000 + 0x034) as *mut u32;
        // let div_clk_c906_0_0 = (0x03002000 + 0x130) as *mut u32;
        // let div_clk_c906_0_1 = (0x03002000 + 0x134) as *mut u32;
        // let clk_sel_0 = (0x03002000 + 0x020) as *mut u32;
        // // clk_sel_0.write_volatile(1<<23);
        // clk_byp_1_6.write_volatile(!(1<<6));
        // let clk_en_0 = (0x03002000 + 0x000) as *mut u32;
        // // clk_gpio_db
        // // clk_apb_gpio
        // // clk_en_0.write_volatile(!((1<<31) | (1<<29)));

        let div_clk_axi6 = (0x03002000 + 0x0bc) as *mut u32;
        div_clk_axi6.write_volatile((1 << 3) | 0x40000 | (1));

        // let div_clk_1m = (0x03002000 + 0x0fc) as *mut u32;
        // div_clk_1m.write_volatile((1 << 3) | 0x10000 | (1));

        // let div_clk_gpio_db = (0x03002000 + 0x094) as *mut u32;
        // div_clk_gpio_db.write_volatile((1 << 3) | 0x10000 | (1));

        // // clk_axi6
        // let clk_byp_0 = (0x03002000 + 0x030) as *mut u32;
        // clk_byp_0.write_volatile(!(1 << 20));


        // let div_clk_1m = (0x03002000 + 0x0fc) as *mut u32;
        // div_clk_1m.write_volatile((1<<3) | 0xA0000 | (1));
    }

    uart::print("Starting VGA\n");
    unsafe { vga() }
    uart::print("Starting console\n");

    cmd::run();
}
