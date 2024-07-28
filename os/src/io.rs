pub static mut SOUT: fn(&[u8]) = |b| crate::write(b);

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
                let func: fn(usize) = core::mem::transmute(0x000000000c000000usize + 40);
                milkv_rs::csr::disable_interrupts();
                func(1);
            }
        } else {
            COUNT = 0;
        }
    }
    b
}
pub fn has_b() -> bool {
    unsafe { !crate::uart_sys::RX.is_empty() }
}
