#![no_std]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Out<'a> {
    Ansi(Ansi<'a>),
    Data(char),
    None,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ansi<'a> {
    /// 0x00-0x32 + 0x7F
    /// Space + Delete included but not technically in this catigory
    C0(C0),
    /// 0x1b
    C1(C1<'a>),
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum C0 {
    /// Null
    NUL = 0,
    /// Start of Heading
    SOH = 1,
    /// Start of Text
    STX = 2,
    /// End of Text
    ETX = 3,
    /// End of Transmission
    EOT = 4,
    /// Enquiry (Terminal status / present)
    ENQ = 5,
    /// Acknowledge
    ACK = 6,
    /// Bell, Alert
    BEL= 7,
    /// Backspace
    BS = 8,
    /// Character/Horizontal Tabulation
    HT = 9,
    /// Line Feed
    LF = 10,
    /// Line/Vertical Tabulation
    VT = 11,
    /// Form Feed
    FF = 12,
    /// Carriage Return
    CR = 13,
    /// Shift Out (Switch to an alternative character set)
    SO = 14,
    /// Shift In (Switch to regular character set)
    SI = 15,
    /// Data Link Escape
    DLE= 16,
    /// Device Control One
    DC1 = 17,
    /// Device Control Two
    DC2 = 18,
    /// Drvice Control Three
    DC3 = 19,
    /// Device Control Four
    DC4 = 20,
    /// Negative Acknowledge
    NAK = 21,
    /// Synchronous Idle
    SYN = 22,
    /// End of Tranmission Block
    ETB = 23,
    /// Cancel
    CAN = 24,
    /// End of Medium
    EM = 25,
    /// Substitude
    SUB = 26,
    /// File Separator
    FS = 28,
    /// Group Separator
    GS = 29,
    /// Record Separator
    RS = 30,
    /// Unit Separator
    US = 31,
    /// Space
    SP = 32,
    /// Delete
    DEL = 127,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// This is a sequence starting with 0x1b
pub enum C1<'a> {
    /// 0x20-0x2F,
    nF(nF),
    /// 0x30-0x3F,
    Fp(Fp),
    /// 0x40-0x5F,
    Fe(Fe<'a>),
    /// 0x60-0x7E,
    Fs(Fs),
    /// Not nF, Fp, Fe, Fs
    Invalid(char),
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// A sequence starting with 0x1b with a character in the range 0x20-0x2F following
pub enum nF {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A sequence starting with 0x1b with a character in the range 0x30-0x3F following
pub enum Fp {
    /// Back Index
    DECFI = b'6',
    /// Save Cursor
    DECSC = b'7',
    /// Restore Cursor
    DECRC = b'8',
    /// Forward Index
    DECKPAM = b'=',
    /// Application Keypad
    DECKPNM = b'>',
    /// Normal Keypad
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]

/// A sequence starting with 0x1b with a character in the range 0x40-0x5F following
pub enum Fe<'a> {
    /// '@' Padding Character
    PAD,
    /// 'A' High Octet Preset
    HOP,
    /// '' Break Permitted Here
    BPH,
    /// 'C' No Break Here
    NBH,
    /// 'D' Index
    IND,
    /// 'E' Next Line
    NEL,
    /// 'F' Start of Selected Area
    SSA,
    /// 'G' End of Selected Area
    ESA,
    /// 'H' Character Tabulation Set/Horizontal Tabulation Set
    HTS,
    /// 'I' Character Tabulation With Justification/Horizontal Tabulation With Justification
    HTJ,
    /// 'J' Line Tabulation Set/Vertical Tabulation Set
    VTS,
    /// 'K' Partial Line Forward/Partial Line Down
    PLD,
    /// 'L' Partial Line Backward/Partial Line Up
    PLU,
    /// 'M' Reverse Line Feed/Reverse Index
    RI,
    /// 'N' Single-Shift 2
    SS2,
    /// 'O' Single-Shift 3
    SS3,
    /// 'P' Device Control String
    DCS,
    /// 'Q' Private Use 1
    PU1,
    /// 'R' Private Use 2
    PU2,
    /// 'S' Set Transmit State
    STS,
    /// 'T' Cancel character
    CCH,
    /// 'U' Message Waiting
    MW,
    /// 'V' Start of Protected Area
    SPA,
    /// 'W' End of Protected Area
    EPA,
    /// 'X' Start of String
    SOS,
    /// 'Y' Single Graphic Character Introducer
    SGCI,
    /// 'Z' Single Character Introducer
    SCI,
    /// '[' Control Sequence Introducer [CSI]
    CSI(CSI<'a>),
    /// '\' String Terminator
    ST,
    /// ']' Operating System Command
    OSC,
    /// '^' Privacy Message
    PM,
    /// '_' Application Program Command
    APC,

    DCSData(char),
    SData(char),
    PMData(char),
    APCData(char),
    OSData(char),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A sequence starting with 0x1b with a character in the range 0x60-0x7E following
pub enum Fs {
    /// Disable manual input
    DMI = b'`',
    /// Interrupt
    INT = b'a',
    /// Enable manual input
    EMI = b'b',
    /// Reset to initial state
    RIS = b'c',
    /// Coding method delimiter
    CMD = b'd',

    /// Memory Lock (Locks memory above the cursor)
    LCKMEM = b'l',
    /// Memory Unlock
    ULKMEM = b'm',

    /// Locking shift two
    LS2 = b'n',
    /// Locking shift three
    LS3 = b'o',
    /// Locking shift three right
    LS3R = b'|',
    /// Locking shift two right
    LS2R = b'}',
    /// Locking shift one right
    LS1R = b'~',
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CSI<'a> {
    CursorUp(u16),
    CursorDown(u16),
    CursorLeft(u16),
    CursorRight(u16),

    CursorNextLine(u16),
    CursorPreviousLine(u16),

    CursorHorizontalAbsolute(u16),
    CursorTo {
        line: u16,
        col: u16,
    },
    HorizontalVerticalPosition {
        line: u16,
        col: u16,
    },
    CursorPosition,

    EraseDisplay,
    EraseFromCursor,
    EraseToCursor,
    EraseScreen,

    EraseSavedLines,
    EraseFromCursorToEndOfLine,
    EraseStartOfLineToCursor,
    EraseLine,

    ScrollUp(u16),
    ScrollDown(u16),
    AuxPortOn,
    AuxPortOff,
    DeviceStatusReport,
    SelectGraphicRendition(GraphicsRendition<'a>),

    SaveCurrentCursorPosition,
    RestoreCurrentCursorPosition,
    ShowCursor,
    HideCursor,

    EnableFocusReporting,
    DisableFocusReporting,

    RestoreScreen,
    SaveScreen,

    EnableAlternativeBuffer,
    DisableAlternativeBuffer,
    ScreenMode(ScreenMode),
    ResetScreenMode(ScreenMode),
    SetScrollingRegion {
        top: u16,
        bottom: u16,
    },
    DeleteLines(u16),
    InsertLines(u16),
    /// CSI r ; c R
    ReportCursorPosition,
    CursorLineAbsolute(u16),

    SequenceTooLarge,
    IntegerOverflow,
    Unknown{
        sequence: &'a[u16], 
        modifier: CsiMod,
        end: char
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Default,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    VGA(u8),
    RGB([u8; 3]),

    NotPresent,
    Invalid(u16),
    LongNotPresnet,
    InvalidLong(u16),
    MalformedVGA,
    MalformedRGB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ScreenMode {
    Monochrome40x25 = 0,
    Color40x25 = 1,
    Monochrome80x25 = 2,
    Color80x25 = 3,
    Graphics4Color320x200 = 4,
    GraphicsMonochrome320x200 = 5,
    GraphicsMonochrome640x200 = 6,
    EnableLineWrapping = 7,
    StopBlinkingCursor = 12,
    GraphicsColor320x200 = 13,
    Graphics16Color640x200 = 14,
    GraphicsMonochrome630x350 = 15,
    Graphics16Color640x350 = 16,
    GraphicsMonochrome640x480 = 17,
    Graphics16Color640x480 = 18,
    Graphics256Color320x200 = 19,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SelectGraphic {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    InvertFgBg,
    Conceal,
    CrossedOut,
    PrimaryFont,
    AlternativeFont(u8),
    Fraktur,
    DoublyUnderlined,
    NormalIntensity,
    NeitherItalicNorBackletter,
    NotUnderlined,
    NotBlinking,
    ProportionalSpacing,
    NotInvertedFgBg,
    Reveal,
    NotCrossedOut,
    Fg(Color),
    Bg(Color),
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    UnderlineColor(Color),
    IdeogramUnderline,
    IdeogramDoubleUnderline,
    IdeogramOverline,
    IdeogramStressMarking,
    IdeogramAttributes,
    Superscript,
    Subscript,
    NeitherSuperscriptNorSubScript,

    Unknown(u16),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GraphicsRendition<'a>(&'a [u16]);

impl<'a> core::fmt::Debug for GraphicsRendition<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a> GraphicsRendition<'a> {
    fn parse_color(
        &mut self,
        val: u16,
        default: Option<u16>,
        reg_start: Option<u16>,
        bright_start: Option<u16>,
        long: Option<u16>,
    ) -> Color {
        if Some(val) == default {
            return Color::Default;
        }
        if Some(val) == long {
            let Some((long, rest)) = self.0.split_first() else {
                return Color::LongNotPresnet;
            };
            self.0 = rest;
            match *long {
                2 => {
                    let Some((rgb, rest)) = self.0.split_first_chunk::<3>() else {
                        return Color::MalformedRGB;
                    };
                    self.0 = rest;
                    if let (Ok(r), Ok(g), Ok(b)) =
                        (rgb[0].try_into(), rgb[1].try_into(), rgb[2].try_into())
                    {
                        return Color::RGB([r, g, b]);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                5 => {
                    let Some((vga, rest)) = self.0.split_first() else {
                        return Color::MalformedVGA;
                    };
                    self.0 = rest;
                    if let Ok(vga) = (*vga).try_into() {
                        return Color::VGA(vga);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                other => return Color::InvalidLong(other),
            }
        }
        if let Some(start) = reg_start {
            match val.wrapping_sub(start) {
                0 => return Color::Black,
                1 => return Color::Red,
                2 => return Color::Green,
                3 => return Color::Yellow,
                4 => return Color::Blue,
                5 => return Color::Magenta,
                6 => return Color::Cyan,
                7 => return Color::Black,
                _ => {}
            }
        }
        if let Some(start) = bright_start {
            match val.wrapping_sub(start) {
                0 => return Color::BrightBlack,
                1 => return Color::BrightRed,
                2 => return Color::BrightGreen,
                3 => return Color::BrightYellow,
                4 => return Color::BrightBlue,
                5 => return Color::BrightMagenta,
                6 => return Color::BrightCyan,
                7 => return Color::BrightBlack,
                _ => {}
            }
        }
        Color::Invalid(val)
    }
}

impl<'a> Iterator for GraphicsRendition<'a> {
    type Item = SelectGraphic;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.0.split_first()?;
        self.0 = rest;
        match *first {
            0 => Some(SelectGraphic::Reset),
            1 => Some(SelectGraphic::Bold),
            2 => Some(SelectGraphic::Faint),
            3 => Some(SelectGraphic::Italic),
            4 => Some(SelectGraphic::Underline),
            5 => Some(SelectGraphic::SlowBlink),
            6 => Some(SelectGraphic::RapidBlink),
            7 => Some(SelectGraphic::InvertFgBg),
            8 => Some(SelectGraphic::Conceal),
            9 => Some(SelectGraphic::CrossedOut),
            10 => Some(SelectGraphic::PrimaryFont),
            f @ 11..=19 => Some(SelectGraphic::AlternativeFont(f as u8 - 11)),
            20 => Some(SelectGraphic::Fraktur),
            21 => Some(SelectGraphic::DoublyUnderlined),
            22 => Some(SelectGraphic::NormalIntensity),
            23 => Some(SelectGraphic::NeitherItalicNorBackletter),
            24 => Some(SelectGraphic::NotUnderlined),
            25 => Some(SelectGraphic::NotBlinking),
            26 => Some(SelectGraphic::ProportionalSpacing),
            27 => Some(SelectGraphic::NotInvertedFgBg),
            28 => Some(SelectGraphic::Reveal),
            29 => Some(SelectGraphic::NotCrossedOut),
            c @ (30..=39 | 90..=97) => Some(SelectGraphic::Fg(self.parse_color(
                c,
                Some(39),
                Some(30),
                Some(90),
                Some(38),
            ))),
            c @ (40..=49 | 100..=107) => Some(SelectGraphic::Fg(self.parse_color(
                c,
                Some(49),
                Some(40),
                Some(100),
                Some(48),
            ))),
            50 => Some(SelectGraphic::DisableProportionalSpacing),
            51 => Some(SelectGraphic::Framed),
            52 => Some(SelectGraphic::Encircled),
            53 => Some(SelectGraphic::Overlined),
            54 => Some(SelectGraphic::NeitherFramedNorEncircled),
            55 => Some(SelectGraphic::NotOverlined),
            c @ 58..=59 => Some(SelectGraphic::Fg(self.parse_color(
                c,
                Some(59),
                None,
                None,
                Some(58),
            ))),
            60 => Some(SelectGraphic::IdeogramUnderline),
            61 => Some(SelectGraphic::IdeogramDoubleUnderline),
            62 => Some(SelectGraphic::IdeogramOverline),
            63 => Some(SelectGraphic::IdeogramDoubleUnderline),
            64 => Some(SelectGraphic::IdeogramStressMarking),
            65 => Some(SelectGraphic::IdeogramAttributes),
            73 => Some(SelectGraphic::Superscript),
            74 => Some(SelectGraphic::Subscript),
            75 => Some(SelectGraphic::NeitherSuperscriptNorSubScript),

            _ => Some(SelectGraphic::Unknown(*first)),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CsiMod {
    #[default]
    Standard,
    Equal,
    Question,
    Unknown(u8)
}

#[derive(Clone, Copy)]
enum StringKind{
    DeviceControl,
    Regular,
    Privacy,
    ApplicationProgramCommand,
    Os,
}

#[derive(Default, Clone, Copy)]
enum State {
    #[default]
    Default,
    Escape,
    CsiStart,
    Csi,

    String(StringKind),
    StringEcp(StringKind)
}

enum InvalidKind{
    Ok,
    IntegerOverflow,
    SequenceTooLarge,
}

pub struct AnsiParser<const CSI_MAX: usize = 32> {
    state: State,
    curr: u8,
    size: u8,
    nums: [u16; CSI_MAX],
    mode: CsiMod,
    invalid: InvalidKind,
    string_extra_char: Option<char>,
}

impl<const CSI_MAX: usize> core::default::Default for AnsiParser<CSI_MAX>{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> AnsiParser<N> {
    pub const fn new() -> Self {
        Self {
            state: State::Default,
            curr: 0,
            size: 0,
            nums: [0; N],
            mode: CsiMod::Standard,
            invalid: InvalidKind::Ok,
            string_extra_char: None,
        }
    }

    pub fn next(&mut self, input: char) -> Out {
        match self.state {
            State::Default => Out::Ansi(Ansi::C0(match input as u32 {
                0 => C0::NUL,
                1 => C0::SOH,
                2 => C0::STX,
                3 => C0::ETX,
                4 => C0::EOT,
                5 => C0::ENQ,
                6 => C0::ACK,
                7 => C0::BEL,
                8 => C0::BS,
                9 => C0::HT,
                10 => C0::LF,
                11 => C0::VT,
                12 => C0::FF,
                13 => C0::CR,
                14 => C0::SO,
                15 => C0::SI,
                16 => C0::DLE,
                17 => C0::DC1,
                18 => C0::DC2,
                19 => C0::DC3,
                20 => C0::DC4,
                21 => C0::NAK,
                22 => C0::SI,
                23 => C0::ETB,
                24 => C0::CAN,
                25 => C0::EM,
                26 => C0::SUB,
                0x1b => {
                    self.state = State::Escape;
                    self.size = 0;
                    self.curr = 0;
                    self.nums[0] = 0;
                    self.invalid = InvalidKind::Ok;
                    return Out::None;
                }
                28 => C0::FS,
                29 => C0::GS,
                30 => C0::RS,
                31 => C0::US,
                32 => C0::SP,
                127 => C0::DEL,
                _ => return Out::Data(input),
            })),
            State::Escape => match input as u32 {
                0x20..=0x2F => {
                    // nF
                    self.state = State::Default;
                    match input {
                        ' ' => {}
                        '!' => {}
                        '"' => {}
                        '#' => {}
                        '$' => {}
                        '%' => {}
                        '&' => {}
                        '\'' => {}
                        '(' => {}
                        ')' => {}
                        '*' => {}
                        '+' => {}
                        ',' => {}
                        '-' => {}
                        '.' => {}
                        '/' => {}
                        _ => unreachable!()
                    }
                    Out::None
                }
                0x30..=0x3F => {
                    // Fp
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fp(match input {
                        '0' => Fp::Unknown(input as u8),
                        '1' => Fp::Unknown(input as u8),
                        '2' => Fp::Unknown(input as u8),
                        '3' => Fp::Unknown(input as u8),
                        '4' => Fp::Unknown(input as u8),
                        '5' => Fp::Unknown(input as u8),
                        '6' => Fp::Unknown(input as u8),
                        '7' => Fp::DECSC,
                        '8' => Fp::DECRC,
                        '9' => Fp::DECFI,
                        ':' => Fp::Unknown(input as u8),
                        ';' => Fp::Unknown(input as u8),
                        '<' => Fp::Unknown(input as u8),
                        '=' => Fp::DECKPAM,
                        '>' => Fp::DECKPNM,
                        '?' => Fp::Unknown(input as u8),
                        _ => unreachable!(),
                    })))
                }
                0x40..=0x5F => {
                    // Fe
                    self.state = State::Default;
                    match input {
                        '@' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))),
                        'A' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HOP))),
                        'B' => Out::Ansi(Ansi::C1(C1::Fe(Fe::BPH))),
                        'C' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NBH))),
                        'D' => Out::Ansi(Ansi::C1(C1::Fe(Fe::IND))),
                        'E' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NEL))),
                        'F' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SSA))),
                        'G' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ESA))),
                        'H' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTS))),
                        'I' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTJ))),
                        'J' => Out::Ansi(Ansi::C1(C1::Fe(Fe::VTS))),
                        'K' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLD))),
                        'L' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLU))),
                        'M' => Out::Ansi(Ansi::C1(C1::Fe(Fe::RI))),
                        'N' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS2))),
                        'O' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS3))),
                        'P' => {
                            self.state = State::String(StringKind::DeviceControl);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::DCS)))
                        },
                        'Q' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU1))),
                        'R' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU2))),
                        'S' => Out::Ansi(Ansi::C1(C1::Fe(Fe::STS))),
                        'T' => Out::Ansi(Ansi::C1(C1::Fe(Fe::CCH))),
                        'U' => Out::Ansi(Ansi::C1(C1::Fe(Fe::MW))),
                        'V' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SPA))),
                        'W' => Out::Ansi(Ansi::C1(C1::Fe(Fe::EPA))),
                        'X' => {
                            self.state = State::String(StringKind::Regular);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::SOS)))
                        },
                        'Y' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SGCI))),
                        'Z' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SCI))),
                        '[' => {
                            self.state = State::CsiStart;
                            Out::None
                        }
                        '\\' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ST))),
                        ']' => {
                            self.state = State::String(StringKind::Os);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::OSC)))
                        },
                        '^' => {
                            self.state = State::String(StringKind::Privacy);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::PM)))
                        },
                        '_' => {
                            self.state = State::String(StringKind::ApplicationProgramCommand);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::APC)))
                        },
                        _ => unreachable!(),
                    }
                }
                0x60..=0x7E => {
                    // Fs
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fs(match input {
                        '`' => Fs::DMI,
                        'a' => Fs::INT,
                        'b' => Fs::EMI,
                        'c' => Fs::RIS,
                        'd' => Fs::CMD,
                        'e' => Fs::Unknown(input as u8),
                        'f' => Fs::Unknown(input as u8),
                        'g' => Fs::Unknown(input as u8),
                        'h' => Fs::Unknown(input as u8),
                        'i' => Fs::Unknown(input as u8),
                        'j' => Fs::Unknown(input as u8),
                        'k' => Fs::Unknown(input as u8),
                        'l' => Fs::LCKMEM,
                        'm' => Fs::ULKMEM,
                        'n' => Fs::LS2,
                        'o' => Fs::LS3,
                        'p' => Fs::Unknown(input as u8),
                        'q' => Fs::Unknown(input as u8),
                        'r' => Fs::Unknown(input as u8),
                        's' => Fs::Unknown(input as u8),
                        't' => Fs::Unknown(input as u8),
                        'u' => Fs::Unknown(input as u8),
                        'v' => Fs::Unknown(input as u8),
                        'w' => Fs::Unknown(input as u8),
                        'x' => Fs::Unknown(input as u8),
                        'y' => Fs::Unknown(input as u8),
                        'z' => Fs::Unknown(input as u8),
                        '{' => Fs::Unknown(input as u8),
                        '|' => Fs::LS3R,
                        '}' => Fs::LS2R,
                        '~' => Fs::LS1R,
                        _ => unreachable!(),
                    })))
                }
                _ => {
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Invalid(input)))
                }
            },
            State::CsiStart => match input {
                '=' => {
                    self.state = State::Csi;
                    self.mode = CsiMod::Equal;
                    Out::None
                }
                '?' => {
                    self.state = State::Csi;
                    self.mode = CsiMod::Question;
                    Out::None
                }
                _ => {
                    self.state = State::Csi;
                    self.mode = CsiMod::Standard;
                    return self.next(input);
                }
            },
            State::Csi => match input {
                d @ '0'..='9' => {
                    self.size = self.curr + 1;
                    if let Some(v) = self.nums.get_mut(self.curr as usize) {
                        //TODO test if this overflows
                        
                        if let Some(nv) = v.checked_mul(10).and_then(|v|v.checked_add((d as u8 - b'0') as u16)){
                            *v = nv;
                        }else{
                            self.invalid = InvalidKind::IntegerOverflow;
                        }
                        // *v = *v * 10 + (d as u8 - b'0') as u16;
                    } else {
                        self.invalid = InvalidKind::SequenceTooLarge;
                    }
                    Out::None
                }
                ';' => {
                    if let Some(next) = self.curr.checked_add(1) {
                        self.curr = next;
                        if let Some(next) = self.nums.get_mut(self.curr as usize){
                            *next = 0;
                        }
                    } else {
                        self.invalid = InvalidKind::SequenceTooLarge;
                    }
                    Out::None
                }
                c => {
                    self.state = State::Default;
                    match self.invalid{
                        InvalidKind::Ok => self.parse_csi(c),
                        InvalidKind::IntegerOverflow => Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::IntegerOverflow)))),
                        InvalidKind::SequenceTooLarge => Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::SequenceTooLarge)))),
                    }
                }
            },
            State::String(kind) => match input as u32{
                0x1b => {
                    self.state = State::StringEcp(kind);
                    if let Some(previous) = self.string_extra_char{
                        match kind{
                            StringKind::DeviceControl => Out::Ansi(Ansi::C1(C1::Fe(Fe::DCSData(previous)))),
                            StringKind::Regular => Out::Ansi(Ansi::C1(C1::Fe(Fe::SData(previous)))),
                            StringKind::Os => Out::Ansi(Ansi::C1(C1::Fe(Fe::OSData(previous)))),
                            StringKind::Privacy => Out::Ansi(Ansi::C1(C1::Fe(Fe::PMData(previous)))),
                            StringKind::ApplicationProgramCommand => Out::Ansi(Ansi::C1(C1::Fe(Fe::APCData(previous)))),
                        }
                    }else{
                        Out::None
                    }
                }
                _ => {
                    let c = if let Some(previous) = self.string_extra_char{
                        self.string_extra_char = Some(input);
                        previous
                    }else{
                        input
                    };
                    match kind{
                        StringKind::DeviceControl => Out::Ansi(Ansi::C1(C1::Fe(Fe::DCSData(c)))),
                        StringKind::Regular => Out::Ansi(Ansi::C1(C1::Fe(Fe::SData(c)))),
                        StringKind::Os => Out::Ansi(Ansi::C1(C1::Fe(Fe::OSData(c)))),
                        StringKind::Privacy => Out::Ansi(Ansi::C1(C1::Fe(Fe::PMData(c)))),
                        StringKind::ApplicationProgramCommand => Out::Ansi(Ansi::C1(C1::Fe(Fe::APCData(c)))),
                    }
                }
            },
            State::StringEcp(kind) => match input{
                '\\' => {
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fe(Fe::ST)))
                }
                c => {
                    self.string_extra_char = Some(c);
                    match kind{
                        StringKind::DeviceControl => Out::Ansi(Ansi::C1(C1::Fe(Fe::DCSData(0x1b as char)))),
                        StringKind::Regular => Out::Ansi(Ansi::C1(C1::Fe(Fe::SData(0x1b as char)))),
                        StringKind::Os => Out::Ansi(Ansi::C1(C1::Fe(Fe::OSData(0x1b as char)))),
                        StringKind::Privacy => Out::Ansi(Ansi::C1(C1::Fe(Fe::PMData(0x1b as char)))),
                        StringKind::ApplicationProgramCommand => Out::Ansi(Ansi::C1(C1::Fe(Fe::APCData(0x1b as char)))),
                    }
                }
            },
        }
    }

    fn parse_csi(&mut self, c: char) -> Out{
        Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(
            match (self.mode, &self.nums[..self.size as usize], c) {
                (CsiMod::Standard, [line, col], 'f') => {
                    CSI::HorizontalVerticalPosition {
                        line: *line,
                        col: *col,
                    }
                }
                (CsiMod::Standard, [], 'f') => CSI::CursorTo { line: 1, col: 1 },

                (CsiMod::Standard, [line, col], 'H') => CSI::CursorTo {
                    line: *line,
                    col: *col,
                },
                (CsiMod::Standard, [], 'H') => CSI::CursorTo { line: 1, col: 1 },

                (CsiMod::Standard, [amount], 'A') => CSI::CursorUp(*amount),
                (CsiMod::Standard, [], 'A') => CSI::CursorUp(1),

                (CsiMod::Standard, [amount], 'B') => CSI::CursorDown(*amount),
                (CsiMod::Standard, [], 'B') => CSI::CursorDown(1),

                (CsiMod::Standard, [amount], 'C') => CSI::CursorRight(*amount),
                (CsiMod::Standard, [], 'C') => CSI::CursorRight(1),

                (CsiMod::Standard, [amount], 'D') => CSI::CursorLeft(*amount),
                (CsiMod::Standard, [], 'D') => CSI::CursorLeft(1),

                (CsiMod::Standard, [amount], 'E') => CSI::CursorNextLine(*amount),
                (CsiMod::Standard, [], 'E') => CSI::CursorNextLine(1),

                (CsiMod::Standard, [amount], 'F') => CSI::CursorPreviousLine(*amount),
                (CsiMod::Standard, [], 'F') => CSI::CursorPreviousLine(1),

                (CsiMod::Standard, [col], 'G') => CSI::CursorHorizontalAbsolute(*col),
                (CsiMod::Standard, [], 'G') => CSI::CursorHorizontalAbsolute(1),

                (CsiMod::Standard, [line], 'd') => CSI::CursorLineAbsolute(*line),
                (CsiMod::Standard, [], 'd') => CSI::CursorLineAbsolute(1),

                (CsiMod::Standard, [], 'J') => CSI::EraseDisplay,
                (CsiMod::Standard, [0], 'J') => CSI::EraseFromCursor,
                (CsiMod::Standard, [1], 'J') => CSI::EraseToCursor,
                (CsiMod::Standard, [2], 'J') => CSI::EraseScreen,
                (CsiMod::Standard, [3], 'J') => CSI::EraseSavedLines,

                (CsiMod::Standard, [], 'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [0], 'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [1], 'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [2], 'K') => CSI::EraseLine,

                (CsiMod::Standard, [], 'L') => CSI::InsertLines(1),
                (CsiMod::Standard, [lines], 'L') => CSI::InsertLines(*lines),

                (CsiMod::Standard, [], 'M') => CSI::DeleteLines(1),
                (CsiMod::Standard, [lines], 'M') => CSI::DeleteLines(*lines),

                (CsiMod::Standard, [5], 'i') => CSI::AuxPortOn,
                (CsiMod::Standard, [4], 'i') => CSI::AuxPortOff,
                (CsiMod::Standard, [5], 'n') => CSI::DeviceStatusReport,
                (CsiMod::Standard, [6], 'n') => CSI::ReportCursorPosition,

                (CsiMod::Standard, [], 's') => CSI::SaveCurrentCursorPosition,
                (CsiMod::Standard, [], 'u') => CSI::RestoreCurrentCursorPosition,

                (CsiMod::Question, [25], 'h') => CSI::ShowCursor,
                (CsiMod::Question, [25], 'l') => CSI::HideCursor,

                (CsiMod::Question, [1004], 'h') => CSI::EnableFocusReporting,
                (CsiMod::Question, [1004], 'l') => CSI::DisableFocusReporting,

                (CsiMod::Question, [47], 'h') => CSI::RestoreScreen,
                (CsiMod::Question, [47], 'l') => CSI::SaveScreen,

                (CsiMod::Question, [1049], 'h') => CSI::EnableAlternativeBuffer,
                (CsiMod::Question, [1049], 'l') => CSI::DisableAlternativeBuffer,

                (CsiMod::Standard, [], 'm') => {
                    CSI::SelectGraphicRendition(GraphicsRendition(&[0]))
                }
                (CsiMod::Standard, gr, 'm') => {
                    CSI::SelectGraphicRendition(GraphicsRendition(gr))
                }

                (CsiMod::Standard, [top, bottom], 'r') => CSI::SetScrollingRegion {
                    top: *top,
                    bottom: *bottom,
                },

                (CsiMod::Question, [0], 'h') => {
                    CSI::ScreenMode(ScreenMode::Monochrome40x25)
                }
                (CsiMod::Question, [1], 'h') => CSI::ScreenMode(ScreenMode::Color40x25),
                (CsiMod::Question, [2], 'h') => {
                    CSI::ScreenMode(ScreenMode::Monochrome80x25)
                }
                (CsiMod::Question, [3], 'h') => CSI::ScreenMode(ScreenMode::Color80x25),
                (CsiMod::Question, [4], 'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics4Color320x200)
                }
                (CsiMod::Question, [5], 'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome320x200)
                }
                (CsiMod::Question, [6], 'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x200)
                }
                (CsiMod::Question, [7], 'h') => {
                    CSI::ScreenMode(ScreenMode::EnableLineWrapping)
                }
                (CsiMod::Question, [13], 'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsColor320x200)
                }
                (CsiMod::Question, [14], 'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x200)
                }
                (CsiMod::Question, [15], 'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome630x350)
                }
                (CsiMod::Question, [16], 'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x350)
                }
                (CsiMod::Question, [17], 'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x480)
                }
                (CsiMod::Question, [18], 'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x480)
                }
                (CsiMod::Question, [19], 'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics256Color320x200)
                }

                (CsiMod::Question, [0], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Monochrome40x25)
                }
                (CsiMod::Question, [1], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Color40x25)
                }
                (CsiMod::Question, [2], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Monochrome80x25)
                }
                (CsiMod::Question, [3], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Color80x25)
                }
                (CsiMod::Question, [4], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics4Color320x200)
                }
                (CsiMod::Question, [5], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome320x200)
                }
                (CsiMod::Question, [6], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x200)
                }
                (CsiMod::Question, [7], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::EnableLineWrapping)
                }
                (CsiMod::Question, [12], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::StopBlinkingCursor)
                }
                (CsiMod::Question, [13], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsColor320x200)
                }
                (CsiMod::Question, [14], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x200)
                }
                (CsiMod::Question, [15], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome630x350)
                }
                (CsiMod::Question, [16], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x350)
                }
                (CsiMod::Question, [17], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x480)
                }
                (CsiMod::Question, [18], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x480)
                }
                (CsiMod::Question, [19], 'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics256Color320x200)
                }

                (modifier, sequence, end) => CSI::Unknown { sequence, modifier, end },
            },
        ))))
    }
}