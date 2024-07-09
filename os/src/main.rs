#![no_std]
#![no_main]
#![feature(asm_const)]

use milkv_rs::*;
// pub mod entry;
// pub mod interrupt_vector;
pub mod panic;
// pub mod prelude;

// pub use prelude::*;

#[no_mangle]
extern "C" fn os_main() {}

#[no_mangle]
extern "C" fn _os_start() -> (usize, usize) {
    let rstart = timer::get_mtimer();
    let mut val = 0;
    unsafe {
        csr::disable_interrupts();
    }
    let start = timer::get_mtimer();
    while start == timer::get_mtimer() {
        val += 1;
    }
    unsafe {
        csr::enable_interrupts();
    }
    let t = timer::get_mtimer().wrapping_sub(rstart) as usize;
    // uart::print("magic string :3\n");
    (val, t)
}
