use core::arch;

use crate::io;
pub use crate::prelude::*;
use crate::print;
use crate::println;

pub fn print_mem_u8(addr_area: usize, size: usize) {
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

pub fn print_mem_u16(addr_area: usize, size: usize) {
    const BLOCK_SIZE: usize = 16;
    let addr_start = addr_area & !(BLOCK_SIZE - 1);
    let addr_end = (addr_area + size + BLOCK_SIZE - 1) & !(BLOCK_SIZE - 1);

    print!("                  ");
    for i in (0..BLOCK_SIZE).step_by(core::mem::size_of::<u16>()) {
        print!("  {i:02X} ");
    }
    println!();

    for block_start in (addr_start..addr_end).step_by(BLOCK_SIZE) {
        print!("0x{block_start:016X}");
        for i in (0..BLOCK_SIZE).step_by(core::mem::size_of::<u16>()) {
            let addr = block_start + i;
            let ptr = addr as *const u16;
            print!(" {:04X}", unsafe { ptr.read_volatile() });
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

pub fn print_mem_u32(addr_area: usize, size: usize) {
    const BLOCK_SIZE: usize = 16;
    let addr_start = addr_area & !(BLOCK_SIZE - 1);
    let addr_end = (addr_area + size + BLOCK_SIZE - 1) & !(BLOCK_SIZE - 1);

    print!("                  ");
    for i in (0..BLOCK_SIZE).step_by(core::mem::size_of::<u32>()) {
        print!("    {i:02X}   ");
    }
    println!();

    for block_start in (addr_start..addr_end).step_by(BLOCK_SIZE) {
        print!("0x{block_start:016X}");
        for i in (0..BLOCK_SIZE).step_by(core::mem::size_of::<u32>()) {
            let addr = block_start + i;
            let ptr = addr as *const u32;
            print!(" {:08X}", unsafe { ptr.read_volatile() });
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

fn user_in(buffer: &mut [u8]) -> &str {
    let mut len = 0;
    let mut pos = 0;

    io::print("milkv :>");
    'outer: loop {
        // if len != pos && len != 0{
        //     io::print("\x1b[2K\x1b[0G");
        //     let start_msg = "milkv :>";
        //     io::print(start_msg);
        //     io::print_bytes(&buffer[..len]);
        //     print!("\x1b[0G\x1b[{}C", pos + start_msg.len());
        // }
        'inner: while {
            match uart::get_b() {
                0x1b => {
                    if uart::get_b() == 0x5b {
                        match uart::get_b() {
                            0x43 => {
                                // right
                                if pos < buffer.len() - 1 && pos < len {
                                    pos += 1;
                                    io::print("\x1b[1C");
                                }
                            }
                            0x44 => {
                                // left
                                if pos != 0 {
                                    pos -= 1;
                                    io::print("\x1b[1D");
                                }
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
                    if pos != 0 {
                        print!("\x1b[{}D", pos);
                    }
                    io::print("\x1b[0J");
                    pos = 0;
                    len = 0;
                }
                13 => break 'outer,
                0x08 => {
                    if len > 0 && pos > 0 {
                        for i in (pos - 1)..(len - 1) {
                            buffer[i] = buffer[i + 1];
                        }
                        pos -= 1;
                        len -= 1;

                        io::print("\x1b[1D");
                        io::print("\x1b[0J");
                        if len - pos > 0 {
                            io::print_bytes(&buffer[pos..len]);
                            print!("\x1b[{}D", len - pos);
                        }
                    }
                    break 'inner;
                }
                reg if reg.is_ascii() && !reg.is_ascii_control() => {
                    if len < buffer.len() {
                        for i in (pos..len).rev() {
                            buffer[i + 1] = buffer[i];
                        }
                        buffer[pos] = reg;
                        pos += 1;
                        len += 1;
                        io::print_bytes(core::slice::from_ref(&reg));
                        if len - pos > 0 {
                            io::print_bytes(&buffer[pos..len]);
                            print!("\x1b[{}D", len - pos);
                        }
                    }
                }
                c => panic!("invalid char: {:#?}, {:02x}", c as char, c),
            }
            uart::has_b()
        } {}
    }
    io::print("\n");

    let command = &buffer[..len];
    core::str::from_utf8(command).unwrap()
}

trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, args: &str);
    fn help(&self) -> &'static str;
}

macro_rules! cmd {
    ($cmd_mnemonic:literal, $help:literal, ($self:ident, $args:ident) -> $code:block) => {{
        struct CmdImpl;

        impl Command for CmdImpl{
            fn name(&self) -> &'static str {
                $cmd_mnemonic
            }

            fn run(&$self, $args: &str) {
                $code
            }

            fn help(&self) -> &'static str{
                $help
            }
        }

        &CmdImpl
    }};
}

trait Argument: Sized {
    const DISP: &str;
    fn parse(str: &str) -> Result<Self, &'static str>;
}

impl Argument for u8 {
    const DISP: &str = "u8";

    fn parse(str: &str) -> Result<Self, &'static str> {
        fn parse_str(str: &str, rad: u32) -> Result<u8, &'static str> {
            match u8::from_str_radix(str, rad) {
                Ok(v) => Ok(v),
                Err(err) => Err(match err.kind() {
                    core::num::IntErrorKind::Empty => "Empty",
                    core::num::IntErrorKind::InvalidDigit => "Invalid digit",
                    core::num::IntErrorKind::PosOverflow => "Integer too large",
                    core::num::IntErrorKind::NegOverflow => "Integer too small",
                    core::num::IntErrorKind::Zero => "Cannot be zero",
                    _ => "",
                }),
            }
        }
        if str.starts_with("0x") {
            parse_str(&str[2..], 16)
        } else if str.starts_with("0b") {
            parse_str(&str[2..], 2)
        } else {
            parse_str(&str, 10)
        }
    }
}

impl Argument for u16 {
    const DISP: &str = "u16";

    fn parse(str: &str) -> Result<Self, &'static str> {
        fn parse_str(str: &str, rad: u32) -> Result<u16, &'static str> {
            match u16::from_str_radix(str, rad) {
                Ok(v) => Ok(v),
                Err(err) => Err(match err.kind() {
                    core::num::IntErrorKind::Empty => "Empty",
                    core::num::IntErrorKind::InvalidDigit => "Invalid digit",
                    core::num::IntErrorKind::PosOverflow => "Integer too large",
                    core::num::IntErrorKind::NegOverflow => "Integer too small",
                    core::num::IntErrorKind::Zero => "Cannot be zero",
                    _ => "",
                }),
            }
        }
        if str.starts_with("0x") {
            parse_str(&str[2..], 16)
        } else if str.starts_with("0b") {
            parse_str(&str[2..], 2)
        } else {
            parse_str(&str, 10)
        }
    }
}

