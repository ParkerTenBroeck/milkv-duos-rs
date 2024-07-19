use ansi::*;

const DO: bool = false;

fn main() {
    let file = std::fs::read("../ttyout2.txt").unwrap();
    let mut parser: AnsiParser = AnsiParser::new();

    for byte in file {
        match parser.next(byte as char) {
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


#[test]
fn test(){
    let mut buf = [0; 4];
    for i in 0x9F + 1..=u32::MAX{
        if let Some(v) = char::from_u32(i){
            let str =  v.encode_utf8(&mut buf);
            // println!("{:02x?}", str.as_bytes())
            // if str.len() == 1{
            //     continue;
            // }
            let has = str.as_bytes().iter().any(|v|(0x80..=0x9f).contains(v));
            assert!(!has, "{:02x?}", (i, str.as_bytes()));
            // println!("{}")
        }
    }
}