#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;

// pub use prelude::*;

#[no_mangle]
extern "C" fn os_main() {

}