impl Argument for u32 {
    const DISP: &str = "u32";

    fn parse(str: &str) -> Result<Self, &'static str> {
        fn parse_str(str: &str, rad: u32) -> Result<u32, &'static str> {
            match u32::from_str_radix(str, rad) {
                Ok(v) => Ok(v),
                Err(err) => Err(match err.kind() {
                    core::num::IntErrorKind::Empty => "Empty",
                    core::num::IntErrorKind::InvalidDigit => "Invalid digit",
                    core::num::IntErrorKind::PosOverflow => "Integer too large",
                    core::num::IntErrorKind::NegOverflow => "Integer too small",
                    core::num::IntErrorKind::Zero => "Cannot be zero",
                    _ => "",
                }),
            }
        }
        if str.starts_with("0x") {
            parse_str(&str[2..], 16)
        } else if str.starts_with("0b") {
            parse_str(&str[2..], 2)
        } else {
            parse_str(&str, 10)
        }
    }
}

impl Argument for usize {
    const DISP: &str = "usize";

    fn parse(str: &str) -> Result<Self, &'static str> {
        fn parse_str(str: &str, rad: u32) -> Result<usize, &'static str> {
            match usize::from_str_radix(str, rad) {
                Ok(v) => Ok(v),
                Err(err) => Err(match err.kind() {
                    core::num::IntErrorKind::Empty => "Empty",
                    core::num::IntErrorKind::InvalidDigit => "Invalid digit",
                    core::num::IntErrorKind::PosOverflow => "Integer too large",
                    core::num::IntErrorKind::NegOverflow => "Integer too small",
                    core::num::IntErrorKind::Zero => "Cannot be zero",
                    _ => "",
                }),
            }
        }
        if str.starts_with("0x") {
            parse_str(&str[2..], 16)
        } else if str.starts_with("0b") {
            parse_str(&str[2..], 2)
        } else {
            parse_str(&str, 10)
        }
    }
}

