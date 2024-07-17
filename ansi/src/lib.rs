#![no_std]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Out<'a> {
    Ansi(Ansi<'a>),
    Data(u8),
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
    Null = 0,
    StartOfHeating = 1,
    StartOfText = 2,
    EndOfText = 3,
    EndOfTransmission = 4,
    ReturnTerminalStatus = 5,
    Acknowledge = 6,
    Bell = 7,
    Backspace = 8,
    HorizontalTab = 9,
    LineFeed = 10,
    VerticalTab = 11,
    FormFeed = 12,
    CarriageReturn = 13,
    SwitchToAlternateCharacterSet = 14,
    SwitchToStandardCharacterSet = 15,
    DataLinkEscape = 16,
    DeviceControlOne = 17,
    DeviceControlTwo = 18,
    DeviceControlThree = 19,
    DeviceControlFour = 20,
    NegativeAcknowledge = 21,
    SynchronusIdle = 22,
    EndOfTransmissionBlock = 23,
    Cancel = 24,
    EndOfMedium = 25,
    Substitude = 26,
    FileSeparator = 28,
    GroupSeparator = 29,
    RecordSeparator = 30,
    UnitSeparator = 31,
    Space = 32,
    Delete = 127,
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
    Invalid(u8),
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
    /// b'@' Padding Character
    PAD,
    /// b'A' High Octet Preset
    HOP,
    /// b'B' Break Permitted Here
    BPH,
    /// b'C' No Break Here
    NBH,
    /// b'D' Index
    IND,
    /// b'E' Next Line
    NEL,
    /// b'F' Start of Selected Area
    SSA,
    /// b'G' End of Selected Area
    ESA,
    /// b'H' Character Tabulation Set/Horizontal Tabulation Set
    HTS,
    /// b'I' Character Tabulation With Justification/Horizontal Tabulation With Justification
    HTJ,
    /// b'J' Line Tabulation Set/Vertical Tabulation Set
    VTS,
    /// b'K' Partial Line Forward/Partial Line Down
    PLD,
    /// b'L' Partial Line Backward/Partial Line Up
    PLU,
    /// b'M' Reverse Line Feed/Reverse Index
    RI,
    /// b'N' Single-Shift 2
    SS2,
    /// b'O' Single-Shift 3
    SS3,
    /// b'P' Device Control String
    DCS,
    /// b'Q' Private Use 1
    PU1,
    /// b'R' Private Use 2
    PU2,
    /// b'S' Set Transmit State
    STS,
    /// b'T' Cancel character
    CCH,
    /// b'U' Message Waiting
    MW,
    /// b'V' Start of Protected Area
    SPA,
    /// b'W' End of Protected Area
    EPA,
    /// b'X' Start of String
    SOS,
    /// b'Y' Single Graphic Character Introducer
    SGCI,
    /// b'Z' Single Character Introducer
    SCI,
    /// b'[' Control Sequence Introducer [CSI]
    CSI(CSI<'a>),
    /// b'\' String Terminator
    ST,
    /// b']' Operating System Command
    OSC,
    /// b'^' Privacy Message
    PM,
    /// b'_' Application Program Command
    APC,

    DCSData(u8),
    SData(u8),
    PMData(u8),
    APCData(u8),
    OSData(u8),
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
        end: u8
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
    CSI,

    String(StringKind),
    StringEcp(StringKind)
}

enum InvalidKind{
    Ok,
    IntegerOverflow,
    SequenceTooLarge,
}

union Buf<const U16: usize, const U8: usize>{
    nums: [u16; U16],
    buf: [u8; U8]
}

pub struct AnsiParser<const CSI_MAX: usize = 32, const STR_MAX: usize = 64> {
    state: State,
    curr: u8,
    size: u8,
    nums: [u16; CSI_MAX],
    mode: CsiMod,
    invalid: InvalidKind,
    string_extra_byte: Option<u8>,
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
            string_extra_byte: None,
        }
    }

    pub fn next(&mut self, input: u8) -> Out {
        match self.state {
            State::Default => Out::Ansi(Ansi::C0(match input {
                0 => C0::Null,
                1 => C0::StartOfHeating,
                2 => C0::StartOfText,
                3 => C0::EndOfText,
                4 => C0::EndOfTransmission,
                5 => C0::ReturnTerminalStatus,
                6 => C0::Acknowledge,
                7 => C0::Bell,
                8 => C0::Backspace,
                9 => C0::HorizontalTab,
                10 => C0::LineFeed,
                11 => C0::VerticalTab,
                12 => C0::FormFeed,
                13 => C0::CarriageReturn,
                14 => C0::SwitchToAlternateCharacterSet,
                15 => C0::SwitchToStandardCharacterSet,
                16 => C0::DataLinkEscape,
                17 => C0::DeviceControlOne,
                18 => C0::DeviceControlTwo,
                19 => C0::DeviceControlThree,
                20 => C0::DeviceControlFour,
                21 => C0::NegativeAcknowledge,
                22 => C0::SynchronusIdle,
                23 => C0::EndOfTransmissionBlock,
                24 => C0::Cancel,
                25 => C0::EndOfMedium,
                26 => C0::Substitude,
                b'\x1b' => {
                    self.state = State::Escape;
                    self.size = 0;
                    self.curr = 0;
                    self.nums[0] = 0;
                    self.invalid = InvalidKind::Ok;
                    return Out::None;
                }
                28 => C0::FileSeparator,
                29 => C0::GroupSeparator,
                30 => C0::RecordSeparator,
                31 => C0::UnitSeparator,
                32 => C0::Space,
                127 => C0::Delete,
                c => return Out::Data(c),
            })),
            State::Escape => match input {
                c @ 0x20..=0x2F => {
                    // nF
                    self.state = State::Default;
                    match c {
                        b' ' => {}
                        b'!' => {}
                        b'"' => {}
                        b'#' => {}
                        b'$' => {}
                        b'%' => {}
                        b'&' => {}
                        b'\'' => {}
                        b'(' => {}
                        b')' => {}
                        b'*' => {}
                        b'+' => {}
                        b',' => {}
                        b'-' => {}
                        b'.' => {}
                        b'/' => {}
                        _ => unreachable!()
                    }
                    Out::None
                }
                c @ 0x30..=0x3F => {
                    // Fp
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fp(match c {
                        b'0' => Fp::Unknown(c),
                        b'1' => Fp::Unknown(c),
                        b'2' => Fp::Unknown(c),
                        b'3' => Fp::Unknown(c),
                        b'4' => Fp::Unknown(c),
                        b'5' => Fp::Unknown(c),
                        b'6' => Fp::Unknown(c),
                        b'7' => Fp::DECSC,
                        b'8' => Fp::DECRC,
                        b'9' => Fp::DECFI,
                        b':' => Fp::Unknown(c),
                        b';' => Fp::Unknown(c),
                        b'<' => Fp::Unknown(c),
                        b'=' => Fp::DECKPAM,
                        b'>' => Fp::DECKPNM,
                        b'?' => Fp::Unknown(c),
                        _ => unreachable!(),
                    })))
                }
                c @ 0x40..=0x5F => {
                    // Fe
                    self.state = State::Default;
                    match c {
                        b'@' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))),
                        b'A' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HOP))),
                        b'B' => Out::Ansi(Ansi::C1(C1::Fe(Fe::BPH))),
                        b'C' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NBH))),
                        b'D' => Out::Ansi(Ansi::C1(C1::Fe(Fe::IND))),
                        b'E' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NEL))),
                        b'F' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SSA))),
                        b'G' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ESA))),
                        b'H' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTS))),
                        b'I' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTJ))),
                        b'J' => Out::Ansi(Ansi::C1(C1::Fe(Fe::VTS))),
                        b'K' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLD))),
                        b'L' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLU))),
                        b'M' => Out::Ansi(Ansi::C1(C1::Fe(Fe::RI))),
                        b'N' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS2))),
                        b'O' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS3))),
                        b'P' => {
                            self.state = State::String(StringKind::DeviceControl);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::DCS)))
                        },
                        b'Q' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU1))),
                        b'R' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU2))),
                        b'S' => Out::Ansi(Ansi::C1(C1::Fe(Fe::STS))),
                        b'T' => Out::Ansi(Ansi::C1(C1::Fe(Fe::CCH))),
                        b'U' => Out::Ansi(Ansi::C1(C1::Fe(Fe::MW))),
                        b'V' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SPA))),
                        b'W' => Out::Ansi(Ansi::C1(C1::Fe(Fe::EPA))),
                        b'X' => {
                            self.state = State::String(StringKind::Regular);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::SOS)))
                        },
                        b'Y' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SGCI))),
                        b'Z' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SCI))),
                        b'[' => {
                            self.state = State::CsiStart;
                            Out::None
                        }
                        b'\\' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ST))),
                        b']' => {
                            self.state = State::String(StringKind::Os);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::OSC)))
                        },
                        b'^' => {
                            self.state = State::String(StringKind::Privacy);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::PM)))
                        },
                        b'_' => {
                            self.state = State::String(StringKind::ApplicationProgramCommand);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::APC)))
                        },
                        _ => unreachable!(),
                    }
                }
                c @ 0x60..=0x7E => {
                    // Fs
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fs(match c {
                        b'`' => Fs::DMI,
                        b'a' => Fs::INT,
                        b'b' => Fs::EMI,
                        b'c' => Fs::RIS,
                        b'd' => Fs::CMD,
                        b'e' => Fs::Unknown(c),
                        b'f' => Fs::Unknown(c),
                        b'g' => Fs::Unknown(c),
                        b'h' => Fs::Unknown(c),
                        b'i' => Fs::Unknown(c),
                        b'j' => Fs::Unknown(c),
                        b'k' => Fs::Unknown(c),
                        b'l' => Fs::LCKMEM,
                        b'm' => Fs::ULKMEM,
                        b'n' => Fs::LS2,
                        b'o' => Fs::LS3,
                        b'p' => Fs::Unknown(c),
                        b'q' => Fs::Unknown(c),
                        b'r' => Fs::Unknown(c),
                        b's' => Fs::Unknown(c),
                        b't' => Fs::Unknown(c),
                        b'u' => Fs::Unknown(c),
                        b'v' => Fs::Unknown(c),
                        b'w' => Fs::Unknown(c),
                        b'x' => Fs::Unknown(c),
                        b'y' => Fs::Unknown(c),
                        b'z' => Fs::Unknown(c),
                        b'{' => Fs::Unknown(c),
                        b'|' => Fs::LS3R,
                        b'}' => Fs::LS2R,
                        b'~' => Fs::LS1R,
                        _ => unreachable!(),
                    })))
                }
                c => {
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Invalid(c)))
                }
            },
            State::CsiStart => match input {
                b'=' => {
                    self.state = State::CSI;
                    self.mode = CsiMod::Equal;
                    Out::None
                }
                b'?' => {
                    self.state = State::CSI;
                    self.mode = CsiMod::Question;
                    Out::None
                }
                _ => {
                    self.state = State::CSI;
                    self.mode = CsiMod::Standard;
                    return self.next(input);
                }
            },
            State::CSI => match input {
                d @ b'0'..=b'9' => {
                    self.size = self.curr + 1;
                    if let Some(v) = self.nums.get_mut(self.curr as usize) {
                        //TODO test if this overflows
                        *v = *v * 10 + (d - b'0') as u16;
                    } else {
                        self.invalid = InvalidKind::SequenceTooLarge;
                    }
                    Out::None
                }
                b';' => {
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
            State::String(kind) => match input{
                0x1b => {
                    self.state = State::StringEcp(kind);
                    if let Some(previous) = self.string_extra_byte{
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
                c => {
                    let c = if let Some(previous) = self.string_extra_byte{
                        self.string_extra_byte = Some(c);
                        previous
                    }else{
                        c
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
                b'\\' => {
                    self.state = State::Default;
                    Out::Ansi(Ansi::C1(C1::Fe(Fe::ST)))
                }
                c => {
                    self.string_extra_byte = Some(c);
                    match kind{
                        StringKind::DeviceControl => Out::Ansi(Ansi::C1(C1::Fe(Fe::DCSData(0x1b)))),
                        StringKind::Regular => Out::Ansi(Ansi::C1(C1::Fe(Fe::SData(0x1b)))),
                        StringKind::Os => Out::Ansi(Ansi::C1(C1::Fe(Fe::OSData(0x1b)))),
                        StringKind::Privacy => Out::Ansi(Ansi::C1(C1::Fe(Fe::PMData(0x1b)))),
                        StringKind::ApplicationProgramCommand => Out::Ansi(Ansi::C1(C1::Fe(Fe::APCData(0x1b)))),
                    }
                }
            },
        }
    }

    fn parse_csi(&mut self, c: u8) -> Out{
        Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(
            match (self.mode, &self.nums[..self.size as usize], c) {
                (CsiMod::Standard, [line, col], b'f') => {
                    CSI::HorizontalVerticalPosition {
                        line: *line,
                        col: *col,
                    }
                }
                (CsiMod::Standard, [], b'f') => CSI::CursorTo { line: 1, col: 1 },

                (CsiMod::Standard, [line, col], b'H') => CSI::CursorTo {
                    line: *line,
                    col: *col,
                },
                (CsiMod::Standard, [], b'H') => CSI::CursorTo { line: 1, col: 1 },

                (CsiMod::Standard, [amount], b'A') => CSI::CursorUp(*amount),
                (CsiMod::Standard, [], b'A') => CSI::CursorUp(1),

                (CsiMod::Standard, [amount], b'B') => CSI::CursorDown(*amount),
                (CsiMod::Standard, [], b'B') => CSI::CursorDown(1),

                (CsiMod::Standard, [amount], b'C') => CSI::CursorRight(*amount),
                (CsiMod::Standard, [], b'C') => CSI::CursorRight(1),

                (CsiMod::Standard, [amount], b'D') => CSI::CursorLeft(*amount),
                (CsiMod::Standard, [], b'D') => CSI::CursorLeft(1),

                (CsiMod::Standard, [amount], b'E') => CSI::CursorNextLine(*amount),
                (CsiMod::Standard, [], b'E') => CSI::CursorNextLine(1),

                (CsiMod::Standard, [amount], b'F') => CSI::CursorPreviousLine(*amount),
                (CsiMod::Standard, [], b'F') => CSI::CursorPreviousLine(1),

                (CsiMod::Standard, [col], b'G') => CSI::CursorHorizontalAbsolute(*col),
                (CsiMod::Standard, [], b'G') => CSI::CursorHorizontalAbsolute(1),

                (CsiMod::Standard, [line], b'd') => CSI::CursorLineAbsolute(*line),
                (CsiMod::Standard, [], b'd') => CSI::CursorLineAbsolute(1),

                (CsiMod::Standard, [], b'J') => CSI::EraseDisplay,
                (CsiMod::Standard, [0], b'J') => CSI::EraseFromCursor,
                (CsiMod::Standard, [1], b'J') => CSI::EraseToCursor,
                (CsiMod::Standard, [2], b'J') => CSI::EraseScreen,
                (CsiMod::Standard, [3], b'J') => CSI::EraseSavedLines,

                (CsiMod::Standard, [], b'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [0], b'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [1], b'K') => CSI::EraseFromCursorToEndOfLine,
                (CsiMod::Standard, [2], b'K') => CSI::EraseLine,

                (CsiMod::Standard, [], b'L') => CSI::InsertLines(1),
                (CsiMod::Standard, [lines], b'L') => CSI::InsertLines(*lines),

                (CsiMod::Standard, [], b'M') => CSI::DeleteLines(1),
                (CsiMod::Standard, [lines], b'M') => CSI::DeleteLines(*lines),

                (CsiMod::Standard, [5], b'i') => CSI::AuxPortOn,
                (CsiMod::Standard, [4], b'i') => CSI::AuxPortOff,
                (CsiMod::Standard, [5], b'n') => CSI::DeviceStatusReport,
                (CsiMod::Standard, [6], b'n') => CSI::ReportCursorPosition,

                (CsiMod::Standard, [], b's') => CSI::SaveCurrentCursorPosition,
                (CsiMod::Standard, [], b'u') => CSI::RestoreCurrentCursorPosition,

                (CsiMod::Question, [25], b'h') => CSI::ShowCursor,
                (CsiMod::Question, [25], b'l') => CSI::HideCursor,

                (CsiMod::Question, [1004], b'h') => CSI::EnableFocusReporting,
                (CsiMod::Question, [1004], b'l') => CSI::DisableFocusReporting,

                (CsiMod::Question, [47], b'h') => CSI::RestoreScreen,
                (CsiMod::Question, [47], b'l') => CSI::SaveScreen,

                (CsiMod::Question, [1049], b'h') => CSI::EnableAlternativeBuffer,
                (CsiMod::Question, [1049], b'l') => CSI::DisableAlternativeBuffer,

                (CsiMod::Standard, [], b'm') => {
                    CSI::SelectGraphicRendition(GraphicsRendition(&[0]))
                }
                (CsiMod::Standard, gr, b'm') => {
                    CSI::SelectGraphicRendition(GraphicsRendition(gr))
                }

                (CsiMod::Standard, [top, bottom], b'r') => CSI::SetScrollingRegion {
                    top: *top,
                    bottom: *bottom,
                },

                (CsiMod::Question, [0], b'h') => {
                    CSI::ScreenMode(ScreenMode::Monochrome40x25)
                }
                (CsiMod::Question, [1], b'h') => CSI::ScreenMode(ScreenMode::Color40x25),
                (CsiMod::Question, [2], b'h') => {
                    CSI::ScreenMode(ScreenMode::Monochrome80x25)
                }
                (CsiMod::Question, [3], b'h') => CSI::ScreenMode(ScreenMode::Color80x25),
                (CsiMod::Question, [4], b'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics4Color320x200)
                }
                (CsiMod::Question, [5], b'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome320x200)
                }
                (CsiMod::Question, [6], b'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x200)
                }
                (CsiMod::Question, [7], b'h') => {
                    CSI::ScreenMode(ScreenMode::EnableLineWrapping)
                }
                (CsiMod::Question, [13], b'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsColor320x200)
                }
                (CsiMod::Question, [14], b'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x200)
                }
                (CsiMod::Question, [15], b'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome630x350)
                }
                (CsiMod::Question, [16], b'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x350)
                }
                (CsiMod::Question, [17], b'h') => {
                    CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x480)
                }
                (CsiMod::Question, [18], b'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics16Color640x480)
                }
                (CsiMod::Question, [19], b'h') => {
                    CSI::ScreenMode(ScreenMode::Graphics256Color320x200)
                }

                (CsiMod::Question, [0], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Monochrome40x25)
                }
                (CsiMod::Question, [1], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Color40x25)
                }
                (CsiMod::Question, [2], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Monochrome80x25)
                }
                (CsiMod::Question, [3], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Color80x25)
                }
                (CsiMod::Question, [4], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics4Color320x200)
                }
                (CsiMod::Question, [5], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome320x200)
                }
                (CsiMod::Question, [6], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x200)
                }
                (CsiMod::Question, [7], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::EnableLineWrapping)
                }
                (CsiMod::Question, [12], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::StopBlinkingCursor)
                }
                (CsiMod::Question, [13], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsColor320x200)
                }
                (CsiMod::Question, [14], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x200)
                }
                (CsiMod::Question, [15], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome630x350)
                }
                (CsiMod::Question, [16], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x350)
                }
                (CsiMod::Question, [17], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x480)
                }
                (CsiMod::Question, [18], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics16Color640x480)
                }
                (CsiMod::Question, [19], b'l') => {
                    CSI::ResetScreenMode(ScreenMode::Graphics256Color320x200)
                }

                (modifier, sequence, end) => CSI::Unknown { sequence, modifier, end },
            },
        ))))
    }
}
