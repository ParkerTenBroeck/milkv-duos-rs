#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod entry;
pub mod mmio;
pub mod panic;
pub mod prelude;
pub mod timer;
pub mod uart;

pub use prelude::*;

pub fn print_mem_b(addr_area: usize, size: usize) {
    const BLOCK_SIZE: usize = 16;
    let addr_start = addr_area & !(BLOCK_SIZE - 1);
    let addr_end = (addr_area + size + BLOCK_SIZE - 1) & !(BLOCK_SIZE - 1);

    print!("                  ");
    for i in 0..BLOCK_SIZE {
        print!(" {i:02X}");
    }
    println!();

    for block_start in (addr_start..addr_end).step_by(BLOCK_SIZE) {
        print!("0x{block_start:016X}");
        for i in 0..BLOCK_SIZE {
            let addr = block_start + i;
            let ptr = addr as *const u8;
            print!(" {:02X}", unsafe { ptr.read_volatile() });
        }
        print!(" ");
        for i in 0..BLOCK_SIZE {
            let addr = block_start + i;
            let ptr = addr as *const u8;
            let val = unsafe { ptr.read_volatile() };
            if val.is_ascii_graphic() {
                print!("{}", val as char);
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[no_mangle]
pub extern "C" fn bl_rust_main(start: u64) {
    timer::mdelay(250);
    unsafe {
        uart::console_init();
    }
    timer::mdelay(250);

    uart::print("\n\n\nBooted into firmware. Starting console\n");

    let mut user_in_buf = [0; 512];
    loop {
        let mut len = 0;
        let mut pos = 0;

        uart::print_c(b'\n');
        loop {
            {
                uart::print("\x1b[2K\x1b[0G");
                let start_msg = "milkv :>";
                uart::print(start_msg);
                for b in &user_in_buf[..len] {
                    uart::print_b(*b);
                }
                print!("\x1b[0G\x1b[{}C", pos + start_msg.len());
            }

            match uart::get_b() {
                0x1b => {
                    match uart::get_b() {
                        0x5b => {
                            match uart::get_b() {
                                0x43 => {
                                    pos = (pos + 1).min(user_in_buf.len() - 1).min(len);
                                    // right
                                }
                                0x44 => {
                                    pos = pos.saturating_sub(1);
                                    // left
                                }
                                0x41 => {
                                    // up
                                }
                                0x42 => {
                                    // down
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0x03 => {
                    pos = 0;
                    len = 0;
                }
                13 => break,
                0x08 => {
                    if len > 0 && pos > 0 {
                        for i in pos..len {
                            user_in_buf[i] = user_in_buf[i + 1];
                        }
                        pos -= 1;
                        len -= 1;
                    }
                }
                reg if reg.is_ascii() && !reg.is_ascii_control() => {
                    if len < user_in_buf.len() {
                        for i in (pos..len).rev() {
                            user_in_buf[i + 1] = user_in_buf[i];
                        }
                        user_in_buf[pos] = reg;
                        pos += 1;
                        len += 1;
                    }
                }
                _ => {}
            }
        }
        uart::print_c(b'\n');

        let command = &user_in_buf[..len];

        for b in command {
            uart::print_b(*b);
        }

        // print_mem_b(start as usize, 256);
        // uart::print("\n");
        // uart::print("\n");
        // uart::print("\n");
        // uart::flush();
        // timer::mdelay(1000);
        // start += 256;
    }
}
