#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(const_type_name)]

pub mod entry;
pub mod mmio;
pub mod panic;
pub mod prelude;
pub mod timer;
pub mod uart;

use panic::reset;
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


fn user_in(buffer: &mut [u8]) -> &str{
    let mut len = 0;
    let mut pos = 0;

    uart::print_c(b'\n');
    loop {
        {
            uart::print("\x1b[2K\x1b[0G");
            let start_msg = "milkv :>";
            uart::print(start_msg);
            for b in &buffer[..len] {
                uart::print_b(*b);
            }
            print!("\x1b[0G\x1b[{}C", pos + start_msg.len());
        }

        match uart::get_b() {
            0x1b => {
                if uart::get_b() == 0x5b{
                    match uart::get_b() {
                        0x43 => {
                            pos = (pos + 1).min(buffer.len() - 1).min(len);
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
            }
            0x03 => {
                pos = 0;
                len = 0;
            }
            13 => break,
            0x08 => {
                if len > 0 && pos > 0 {
                    for i in pos..len {
                        buffer[i] = buffer[i + 1];
                    }
                    pos -= 1;
                    len -= 1;
                }
            }
            reg if reg.is_ascii() && !reg.is_ascii_control() => {
                if len < buffer.len() {
                    for i in (pos..len).rev() {
                        buffer[i + 1] = buffer[i];
                    }
                    buffer[pos] = reg;
                    pos += 1;
                    len += 1;
                }
            }
            _ => {}
        }
    }
    uart::print_c(b'\n');

    let command = &buffer[..len];
    core::str::from_utf8(command).unwrap()
}



trait Command{
    fn name(&self) -> &'static str;
    fn run(&self, args: &str) -> Result<(), &'static str>;
    fn help(&self) -> &'static str;
}

macro_rules! cmd {
    ($cmd_mnemonic:literal, $help:literal, ($self:ident, $args:ident) -> $code:block) => {{
        struct CmdImpl;

        impl Command for CmdImpl{
            fn name(&self) -> &'static str {
                $cmd_mnemonic
            }

            fn run(&$self, $args: &str) -> Result<(), &'static str> {
                $code
                #[allow(unreachable_code)]
                Ok(())
            }

            fn help(&self) -> &'static str{
                $help
            }
        }

        &CmdImpl
    }};
}

macro_rules! args {
    ($args:expr, ($($name:ident: $type:ty $(= $default:expr)?),*)) => {
        let mut args = $args.split_whitespace().map(|v|v.trim());
        $(
            let $name = if let Some(arg) = args.next(){
                use core::str::FromStr;
                if let Ok(val) = <$type>::from_str(arg){
                    val
                }else{
                    return Err(cfmt::formatcp!("Unable to parse argument ({}: {})", stringify!($name), core::any::type_name::<$type>()))
                }
            }else{
                args!(DEFAULT, $name, $type, $($default)?)
            };
        )*
    };
    (DEFAULT, $name:ident, $type:ty,) => {
        return Err(cfmt::formatcp!("Expected argument ({}: {}) but it was not provided", stringify!($name), core::any::type_name::<$type>()))
    };
    (DEFAULT, $name:ident, $type:ty, $default:expr) => {
        $default
    };
}

const COMAMNDS: &[&'static dyn Command] = &[
    cmd!("help", "prints this message", (self, _args) -> {
        for cmd in COMAMNDS{
            println!("{}: {}", cmd.name(), cmd.help())
        }
    }),
    cmd!("goto", "(addr: usize) jumps to the specified address", (self, args) -> {
        args!(args, (addr: usize));
        unsafe{
            core::arch::asm!("jalr {0}", in (reg) addr)
        }
    }),
    cmd!("md.b", "(addr: usize, count: usize = 256) displays count bytes starting at 'addr'", (self, args) -> {
        args!(args, (addr: usize, count: usize = 256));
        print_mem_b(addr, count);
    }),
    cmd!("start", "prints the starting address of this bootloader", (self, _args) -> {
        unsafe{
            let addr: u64;
            core::arch::asm!("la {0},bl_entrypoint", out(reg) addr);
            println!("Start address: 0x{addr:016x}");
        }
    }),
    cmd!("panic", "(msg: &str) panics with the provided message", (self, args) -> {
      panic!("{}", args);
    }),
    cmd!("reset", "resets the board", (self, _args) -> {
      unsafe { reset() }
    }),
];

#[no_mangle]
pub extern "C" fn bl_rust_main() {
    timer::mdelay(250);
    unsafe {
        uart::console_init();
    }
    timer::mdelay(250);

    uart::print("\n\n\nBooted into firmware. Starting console\n");

    let mut buffer = [0; 512];
    'next_cmd:
    loop {
        let msg = user_in(&mut buffer);
        let (cmd_mnemonic, args) = msg.trim().split_once(' ').unwrap_or((msg, ""));
        let args = args.trim();

        for cmd in COMAMNDS{
            if cmd.name() == cmd_mnemonic{
                if let Err(err) = cmd.run(args){
                    println!("{} {err}", cmd.name());
                }
                continue 'next_cmd;
            }
        }
        println!("unknown command {cmd_mnemonic:?}. Type 'help' for a list of commands");
    }
}
