use embedded_graphics::{mono_font::MonoTextStyleBuilder, text::renderer::CharacterStyle};
use milkv_rs::*;

pub use vga::*;

pub unsafe fn update_vga() {
    static mut NUMBER: usize = 0;
    NUMBER += 1;
    // timer::mdelay(16);

    use embedded_graphics::mono_font::*;
    use embedded_graphics::{
        mono_font::MonoTextStyle,
        prelude::*,
        text::{Alignment, Text},
    };
    let display = unsafe { &mut *FRAME_BUF };

    let mut character_style = MonoTextStyle::new(&ascii::FONT_5X7, Color::WHITE);
    character_style.set_background_color(Option::Some(Color::BLACK));

    let mut buff = [0u8; 256];
    struct Buff<'a>(&'a mut [u8], usize);
    impl<'a> core::fmt::Write for Buff<'a> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.0[self.1..(self.1 + s.len())].copy_from_slice(s.as_bytes());
            self.1 += s.len();
            Ok(())
        }
    }
    use core::fmt::Write;
    let mut buff = Buff(&mut buff, 0);
    _ = write!(buff, "{NUMBER}");
    Text::with_alignment(
        unsafe { core::str::from_utf8_unchecked(&buff.0[0..buff.1]) },
        display.bounding_box().center() + Point::new(0, 0),
        character_style,
        Alignment::Left,
    )
    .draw(display)
    .unwrap();

    crate::vga::flush_frame(FRAME_BUF as usize)
}

// const STACK_SIZE: usize = 4;
// type Stack = [u64; STACK_SIZE];
// #[no_mangle]
// static mut SECOND_CORE_STACK: Stack = [0; STACK_SIZE];

// core::arch::global_asm!(
//     "
//     second_core_start:

//     la sp, SECOND_CORE_STACK + {stack_size}

//     csrw mscratch, x0

//     # write mtvec and make sure it sticks
//     la t0, mtrap_vector
//     csrw mtvec, t0

//     # set {mxstatus} to init value
//     li x3, 0xc0638000
//     csrw {mxstatus}, x3

//     # set plic_ctrl = 1
//     #li x3, 0x701FFFFC # plic_base + 0x1FFFFC
//     #li x4, 1
//     #sw x4 , 0(x3)

//     # invalid I-cache
//     li x3, 0x33
//     csrc {mcor}, x3
//     li x3, 0x11
//     csrs {mcor}, x3
//     # enable I-cache
//     li x3, 0x1
//     csrs {mhcr}, x3

//     # invalid D-cache
//     li x3, 0x33
//     csrc {mcor}, x3
//     li x3, 0x12
//     csrs {mcor}, x3
//     # enable D-cache
//     li x3, 0x2
//     csrs {mhcr}, x3

//     jal second_core_main
//     ",

//     mxstatus = const csr::mxstatus,
//     mcor = const csr::mcor,
//     mhcr = const csr::mhcr,
//     stack_size = const core::mem::size_of::<Stack>(),
//     // mhint = const mhint,
// );

// #[no_mangle]
// extern "C" fn second_core_main() -> !{
//     unsafe { vga::run_vga(FRAME_BUF as usize) }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u8);

impl Color {
    const RED: Color = Color::new(255, 0, 0);
    const GREEN: Color = Color::new(0, 255, 0);
    const BLUE: Color = Color::new(0, 0, 255);

    const BLACK: Color = Color::new(0, 0, 0);
    const WHITE: Color = Color::new(255, 255, 255);
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(blue & 0b11000000 | (green & 0b11100000) >> 2 | (red & 0b11100000) >> 5)
    }

    pub const fn hsv(h: u8, s: u8, v: u8) -> Self {
        let r;
        let g;
        let b;

        let region;
        let remainder;
        let p;
        let q;
        let t;

        if s == 0 {
            r = v;
            g = v;
            b = v;
            return Color::new(r, g, b);
        }

        region = h / 43;
        remainder = (h as u32 - (region as u32 * 43)) * 6;

        p = ((v as u32 * (255 - s) as u32) >> 8) as u8;
        q = ((v as u32 * (255 - ((s as u32 * remainder as u32) >> 8))) >> 8) as u8;
        t = ((v as u32 * (255 - ((s as u32 * (255 - remainder as u32)) >> 8))) >> 8) as u8;

        match region {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            _ => {
                r = v;
                g = p;
                b = q;
            }
        }

        return Color::new(r, g, b);
    }
}

