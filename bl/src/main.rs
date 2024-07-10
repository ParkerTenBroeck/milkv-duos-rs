#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod cmd;
pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;
pub mod vga;

use embedded_graphics::text::renderer::CharacterStyle;
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

// fn idelay(v: u64) {
//     for _ in 0..v {
//         unsafe { core::arch::asm!("nop") }
//     }
// }

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


    // unsafe{
    //     core::arch::asm!(
    //         "
    //         # invalid I-cache
    //         li x3, 0x33
    //         csrc {mcor}, x3
    //         li x3, 0x11
    //         csrs {mcor}, x3
    //         # enable I-cache
    //         li x3, 0x1
    //         csrc {mhcr}, x3
            
    //         # invalid D-cache
    //         li x3, 0x33
    //         csrc {mcor}, x3
    //         li x3, 0x12
    //         csrs {mcor}, x3
    //         # enable D-cache
    //         li x3, 0x2
    //         csrc {mhcr}, x3
    //         ",
    //         mcor = const csr::mcor,
    //         mhcr = const csr::mhcr,
    //     );
    // }

    uart::print("Speeding up pll\n");
    unsafe {
        milkv_rs::platform::init_pll_speed();
    }


    uart::print("Initializing DDR memory video buffer\n");
    unsafe{
        init_vga();
    }
    unsafe{
        core::arch::asm!("
            th.sync
            th.dcache.call
        ");
    }
    uart::print("Testing second core\n");
    unsafe {
        test_second_core()
    }

    // uart::print("Starting VGA\n");
    // unsafe { vga::vga2() }
    uart::print("Starting console\n");

    // cmd::run();
    let mut i = 0;
    loop{
        i += 1;
        println!("nyaa: {i}");
        timer::mdelay(16);


        use embedded_graphics::{
            mono_font::MonoTextStyle,
            prelude::*,
            text::{Alignment, Text},
        };
        use embedded_graphics::mono_font::*;
        let display = unsafe { &mut *FRAME_BUF };

        let mut character_style = MonoTextStyle::new(&ascii::FONT_5X7, Color::White);
        character_style.set_background_color(Option::Some(Color::Black));

        let mut buff = [0u8; 256];
        struct Buff<'a>(&'a mut[u8], usize);
        impl<'a> core::fmt::Write for Buff<'a>{
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.0[self.1..(self.1+s.len())].copy_from_slice(s.as_bytes());
                self.1 += s.len();
                Ok(())
            }
        }
        use core::fmt::Write;
        let mut buff = Buff(&mut buff, 0);
        _ = write!(buff, "{i}");
        Text::with_alignment(
            unsafe{core::str::from_utf8_unchecked(&buff.0[0..buff.1])},
            display.bounding_box().center() + Point::new(0, 0),
            character_style,
            Alignment::Left,
        )
        .draw(display).unwrap();

        unsafe{
            for i in (0..(WIDTH * HEIGHT)).step_by(64){
                core::arch::asm!("
                    th.dcache.cpa {0}
                ",
                in(reg) 0x80000000usize + i);
            }
        }
    }
}

#[no_mangle]
static mut STACK: [u64; 256*4] = [0; 256*4];

core::arch::global_asm!(
    "
    second_core_start:
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

    la sp, STACK - 8192
    jal second_core_main
    ",

    mxstatus = const csr::mxstatus,
    mcor = const csr::mcor,
    mhcr = const csr::mhcr,
    // mhint = const mhint,
);

#[no_mangle]
extern "C" fn second_core_main() -> !{
    timer::mdelay(1000);
    uart::print("Second core starting\n");


    uart::print("Starting VGA\n");
    unsafe { vga::vga2() }
}

unsafe fn test_second_core(){
    let addr;
    core::arch::asm!(
        "la {0}, second_core_start",
        out(reg) addr
    );
    platform::reset_c906l(addr)
}

const WIDTH: usize = 640 / 2;
const HEIGHT: usize = 480 / 2;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl From<Color> for embedded_graphics::pixelcolor::raw::RawU8 {
    fn from(value: Color) -> Self {
        (value as u8).into()
    }
}

impl embedded_graphics::pixelcolor::PixelColor for Color {
    type Raw = embedded_graphics::pixelcolor::raw::RawU8;
}


type FrameBuf = embedded_graphics::framebuffer::Framebuffer<
    Color,
    embedded_graphics::pixelcolor::raw::RawU8,
    embedded_graphics::pixelcolor::raw::LittleEndian,
    WIDTH,
    HEIGHT,
    { embedded_graphics::framebuffer::buffer_size::<Color>(WIDTH, HEIGHT) },
    >;
const FRAME_BUF: *mut FrameBuf = 0x80000000 as *mut FrameBuf;


unsafe fn init_vga(){
    let mem = unsafe { core::slice::from_raw_parts_mut(0x80000000 as *mut u8, WIDTH * HEIGHT) };
    let mut iter = mem.chunks_mut(WIDTH * HEIGHT / 8);
    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = i as u8;
        }
    }
    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = (i / 2) as u8;
        }
    }
    let hor = iter.next().unwrap();
    for (i, line) in hor.chunks_mut(WIDTH).enumerate() {
        for pix in line {
            *pix = i as u8;
        }
    }
    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = if i & 1 == 0 { u8::MAX } else { 0 };
        }
    }
    for (i, rest) in iter.enumerate() {
        rest.fill(i as u8 + 1)
    }


    let display = unsafe { &mut *FRAME_BUF };

    use embedded_graphics::{
        mono_font::MonoTextStyle,
        prelude::*,
        primitives::{
            Circle, PrimitiveStyle, Rectangle, Triangle,
        },
        text::{Alignment, Text},
    };
    use embedded_graphics::mono_font::*;

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Color::Red, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Color::Green, 3);

    let fill = PrimitiveStyle::with_fill(Color::Blue);
    let character_style = MonoTextStyle::new(&ascii::FONT_5X7, Color::White);

    let yoffset = HEIGHT as i32/8 *3 + 5;

    // Draw a 3px wide outline around the display.
    // display
    //     .bounding_box()
    //     .into_styled(border_stroke)
    //     .draw(display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(display).unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(display).unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(display).unwrap();

    // Draw centered text.
    let text = "embedded-graphics :3";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(display).unwrap();

    for i in 0..256{
        let c = i as u8 as char;
        let mut tmp = [0u8; 4];
        let s = c.encode_utf8(&mut tmp);
        Text::with_alignment(
            s,
            Point::new((i%64)*5, 15 + 7 + (i/64) * 7) 
            + display.bounding_box().center().y_axis(),
            character_style,
            Alignment::Left,
        )
        .draw(display).unwrap();
    }
}