impl Argument for bool {
    const DISP: &str = "bool";

    fn parse(str: &str) -> Result<Self, &'static str> {
        match str {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err("Invalid repr, expected ['true', 'false']"),
        }
    }
}

enum GpioKind {
    Toggle,
    Set,
    Get,
}

impl Argument for GpioKind {
    const DISP: &str = "[Toggle, Set, Get]";

    fn parse(str: &str) -> Result<Self, &'static str> {
        match str {
            "Toggle" | "toggle" => Ok(Self::Toggle),
            "Get" | "get" => Ok(Self::Get),
            "Set" | "set" => Ok(Self::Set),
            _ => Err("Invalid repr, expected ['Toggle', 'Set', 'Get']"),
        }
    }
}

enum TimerOp {
    Read,
    Ei,
    E,
    C,
    Ci,
    Set,
}

impl Argument for TimerOp {
    const DISP: &str = "[Read, Ei, E, C, Ci, Set]";

    fn parse(str: &str) -> Result<Self, &'static str> {
        match str {
            "Read" | "read" => Ok(Self::Read),
            "Ei" | "ei" => Ok(Self::Ei),
            "E" | "e" => Ok(Self::E),
            "C" | "c" => Ok(Self::C),
            "Set" | "set" => Ok(Self::Set),
            "Ci" | "ci" => Ok(Self::Ci),
            _ => Err("Invalid repr, expected ['Read', 'Ei', 'E', 'C', 'Set']"),
        }
    }
}

macro_rules! args {
    ($args:expr, ($($name:ident: $type:ty $(= $default:expr)?),*)) => {
        let mut args = $args.split_whitespace().map(|v|v.trim()).peekable();
        $(
            let $name = if let Some(arg) = args.peek(){
                match <$type as Argument>::parse(arg){
                  Ok(val) => {
                    args.next();
                    val
                  }
                  Err(_err) => {
                    args!(BADPARSE, _err, $name, $type, $($default)?)
                  }
                }
            }else{
                args!(DEFAULT, $name, $type, $($default)?)
            };
        )*
    };
    (BADPARSE, $err:expr, $name:ident, $type:ty,) => {
      return println!("Unable to parse argument ({}: {}) {}", stringify!($name), <$type as Argument>::DISP, $err)
    };
    (BADPARSE, $err:expr, $name:ident, $type:ty, $default:expr) => {
        $default
    };
    (DEFAULT, $name:ident, $type:ty,) => {
        return println!("Expected argument ({}: {}) but it was not provided", stringify!($name), <$type as Argument>::DISP)
    };
    (DEFAULT, $name:ident, $type:ty, $default:expr) => {
        $default
    };
}