impl From<Color> for embedded_graphics::pixelcolor::raw::RawU8 {
    fn from(value: Color) -> Self {
        value.0.into()
    }
}

impl embedded_graphics::pixelcolor::PixelColor for Color {
    type Raw = embedded_graphics::pixelcolor::raw::RawU8;
}

pub type FrameBuf = embedded_graphics::framebuffer::Framebuffer<
    Color,
    embedded_graphics::pixelcolor::raw::RawU8,
    embedded_graphics::pixelcolor::raw::LittleEndian,
    { WIDTH as usize },
    { HEIGHT as usize },
    { embedded_graphics::framebuffer::buffer_size::<Color>(WIDTH as usize, HEIGHT as usize) },
>;
pub static mut FRAME_BUF: *mut FrameBuf =
    (0x80000000usize + 0x20000000 - core::mem::size_of::<FrameBuf>()) as *mut FrameBuf;

pub unsafe fn init_vga() {
    let mem =
        unsafe { core::slice::from_raw_parts_mut(FRAME_BUF as *mut u8, (WIDTH * HEIGHT) as usize) };
    let mut iter = mem.chunks_mut((WIDTH * HEIGHT) as usize / 8);
    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH as usize) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = Color::hsv(i as u8, 255, 255).0;
        }
    }

    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH as usize) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = i as u8;
        }
    }
    let hor = iter.next().unwrap();
    for (i, line) in hor.chunks_mut(WIDTH as usize).enumerate() {
        for pix in line {
            *pix = i as u8;
        }
    }
    let vert = iter.next().unwrap();
    for line in vert.chunks_mut(WIDTH as usize) {
        for (i, pix) in line.iter_mut().enumerate() {
            *pix = if i & 1 == 0 { u8::MAX } else { 0 };
        }
    }
    for (i, rest) in iter.enumerate() {
        rest.fill(i as u8 + 1)
    }

    for pix in &mut mem[..WIDTH as usize] {
        *pix = 0xFF;
    }

    let display = unsafe { &mut *FRAME_BUF };

    use embedded_graphics::mono_font::*;
    use embedded_graphics::{
        mono_font::MonoTextStyle,
        prelude::*,
        primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
        text::{Alignment, Text},
    };

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Color::RED, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Color::GREEN, 3);

    let fill = PrimitiveStyle::with_fill(Color::BLUE);
    let character_style = MonoTextStyle::new(&ascii::FONT_5X8, Color::WHITE);

    let yoffset = HEIGHT as i32 / 8 * 3 + 5;

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
    .draw(display)
    .unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(display)
        .unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(display)
        .unwrap();

    // Draw centered text.
    let text = "embedded-graphics :3";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();

    for i in 0..256 {
        let c = i as u8 as char;
        let mut tmp = [0u8; 4];
        let s = c.encode_utf8(&mut tmp);
        Text::with_alignment(
            s,
            Point::new((i % 64) * 5, 15 + 8 + (i / 64) * 8)
                + display.bounding_box().center().y_axis(),
            character_style,
            Alignment::Left,
        )
        .draw(display)
        .unwrap();
    }

    vga::flush_frame(FRAME_BUF as usize)
}

//------------------- ANSI VGA Controller --------------------------
#[derive(Debug)]
pub struct VGAAnsiController {
    line: i32,
    col: i32,

    bg: Color,
    fg: Color,

    inverted_c: bool,
    bold: bool,
    italic: bool,
    strike: Option<Color>,
    underline: Option<Color>,

