pub static mut SOUT: fn(&[u8]) = milkv_rs::uart::print_bytes;

pub fn print(msg: &str) {
    unsafe { SOUT(msg.as_bytes()) }
}

pub fn print_bytes(msg: &[u8]) {
    unsafe { SOUT(msg) }
}
