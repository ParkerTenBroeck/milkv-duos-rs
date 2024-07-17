use ansi::*;

const DO: bool = false;

fn main() {
    let file = std::fs::read("../ttyout2.txt").unwrap();
    let mut parser: AnsiParser = AnsiParser::new();

    for byte in file {
        match parser.next(byte) {
            Out::Ansi(a) => {
                if false {
                    println!("{a:#?}")
                }
            }
            Out::Data(c) => {
                if DO{
                    print!("{:?}", c as char);
                }
            }
            _ => {}
        }
    }
}