    scroll_start: i32,
    scroll_end: i32,
}

macro_rules! un_print {
    ($($arg:tt)*) => {{
        // $crate::println!($($arg)*)
    }};
}

const CHAR: embedded_graphics::mono_font::MonoFont =
    embedded_graphics::mono_font::iso_8859_10::FONT_6X13;
const CHAR_BOLD: embedded_graphics::mono_font::MonoFont =
    embedded_graphics::mono_font::iso_8859_10::FONT_6X13_BOLD;
const CHAR_ITALIC: embedded_graphics::mono_font::MonoFont =
    embedded_graphics::mono_font::iso_8859_10::FONT_6X13_ITALIC;
const COLS: i32 = (vga::WIDTH / CHAR.character_size.width as u64) as i32;
const LINES: i32 = (vga::HEIGHT / CHAR.character_size.height as u64) as i32;

impl VGAAnsiController {
    pub const fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            bg: Color::BLACK,
            fg: Color::WHITE,
            scroll_start: 0,
            scroll_end: LINES - 1,

            inverted_c: false,
            strike: None,
            underline: None,
            bold: false,
            italic: false,
        }
    }

    pub unsafe fn advance(&mut self, data: ansi::Out) {
        match data {
            ansi::Out::Ansi(ansi) => self.handle_ansi(ansi),
            ansi::Out::Data(char) => self.print_char(char as char),
            ansi::Out::None => {}
        }
    }

    unsafe fn print_char(&mut self, char: char) {
        if self.col >= COLS {
            self.col = 0;
            self.line += 1;
            if self.line >= self.scroll_end {
                let bruh = core::slice::from_raw_parts_mut(
                    FRAME_BUF as *mut u8,
                    (WIDTH * HEIGHT) as usize,
                );
                const TMP: usize = WIDTH as usize * CHAR.character_size.height as usize;
                bruh.copy_within(
                    (self.scroll_start as usize + 1) * TMP..((self.scroll_end as usize + 1) * TMP),
                    (self.scroll_start as usize) * TMP,
                );
                bruh[self.scroll_end as usize * TMP..(self.scroll_end as usize + 1) * TMP]
                    .fill(self.bg.0);
                self.line -= 1;
            }
        }

        let display = unsafe { &mut *FRAME_BUF };

        use embedded_graphics::{prelude::*, text::Text};

        let mut character_style = MonoTextStyleBuilder::new()
            .font(if self.bold {
                &CHAR_BOLD
            } else {
                if self.italic {
                    &CHAR_ITALIC
                } else {
                    &CHAR
                }
            })
            .text_color(if self.inverted_c { self.bg } else { self.fg })
            .background_color(if !self.inverted_c { self.bg } else { self.fg });
        if let Some(color) = self.strike {
            character_style = character_style.strikethrough_with_color(color);
        }
        if let Some(color) = self.underline {
            character_style = character_style.underline_with_color(color);
        }
        let character_style = character_style.build();

        let mut buf = [0u8; 4];
        let str = (char).encode_utf8(&mut buf);

        Text::new(
            str,
            Point::new(
                self.col * CHAR.character_size.width as i32,
                self.line * CHAR.character_size.height as i32 + CHAR.baseline as i32,
            ),
            character_style,
        )
        .draw(display)
        .unwrap();

        self.col += 1;
    }

    unsafe fn handle_ansi(&mut self, ansi: ansi::Ansi) {
        if ansi != ansi::Ansi::C0(ansi::C0::SP) {
            // crate::println!("{ansi:#?}");
        }
        match ansi {
            ansi::Ansi::C0(c0) => self.handle_c0(c0),
            ansi::Ansi::C1(c0) => self.handle_c1(c0),
        }
    }

    unsafe fn handle_c0(&mut self, c0: ansi::C0) {
        use embedded_graphics::prelude::*;
        use embedded_graphics::primitives::PrimitiveStyleBuilder;
        use embedded_graphics::primitives::Rectangle;

        let display = unsafe { &mut *FRAME_BUF };

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(if self.inverted_c { self.bg } else { self.fg })
            .fill_color(if !self.inverted_c { self.bg } else { self.fg })
            .stroke_width(0)
            .build();

        match c0 {
            ansi::C0::BS => {
                if self.line == 0 {
                    if self.col > 0 {
                        self.col -= 1;
                    }
                } else if self.col == 0 {
                    self.col = COLS - 1;
                    self.line -= 1;
                } else {
                    self.col -= 1;
                }
            }
            ansi::C0::BEL => {}
            ansi::C0::CR => {
                self.col = 0;
            }
            ansi::C0::SP => {
                Rectangle::new(
                    Point::new(
                        self.col * CHAR.character_size.width as i32,
                        CHAR.character_size.height as i32 * self.line,
                    ),
                    Size::new(CHAR.character_size.width, CHAR.character_size.height),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();

                if self.col >= COLS - 1 {
                    self.col = 0;
                    self.line += 1;
                    if self.line >= self.scroll_end {
                        let bruh = core::slice::from_raw_parts_mut(
                            FRAME_BUF as *mut u8,
                            (WIDTH * HEIGHT) as usize,
                        );
                        const TMP: usize = WIDTH as usize * CHAR.character_size.height as usize;
                        bruh.copy_within(
                            (self.scroll_start as usize + 1) * TMP
                                ..((self.scroll_end as usize + 1) * TMP),
                            (self.scroll_start as usize) * TMP,
                        );
                        bruh[self.scroll_end as usize * TMP..(self.scroll_end as usize + 1) * TMP]
                            .fill(self.bg.0);
                        self.line -= 1;
                    }
                } else {
                    self.col += 1;
                }
            }
            ansi::C0::FF => {}
            ansi::C0::LF => {
                self.col = 0;
                self.line += 1;
                if self.line >= self.scroll_end {
                    let bruh = core::slice::from_raw_parts_mut(
                        FRAME_BUF as *mut u8,
                        (WIDTH * HEIGHT) as usize,
                    );
                    const TMP: usize = WIDTH as usize * CHAR.character_size.height as usize;
                    bruh.copy_within(
                        (self.scroll_start as usize + 1) * TMP
                            ..((self.scroll_end as usize + 1) * TMP),
                        (self.scroll_start as usize) * TMP,
                    );
                    bruh[self.scroll_end as usize * TMP..(self.scroll_end as usize + 1) * TMP]
                        .fill(self.bg.0);
                    self.line -= 1;
                }
            }
            ansi::C0::HT => self.col = (self.col + 3) & !(4 - 1),
            _ => {}
        }
    }

    unsafe fn handle_c1(&mut self, c1: ansi::C1) {
        match c1 {
            ansi::C1::nF(nf) => self.handle_nf(nf),
            ansi::C1::Fp(fp) => self.handle_fp(fp),
            ansi::C1::Fe(fe) => self.handle_fe(fe),
            ansi::C1::Fs(fs) => self.handle_fs(fs),
            ansi::C1::Invalid(u8) => un_print!("Invalid C1: 0x1b 0x{:?}", u8),
        }
    }

    unsafe fn handle_nf(&mut self, nf: ansi::nF) {
        match nf {
            other => un_print!("{other:?}"),
        }
    }

    unsafe fn handle_fp(&mut self, fp: ansi::Fp) {
        match fp {
            other => un_print!("{other:?}"),
        }
    }

    unsafe fn handle_fe(&mut self, fe: ansi::Fe) {
        match fe {
            ansi::Fe::CSI(csi) => self.handle_csi(csi),
            other => un_print!("{other:?}"),
        }
    }

    unsafe fn handle_fs(&mut self, fs: ansi::Fs) {
        match fs {
            other => un_print!("{other:?}"),
        }
    }

    unsafe fn handle_csi(&mut self, csi: ansi::CSI) {
        let display = unsafe { &mut *FRAME_BUF };

        use embedded_graphics::{
            prelude::*, primitives::PrimitiveStyleBuilder, primitives::Rectangle,
        };

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(self.bg)
            .fill_color(self.bg)
            .stroke_width(0)
            .build();

        match csi {
            ansi::CSI::ReportCursorPosition => {
                /// CSI r ; c R
                struct Bleh;
                impl core::fmt::Write for Bleh {
                    fn write_str(&mut self, s: &str) -> core::fmt::Result {
                        uart::print(s);
                        Ok(())
                    }
                }
                use core::fmt::Write;
                write!(Bleh, "\x1b[{};{}R", self.line + 1, self.col + 1).unwrap();
                crate::println!("{};{}R", self.line + 1, self.col + 1)
            }
            ansi::CSI::CursorTo { line, col }
            | ansi::CSI::HorizontalVerticalPosition { line, col } => {
                self.col = (col as i32).min(COLS) - 1;
                self.line = (line as i32).min(LINES) - 1;
            }
            ansi::CSI::InsertLines(lines) => {
                let bruh = core::slice::from_raw_parts_mut(
                    FRAME_BUF as *mut u8,
                    (WIDTH * HEIGHT) as usize,
                );
                bruh.copy_within(
                    (self.scroll_start as usize) * TMP
                        ..(self.scroll_end as usize + 1 - lines as usize) * TMP,
                    (lines as usize + self.scroll_start as usize) * TMP,
                );
                const TMP: usize = WIDTH as usize * CHAR.character_size.height as usize;
                bruh[self.scroll_start as usize * TMP
                    ..(lines as usize + self.scroll_start as usize) * TMP]
                    .fill(self.bg.0);
            }
            ansi::CSI::CursorUp(amount) => self.line -= amount as i32,
            ansi::CSI::CursorDown(amount) => self.line += amount as i32,
            ansi::CSI::CursorLeft(amount) => self.col -= amount as i32,
            ansi::CSI::CursorRight(amount) => self.col += amount as i32,
            ansi::CSI::CursorNextLine(amount) => {
                self.col = 0;
                self.line += amount as i32
            }
            ansi::CSI::CursorPreviousLine(amount) => {
                self.col = 0;
                self.line -= amount as i32
            }
            ansi::CSI::CursorHorizontalAbsolute(col) => self.col = col as i32 - 1,
            ansi::CSI::CursorLineAbsolute(v) => self.line = v as i32 - 1,
            ansi::CSI::EraseDisplay | ansi::CSI::EraseScreen => {
                self.col = 0;
                self.line = 0;
                Rectangle::new(
                    Point::new(0, 0),
                    Size::new(vga::WIDTH as u32, vga::HEIGHT as u32),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            ansi::CSI::EraseFromCursor | ansi::CSI::EraseFromCursorToEndOfLine => {
                Rectangle::new(
                    Point::new(
                        self.col * CHAR.character_size.width as i32,
                        CHAR.character_size.height as i32 * self.line,
                    ),
                    Size::new(vga::WIDTH as u32, CHAR.character_size.height),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            ansi::CSI::EraseToCursor | ansi::CSI::EraseStartOfLineToCursor => {
                Rectangle::new(
                    Point::new(0, CHAR.character_size.height as i32 * self.line),
                    Size::new(
                        self.col as u32 * CHAR.character_size.width,
                        CHAR.character_size.height,
                    ),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            ansi::CSI::EraseLine => {
                Rectangle::new(
                    Point::new(0, CHAR.character_size.height as i32 * self.line),
                    Size::new(vga::WIDTH as u32, CHAR.character_size.height),
                )
                .into_styled(style)
                .draw(display)
                .unwrap();
            }
            ansi::CSI::SetScrollingRegion { top, bottom } => {
                self.scroll_start = top as i32 - 1;
                self.scroll_end = bottom as i32 - 1;
            }
            ansi::CSI::SelectGraphicRendition(g) => {
                for g in g {
                    self.handle_select_graphic(g);
                }
            }
            other => un_print!("{other:?}"),
        }
    }

    unsafe fn handle_select_graphic(&mut self, sg: ansi::SelectGraphic) {
        match sg {
            ansi::SelectGraphic::Reset => {
                self.bg = Color::BLACK;
                self.fg = Color::WHITE;
                self.inverted_c = false;
                self.bold = false;
                self.italic = false;
                self.underline = None;
                self.strike = None;
            }
            ansi::SelectGraphic::Bold => self.bold = true,
            ansi::SelectGraphic::Italic => self.italic = true,
            ansi::SelectGraphic::Underline => self.underline = Some(self.fg),
            ansi::SelectGraphic::InvertFgBg => self.inverted_c = true,
            ansi::SelectGraphic::NotInvertedFgBg => self.inverted_c = false,
            ansi::SelectGraphic::Fg(c) => {
                self.fg = color_code_to_color(c).unwrap_or(Color::WHITE);
            }
            ansi::SelectGraphic::Bg(c) => {
                self.bg = color_code_to_color(c).unwrap_or(Color::BLACK);
            }
            other => un_print!("{other:?}"),
        }
    }
}

const fn color_code_to_color(c: ansi::Color) -> Option<Color> {
    Some(match c {
        ansi::Color::Default => return None,

        ansi::Color::Black | ansi::Color::VGA(0) => Color::BLACK,
        ansi::Color::Red | ansi::Color::VGA(1) => Color::new(170, 0, 0),
        ansi::Color::Green | ansi::Color::VGA(2) => Color::new(0, 170, 0),
        ansi::Color::Yellow | ansi::Color::VGA(3) => Color::new(170, 80, 0),
        ansi::Color::Blue | ansi::Color::VGA(4) => Color::new(0, 0, 170),
        ansi::Color::Magenta | ansi::Color::VGA(5) => Color::new(170, 0, 170),
        ansi::Color::Cyan | ansi::Color::VGA(6) => Color::new(0, 170, 170),
        ansi::Color::White | ansi::Color::VGA(7) => Color::new(192, 192, 192),

        ansi::Color::BrightBlack | ansi::Color::VGA(8) => Color::new(170, 170, 170),
        ansi::Color::BrightRed | ansi::Color::VGA(9) => Color::new(255, 0, 0),
        ansi::Color::BrightGreen | ansi::Color::VGA(10) => Color::new(0, 255, 0),
        ansi::Color::BrightYellow | ansi::Color::VGA(11) => Color::new(255, 255, 0),
        ansi::Color::BrightBlue | ansi::Color::VGA(12) => Color::new(0, 0, 255),
        ansi::Color::BrightMagenta | ansi::Color::VGA(13) => Color::new(255, 0, 170),
        ansi::Color::BrightCyan | ansi::Color::VGA(14) => Color::new(0, 255, 255),
        ansi::Color::BrightWhite | ansi::Color::VGA(15) => Color::WHITE,

        ansi::Color::VGA(v @ 232..=255) => {
            Color::new((v - 232) * 10 + 8, (v - 232) * 10 + 8, (v - 232) * 10 + 8)
        }
        ansi::Color::VGA(v) => {
            let v = v - 16;
            let lookup = [0x0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

            Color::new(
                lookup[(v as usize / (6 * 6)) % 6],
                lookup[(v as usize / (6)) % 6],
                lookup[v as usize % 6],
            )
        }

        ansi::Color::RGB([r, g, b]) => Color::new(r, g, b),
        _ => return None,
    })
}

//------------------- ANSI VGA Controller --------------------------

pub static mut PARSER: ansi::AnsiParser = ansi::AnsiParser::new();
pub static mut CONTROLLER: VGAAnsiController = VGAAnsiController::new();

pub unsafe fn print(data: &[u8]) {
    for byte in data {
        CONTROLLER.advance(PARSER.next(*byte as char));
    }

    vga::flush_frame_virt(FRAME_BUF as usize);
}