fn print_csrs() {
    println!("marchid   0x{:016x}", csr::marchid());
    println!("mhartid   0x{:016x}", csr::mhartid());
    println!("mimpid    0x{:016x}", csr::mimpid());
    {
        {
            let misa = csr::misa();
            println!("misa      0x{:016x}", misa);
            let mut index = 0;
            for v in 'A'..='Z' {
                if (misa >> index) & 1 == 1 {
                    println!("  {}", v);
                }
                index += 1;
            }
        }
    }
    println!("mvendorid 0x{:016x}", csr::mvendorid());
    {
        let mie = csr::read_mie();
        println!("mie       0x{:016x}", mie);
        let values = [
            ("0", 1),
            ("ssie", 1),
            ("0", 1),
            ("msie", 1),
            ("0", 1),
            ("stie", 1),
            ("0", 1),
            ("mtie", 1),
            ("0", 1),
            ("seie", 1),
            ("0", 1),
            ("meie", 1),
            ("0", 4),
        ];
        let mut index = 0;
        for v in values {
            println!(
                "  {}: 0b{:0width$b}",
                v.0,
                (mie >> index) & ((1 << v.1) - 1),
                width = v.1
            );
            index += v.1;
        }
    }

    println!("medeleg   0x{:016x}", csr::medeleg());
    println!("mideleg   0x{:016x}", csr::mideleg());
    {
        let mip = csr::read_mip();
        println!("mip       0x{:016x}", mip);
        let values = [
            ("0", 1),
            ("ssip", 1),
            ("0", 1),
            ("msip", 1),
            ("0", 1),
            ("stip", 1),
            ("0", 1),
            ("mtip", 1),
            ("0", 1),
            ("seip", 1),
            ("0", 1),
            ("meip", 1),
            ("0", 4),
        ];
        let mut index = 0;
        for v in values {
            println!(
                "  {}: 0b{:0width$b}",
                v.0,
                (mip >> index) & ((1 << v.1) - 1),
                width = v.1
            );
            index += v.1;
        }
    }
    {
        let mstatus = csr::read_mstatus();
        println!("mstatus   0x{:016x}", mstatus);
        let values = [
            ("wpri", 1),
            ("sie", 1),
            ("wpri", 1),
            ("mie", 1),
            ("wpri", 1),
            ("spie", 1),
            ("ube", 1),
            ("mpie", 1),
            ("spp", 1),
            ("vs", 2),
            ("mpp", 2),
            ("fs", 2),
            ("xs", 2),
            ("mprv", 1),
            ("sum", 1),
            ("mxr", 1),
            ("tvm", 1),
            ("tw", 1),
            ("tsr", 1),
            ("wpri", 9),
            ("uxl", 2),
            ("sxl", 2),
            ("sbe", 1),
            ("mbe", 1),
            ("wpri", 25),
            ("sd", 1),
        ];
        let mut index = 0;
        for v in values {
            println!(
                "  {}: 0b{:0width$b}",
                v.0,
                (mstatus >> index) & ((1 << v.1) - 1),
                width = v.1
            );
            index += v.1;
        }
    }
}

