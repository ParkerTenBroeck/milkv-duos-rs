#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ansi{
    CursorHome,
    CursorTo{line: u16, col: u16},
    CursorUp(u16),
    CursorDown(u16),
    CursorLeft(u16),
    CursorRight(u16),

    CursorDownBeginning(u16),
    CursorUpBeginning(u16),

    CursorToCol(u16),
    CursorUpScroll,

    EraseDisplay,
    EraseFromCursor,
    EraseToCursor,
    EraseScreen,
    EraseSavedLines,
    EraseInLine,
    EraseFromCursorToEndOfLine,
    EraseStartOfLineToCursor,
    EraseLine,
    Invalid,
    ResetAllGraphics,
    SetBold,
    ResetBoldDim,
    SetDim,
    SetItalic,
    ResetItalic,
    SetUnderline,
    ResetUnderline,
    SetBlinking,
    ResetBlinking,
    SetInverse,
    ResetInverse,
    SetHidden,
    ResetHidden,
    SetStrikethrough,
    ResetStrikethrough,
}


#[derive(Default, Clone, Copy)]
enum Mode{
    #[default]
    Default,
    Equal,
    Question,
}

#[derive(Default, Clone, Copy)]
enum State{
    #[default]
    Default,
    Escape,
    Brack,

    Reg,
    Graphics,

    Space,
}

#[derive(Default)]
pub struct AnsiParser2{
    state: State,
    curr: u8,
    size: u8,
    nums: [u16; 5],
    mode: Mode
}

pub enum Out{
    Ansi(Ansi),
    Data(u8),
    None
}

impl AnsiParser2{
    pub const fn new() -> Self {
        Self { state: State::Default, curr: 0, size: 0, nums: [0; 5], mode: Mode::Default }
    }

    pub fn next(&mut self, input: u8) -> Out{
        loop{
            return match (self.state, input){
                (State::Default, b'\x1b') => {
                    self.state = State::Escape;
                    self.size = 0;
                    self.curr = 0;
                    self.nums = [0; 5];
                    Out::None
                }
                (State::Default, c) => Out::Data(c),
                (State::Escape, b'[') => {
                    self.state = State::Brack;
                    Out::None
                }

                (State::Brack, b'=') => {
                    self.state = State::Reg;
                    self.mode = Mode::Equal;
                    Out::None
                }
                (State::Brack, b'?') => {
                    self.state = State::Reg;
                    self.mode = Mode::Question;
                    Out::None
                }
                (State::Brack, _) => {
                    self.state = State::Reg;
                    self.mode = Mode::Default;
                    continue;
                }

                (State::Reg, d @ b'0'..=b'9') => {
                    self.size = self.curr + 1;
                    self.nums[self.curr as usize] = self.nums[self.curr as usize] * 10 + (d - b'0') as u16;
                    Out::None
                }
                (State::Reg, b';') => {
                    self.curr += 1;
                    Out::None
                }
                (State::Reg, c) => {
                    self.state = State::Default;
                    Out::Ansi(match (self.mode, &self.nums[..self.size as usize], c){

                        (Mode::Default, [], b'H') => Ansi::CursorHome,
                        (Mode::Default, [line, col], b'H' | b'f') => Ansi::CursorTo { line: *line, col: *col },
                        (Mode::Default, [amount], b'A') => Ansi::CursorUp(*amount),
                        (Mode::Default, [amount], b'B') => Ansi::CursorDown(*amount),
                        (Mode::Default, [amount], b'C') => Ansi::CursorRight(*amount),
                        (Mode::Default, [amount], b'D') => Ansi::CursorLeft(*amount),
                        (Mode::Default, [amount], b'E') => Ansi::CursorDownBeginning(*amount),
                        (Mode::Default, [amount], b'F') => Ansi::CursorUpBeginning(*amount),
                        (Mode::Default, [col], b'G') => Ansi::CursorToCol(*col),

                        (Mode::Default, [], b'J') => Ansi::EraseDisplay,
                        (Mode::Default, [0], b'J') => Ansi::EraseFromCursor,
                        (Mode::Default, [1], b'J') => Ansi::EraseToCursor,
                        (Mode::Default, [2], b'J') => Ansi::EraseScreen,
                        (Mode::Default, [3], b'J') => Ansi::EraseSavedLines,

                        (Mode::Default, [], b'K') => Ansi::EraseInLine,
                        (Mode::Default, [0], b'K') => Ansi::EraseFromCursorToEndOfLine,
                        (Mode::Default, [1], b'K') => Ansi::EraseFromCursorToEndOfLine,
                        (Mode::Default, [2], b'K') => Ansi::EraseLine,

                        (Mode::Default, [0], b'm') => Ansi::ResetAllGraphics,
                        (Mode::Default, [1], b'm') => Ansi::SetBold,
                        (Mode::Default, [22], b'm') => Ansi::ResetBoldDim,
                        (Mode::Default, [2], b'm') => Ansi::SetDim,

                        (Mode::Default, [3], b'm') => Ansi::SetItalic,
                        (Mode::Default, [23], b'm') => Ansi::ResetItalic,
                        (Mode::Default, [4], b'm') => Ansi::SetUnderline,
                        (Mode::Default, [24], b'm') => Ansi::ResetUnderline,
                        (Mode::Default, [5], b'm') => Ansi::SetBlinking,
                        (Mode::Default, [25], b'm') => Ansi::ResetBlinking,
                        (Mode::Default, [7], b'm') => Ansi::SetInverse,
                        (Mode::Default, [27], b'm') => Ansi::ResetInverse,
                        (Mode::Default, [8], b'm') => Ansi::SetHidden,
                        (Mode::Default, [28], b'm') => Ansi::ResetHidden,
                        (Mode::Default, [9], b'm') => Ansi::SetStrikethrough,
                        (Mode::Default, [29], b'm') => Ansi::ResetStrikethrough,

                        _ => Ansi::Invalid
                    })
                } 
    
                _ => {
                    self.state = State::Default;
                    Out::Ansi(Ansi::Invalid)
                }
            };
        }

    }
}