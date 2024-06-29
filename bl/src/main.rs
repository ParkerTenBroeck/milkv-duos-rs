#![no_std]
#![no_main]
#![feature(asm_const)]
#![allow(internal_features)]
#![feature(core_intrinsics)]

pub mod csr_reg;
pub mod ddr;
pub mod entry;
pub mod gpio;
pub mod mmio;
pub mod panic;
pub mod prelude;
pub mod timer;
pub mod uart;
pub mod efuse;
pub mod pinmux;
pub mod plic;
pub mod interrupt_vector;

use panic::reset;
pub use prelude::*;

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
                if uart::get_b() == 0x5b {
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

fn print_csr_regs(){
  println!("marchid   0x{:016x}", csr_reg::marchid());
  println!("mhartid   0x{:016x}", csr_reg::mhartid());
  println!("mimpid    0x{:016x}", csr_reg::mimpid());
  {
    {
      let misa = csr_reg::misa();
      println!("misa      0x{:016x}", misa);
      let mut index = 0;
      for v in 'A'..='Z'{
        if (misa >> index) & 1 == 1{
          println!("  {}", v);
        }
        index += 1;
      }
    }
  }
  println!("mvendorid 0x{:016x}", csr_reg::mvendorid());
  {
    let mie = csr_reg::read_mie();
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
    for v in values{
      println!("  {}: 0b{:0width$b}", v.0, (mie >> index) & ((1<<v.1) - 1), width = v.1);
      index += v.1;
    }
  }

  println!("medeleg   0x{:016x}", csr_reg::medeleg());
  println!("mideleg   0x{:016x}", csr_reg::mideleg());
  {
    let mip = csr_reg::read_mip();
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
    for v in values{
      println!("  {}: 0b{:0width$b}", v.0, (mip >> index) & ((1<<v.1) - 1), width = v.1);
      index += v.1;
    }
  }
  {
    let mstatus = csr_reg::read_mstatus();
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
    for v in values{
      println!("  {}: 0b{:0width$b}", v.0, (mstatus >> index) & ((1<<v.1) - 1), width = v.1);
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
    cmd!("goto", "(addr: usize) jumps to the specified address", (self, args) -> {
        args!(args, (addr: usize));
        let func: fn() = unsafe{ core::mem::transmute(addr) };
        func();
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
          GpioKind::Toggle => {
            gpio::set_gpio0_direction(pin, gpio::Direction::Output);
            gpio::set_gpio0(pin, !gpio::read_gpio0(pin));
          },
          GpioKind::Set => {
            gpio::set_gpio0_direction(pin, gpio::Direction::Output);
            gpio::set_gpio0(pin, value);
          },
          GpioKind::Get => {
            gpio::set_gpio0_direction(pin, gpio::Direction::Input);
            println!("gpio pin {} is {}", pin, if gpio::read_gpio0(pin) { "High" } else { "Low" } );
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
    cmd!("invcache", "", (self, _args) -> {
      csr_reg::invalidate_d_cache()
    }),
    cmd!("mtimer", "reads mtime/mtimecmp", (self, _args) -> {
      println!("mtimer:    0x{:016x}", timer::get_mtimer());
      println!("mtimercmp: 0x{:016x}", timer::get_timercmp());
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
      print_csr_regs();
    }),
    cmd!("mstatuss", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr_reg::set_mstatus(val);
      }
    }),
    cmd!("mstatusc", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr_reg::clear_mstatus(val);
      }
    }),
    cmd!("mies", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr_reg::set_mie(val);
      }
    }),
    cmd!("miec", "(val: usize)", (self, args) -> {
      args!(args, (val: usize));
      unsafe{
        csr_reg::clear_mie(val);
      }
    }),
    cmd!("plicd", "", (self, _args) -> {
      unsafe{
        plic::disp();
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
];


#[no_mangle]
pub extern "C" fn bl_rust_main() {
 
    timer::mdelay(250);
    unsafe {
        uart::console_init();
    }
    timer::mdelay(250);
    uart::print("\n\n\nBooted into firmware\nInitialized uart to 115200\n");
    unsafe{
      // set pinmux to enable output of LED pin
      mmio_write_32!(0x03001074, 0x3);
      gpio::set_gpio0_direction(29, gpio::Direction::Output);
    }
    uart::print("Connfigured pinmux(LED pin 29)\n");

    uart::print("Enabling interrupts\n");
    unsafe{
      csr_reg::enable_timer_interrupt();
      csr_reg::enable_interrupts();
      // trigger an interrupt NOW
      timer::set_timercmp(0);


      // plic is seen as a single external interrupt source
      csr_reg::enable_external_interrupt();
      // all enabled interrupts allowed
      plic::mint_threshhold(0);


      //--------------- timer 2 initialization ----------------------
      // timer 0 interrupt number
      plic::set_priority(79, 1);
      plic::enable_m_interrupt(79);

      // initialize timer0
      timer::mm::set_mode(timer::mm::TIMER0, timer::mm::TimerMode::Count);
      // half second
      timer::mm::set_load_value(timer::mm::TIMER0, timer::SYS_COUNTER_FREQ_IN_SECOND as u32 / 4);
      timer::mm::set_enabled(timer::mm::TIMER0, true);
      //-------------------------------------


      //------------- timer 1 initialization ----------------------
      plic::set_priority(80, 1);
      plic::enable_m_interrupt(80);


      // initialize timer1
      timer::mm::set_mode(timer::mm::TIMER1, timer::mm::TimerMode::Count);
      // second
      timer::mm::set_load_value(timer::mm::TIMER1, timer::SYS_COUNTER_FREQ_IN_SECOND as u32);
      // timer::mm::set_enabled(timer::mm::TIMER1, true);
      //-------------------------------------
    }
    uart::print("Interrupts enabled\n");

    uart::print("Starting console\n");

    let mut buffer = [0; 512];
    'next_cmd: loop {
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