const COMAMNDS: &[&'static dyn Command] = &[
    cmd!("help", "prints this message", (self, _args) -> {
        for cmd in COMAMNDS{
            println!("{}: {}", cmd.name(), cmd.help())
        }
    }),
    cmd!("clear", "Clears the screen", (self, _args) -> {
        io::print("\x1b[2J")
    }),
    cmd!("goto", "(addr: usize) jumps to the specified address", (self, args) -> {
        args!(args, (addr: usize));
        let func: fn() = unsafe{ core::mem::transmute(addr) };
        func();
    }),
    cmd!("memset", "(addr: usize, len: usize, val: u8) writes 'val' to address 'addr' for 'len' bytes", (self, args) -> {
      args!(args, (addr: usize, len: usize, val: u8));
      let time = timer::get_mtimer();
      unsafe{

        (addr as *mut u8).write_bytes(val, len);
      }
      let took = timer::get_mtimer().wrapping_sub(time)/timer::SYS_COUNTER_FREQ_IN_US;
      println!("took: {took}us");
    }),
    cmd!("mw.8", "(addr: usize, val: u8) writes 'val' to address 'addr'", (self, args) -> {
      args!(args, (addr: usize, val: u8));
      unsafe{
        (addr as *mut u8).write_volatile(val);
      }
    }),
    cmd!("mw.16", "(addr: usize, val: u16) writes 'val' to address 'addr'", (self, args) -> {
      args!(args, (addr: usize, val: u16));
      unsafe{
        (addr as *mut u16).write_volatile(val);
      }
    }),
    cmd!("mw.32", "(addr: usize, val: u32) writes 'val' to address 'addr'", (self, args) -> {
      args!(args, (addr: usize, val: u32));
      unsafe{
        (addr as *mut u32).write_volatile(val);
      }
    }),
    cmd!("md.8", "(addr: usize, count: usize = 256) displays 'count' u8s starting at 'addr'", (self, args) -> {
        args!(args, (addr: usize, count: usize = 256));
        print_mem_u8(addr, count);
    }),
    cmd!("md.16", "(addr: usize, count: usize = 256) displays 'count' u16s starting at 'addr'", (self, args) -> {
      args!(args, (addr: usize, count: usize = 256));
      print_mem_u16(addr, count);
    }),
    cmd!("md.32", "(addr: usize, count: usize = 256) displays 'count' u32s starting at 'addr'", (self, args) -> {
        args!(args, (addr: usize, count: usize = 256));
        print_mem_u32(addr, count);
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
    cmd!("gpio", "(kind: [Toggle, Set, Get], pin: u8, value: bool = false) modifies gpio", (self, args) -> {
      args!(args, (kind: GpioKind, pin: u8, value: bool = false));
      match kind{
          GpioKind::Toggle => unsafe {
            gpio::set_gpio_direction(mmio::GPIO0, pin, gpio::Direction::Output);
            gpio::set_gpio(mmio::GPIO0, pin, !gpio::read_gpio(mmio::GPIO0, pin));
          },
          GpioKind::Set => unsafe {
            gpio::set_gpio_direction(mmio::GPIO0, pin, gpio::Direction::Output);
            gpio::set_gpio(mmio::GPIO0, pin, value);
          },
          GpioKind::Get => unsafe {
            gpio::set_gpio_direction(mmio::GPIO0, pin, gpio::Direction::Input);
            println!("gpio pin {} is {}", pin, if gpio::read_gpio(mmio::GPIO0, pin) { "High" } else { "Low" } );
          },
      }
    }),
    cmd!("sap", "", (self, _args) -> {
      unsafe{
        let out: u64;
        core::arch::asm!(
          "csrr {0}, satp",
          out(reg) out
        );
        println!("Value: 0x{out:016x}");

        println!("mode: 0x{:x}", out >> (16+44));
        println!("asid: 0x{:x}", (out >> 44) & ((1<<16) - 1));
        println!("ppn: 0x{:x}", out & ((1<<44) - 1));
      }
    }),
    cmd!("ebreak", "", (self, _args) -> {
      unsafe{
        core::arch::asm!(
          "ebreak"
        );
      }
    }),
    cmd!("ecall", "", (self, _args) -> {
      unsafe{
        core::arch::asm!(
          "ecall"
        );
      }
    }),
    // cmd!("invcache", "", (self, _args) -> {
    //   csr::invalidate_d_cache()
    // }),
    cmd!("mtimer", "reads mtime/mtimecmp", (self, _args) -> {
      println!("mtimer:    0x{:016x}", timer::get_mtimer());
      println!("mtimercmp: 0x{:016x}", timer::get_mtimercmp());
    }),
    cmd!("timer", "(t: usize, op: [Read, Ei, E, C, Ci, Set], val: u32 = 0)", (self, args) -> {
      args!(args, (t: usize, op: TimerOp, val: u32 = 0));
      let base = 0x030A0000;
      let tbase = base + t * 0x14;
      let tim = tbase as *mut u32;
      match op{
          TimerOp::Read => unsafe{
            println!("Timer{t} load: {}", tim.read_volatile());
            println!("Timer{t} curr: {}", tim.add(1).read_volatile());
            {
              let ctrl = tim.add(2).read_volatile();
              println!("Timer{t} int mask: {}", if ctrl & 4 == 4 { "masked" } else { "not masked" });
              println!("Timer{t} mode: {}", if ctrl & 2 == 2 { "count" } else { "free" });
              println!("Timer{t} enable: {}", ctrl & 1 == 1);
            }
            println!("Timer{t} int stat: {}", tim.add(4).read_volatile());
            println!("Timer int stat: 0b{:08b}", ((base+0xa8) as *const u32).read_volatile());
          },
          TimerOp::Ei => unsafe{
            let mut ctrl = tim.add(2).read_volatile();
            if val == 0{
              ctrl &= !0b100;
            }else{
              ctrl |= 0b100;
            }
            tim.add(2).write_volatile(ctrl);
          },
          TimerOp::E => unsafe{
            let mut ctrl = tim.add(2).read_volatile();
            if val == 0{
              ctrl &= !0b1;
            }else{
              ctrl |= 0b1;
            }
            tim.add(2).write_volatile(ctrl);
          },
          TimerOp::C => unsafe{
            let mut ctrl = tim.add(2).read_volatile();
            if val == 0{
              ctrl &= !0b10;
            }else{
              ctrl |= 0b10;
            }
            tim.add(2).write_volatile(ctrl);
          },
          TimerOp::Ci => unsafe {
            ((base + 0xa4) as *const u32).read_volatile();
          },
          TimerOp::Set => unsafe {
            tim.write_volatile(val);
          },
      }
    }),
    cmd!("csrs", "", (self, _args) -> {
      print_csrs();
    }),
    cmd!("mstatuss", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr::set_mstatus(val);
      }
    }),
    cmd!("mstatusc", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr::clear_mstatus(val);
      }
    }),
    cmd!("mies", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr::set_mie(val);
      }
    }),
    cmd!("miec", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr::clear_mie(val);
      }
    }),
    cmd!("plicd", "", (self, _args) -> {
      unsafe{
        plic::disp(Stdout);
      }
    }),
    cmd!("plics", "(int: u32, pri: u32)", (self, args) -> {
      args!(args, (int: u32, pri: u32));
      unsafe{
        if pri == 0{
          plic::set_priority(int, pri);
          plic::enable_m_interrupt(int);
        }else{
          plic::set_priority(int, 0);
          plic::disable_m_interrupt(int);
        }
      }
    }),
    // cmd!("vgai", "", (self, _args) -> {
    //     unsafe{
    //         crate::vga_core::init_vga();
    //     }
    // }),
    // cmd!("vgal", "", (self, _args) -> {
    //     unsafe{
    //         println!("H_FRONT_PORCH: {}", crate::vga::H_FRONT_PORCH);
    //         println!("H_SYNC_PULSE: {}", crate::vga::H_SYNC_PULSE);
    //         println!("H_BACK_PORCH: {}", crate::vga::H_BACK_PORCH);
    //         println!("H_TOTAL: {}", crate::vga::H_TOTAL);

    //         println!("\nH_FP_M: {}", crate::vga::H_FP_M);
    //         println!("H_SP_M: {}", crate::vga::H_SP_M);
    //         println!("H_BP_M: {}", crate::vga::H_BP_M);

    //         println!("\nV_FRONT_PORCH: {}", crate::vga::V_FRONT_PORCH);
    //         println!("V_SYNC_PULSE: {}", crate::vga::V_SYNC_PULSE);
    //         println!("V_BACK_PORCH: {}", crate::vga::V_BACK_PORCH);
    //         println!("V_TOTAL: {}", crate::vga::V_TOTAL);
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    cmd!("mcycle", "", (self, _args) -> {
        println!("{}", csr::mcycle())
    }),
    cmd!("scr", "", (self, _args) -> {
        unsafe{
            platform::reset_c906l();
        }
    }),
    cmd!("scs", "", (self, _args) -> {
        unsafe{
            platform::start_c906l();
        }
    }),
    cmd!("lsc", "", (self, _args) -> {
        unsafe{
            platform::reset_c906l();
            milkv_rs::csr::disable_interrupts();

            let addr = core::array::from_fn::<_, 8, _>(|_|uart::get_b());
            let len = core::array::from_fn::<_, 8, _>(|_|uart::get_b());
            let addr = usize::from_be_bytes(addr);
            let len = usize::from_be_bytes(len);
            for i in addr..(addr + len){
                (i as *mut u8).write(uart::get_b());
            }
            platform::reset_c906l_to_addr(addr);
            for i in (addr..(addr + len)).step_by(64){
                core::arch::asm!("
                    th.dcache.cpa {0}
                ",
                in(reg) i);
            }
            println!("{addr:08x}:{len}");
        }
    }),
    cmd!("lpc", "", (self, _args) -> {
        unsafe{
            platform::reset_c906l();
            milkv_rs::csr::disable_interrupts();

            let addr = core::array::from_fn::<_, 8, _>(|_|uart::get_b());
            let len = core::array::from_fn::<_, 8, _>(|_|uart::get_b());
            let addr = usize::from_be_bytes(addr);
            let len = usize::from_be_bytes(len);
            for i in addr..(addr + len){
                (i as *mut u8).write(uart::get_b());
            }
            println!("{addr:08x}:{len}");
            // print_mem_u32(addr, len);
            // milkv_rs::csr::enable_interrupts();
            core::arch::asm!(
                "
                th.dcache.call
                th.dcache.iall
                th.icache.iall
                jr {0}",
                in(reg) addr
            );
        }
    }),
    // cmd!("vgahsp", "(sync: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_TOTAL -= crate::vga::H_SYNC_PULSE;
    //         crate::vga::H_TOTAL += sync as u64;
    //         crate::vga::H_SYNC_PULSE = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgahfp", "(fp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_TOTAL -= crate::vga::H_FRONT_PORCH;
    //         crate::vga::H_TOTAL += sync as u64;
    //         crate::vga::H_FRONT_PORCH = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgahbp", "(bp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_TOTAL -= crate::vga::H_BACK_PORCH;
    //         crate::vga::H_TOTAL += sync as u64;
    //         crate::vga::H_BACK_PORCH = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgahspm", "(sync: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_SP_M = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgahfpm", "(fp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_FP_M = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgahbpm", "(bp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::H_BP_M = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgavsp", "(sync: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::V_TOTAL -= crate::vga::V_SYNC_PULSE;
    //         crate::vga::V_TOTAL += sync as u64;
    //         crate::vga::V_SYNC_PULSE = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgavfp", "(fp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::V_TOTAL -= crate::vga::V_FRONT_PORCH;
    //         crate::vga::V_TOTAL += sync as u64;
    //         crate::vga::V_FRONT_PORCH = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgavbp", "(bp: u64)", (self, args) -> {
    //     args!(args, (sync: usize));
    //     unsafe{
    //         crate::vga::V_TOTAL -= crate::vga::V_BACK_PORCH;
    //         crate::vga::V_TOTAL += sync as u64;
    //         crate::vga::V_BACK_PORCH = sync as u64;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
    // cmd!("vgad1", "(on: bool)", (self, args) -> {
    //     args!(args, (sync: bool));
    //     unsafe{
    //         crate::vga::D1 = sync;
    //         core::arch::asm!(
    //             "th.dcache.call"
    //         );
    //     }
    // }),
];

pub fn run() {
    'next_cmd: loop {
        let mut buffer = [0; 512];
        let msg = user_in(&mut buffer);
        let (cmd_mnemonic, args) = msg.trim().split_once(' ').unwrap_or((msg, ""));
        let args = args.trim();

        for cmd in COMAMNDS {
            if cmd.name() == cmd_mnemonic {
                cmd.run(args);
                continue 'next_cmd;
            }
        }
        println!("unknown command {cmd_mnemonic:?}. Type 'help' for a list of commands");
    }
}
