use crate::interrupt_vector::reset;

pub static mut SOUT: fn(&[u8]) = |b| milkv_rs::uart::print_bytes(b);

pub fn print(msg: &str) {
    unsafe { SOUT(msg.as_bytes()) }
}

pub fn print_bytes(msg: &[u8]) {
    unsafe { SOUT(msg) }
}

pub fn read_b() -> u8 {
    static mut COUNT: u8 = 0;
    let b = milkv_rs::uart::get_b();

    unsafe {
        if b == 0x7E || (COUNT == 6 && b == b'\r') {
            COUNT += 1;
            // crate::println!("{b}");
            if b == b'\r' {
                reset()
            }
        } else {
            COUNT = 0;
        }
    }
    b
}
pub fn has_b() -> bool {
    milkv_rs::uart::has_b()
    // unsafe { !crate::uart_sys::RX.is_empty() }
}
