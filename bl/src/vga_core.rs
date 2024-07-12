use milkv_rs::*;
use embedded_graphics::text::renderer::CharacterStyle;

use crate::vga;


pub unsafe fn update_vga(){
    static mut NUMBER: usize = 0;
    NUMBER += 1;
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
    _ = write!(buff, "{NUMBER}");
    Text::with_alignment(
        unsafe{core::str::from_utf8_unchecked(&buff.0[0..buff.1])},
        display.bounding_box().center() + Point::new(0, 0),
        character_style,
        Alignment::Left,
    )
    .draw(display).unwrap();

    unsafe{
        crate::vga::flush_frame()
    }
}

#[no_mangle]
static mut STACK: [u64; 4] = [0; 4];

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

    la sp, STACK - 32
    jal second_core_main
    ",

    mxstatus = const csr::mxstatus,
    mcor = const csr::mcor,
    mhcr = const csr::mhcr,
    // mhint = const mhint,
);

#[no_mangle]
extern "C" fn second_core_main() -> !{
    unsafe { vga::run_vga() }
}

pub unsafe fn run_on_second(){
    let addr: usize;
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
const FRAME_BUF: *mut FrameBuf = 0x80000000usize as *mut FrameBuf;


pub unsafe fn init_vga(){
    let mem = unsafe { core::slice::from_raw_parts_mut(FRAME_BUF as *mut u8, WIDTH * HEIGHT) };
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

    unsafe{
        vga::flush_frame()
    }
}