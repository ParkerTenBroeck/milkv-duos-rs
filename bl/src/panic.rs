use crate::*;

#[panic_handler]
pub fn rust_panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    milkv_rs::uart::print("Rust panic... resetting\n");
    unsafe { milkv_rs::reset() }
}
