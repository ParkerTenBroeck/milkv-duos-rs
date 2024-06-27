use crate::*;

#[panic_handler]
pub fn rust_panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    crate::uart::print("Rust panic... resetting\n");
    unsafe { reset() }
}




#[no_mangle]
pub extern "C" fn trap_handler(mcause: usize, mepc: usize) {
    let sync = mcause & (1 << 63) == 0; 
    let code = mcause & !(1 << 63);
    if sync {
        let mepc = mepc - 1;
        let desc = match code{
            0 => "Instruction address misaligned",
            1 => "Instruction access fault",
            2 => "Illegal instruction",
            3 => "Breakpoint",
            4 => "Load address misaligned",
            5 => "Load access fault",
            6 => "Store address mimsaligned",
            7 => "Store access fault",
            8 => "Env call from U-mode",
            9 => "Env call from S-mode",
            11 => {
                println!("\nEnv call from M-mode... returning");
                return;
            },
            12 => "Instruction page fault",
            13 => "Page fault on load",
            15 => "Page fault on store",
            _ => "Unknown exception",
        };
        let ins = unsafe{ (mepc as *const u32).read() };
        println!("\n\n\n{desc}:\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, ins: 0x{ins:08x}\nCannot continue resetting\n\n");
        unsafe { reset() }
    }else{
        println!("\n\n\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}\nCannot continue resetting\n\n");
        unsafe { reset() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    crate::timer::mdelay(500);
    crate::uart::flush();

    macro_rules! mmio_write_32 {
        ($ptr:expr, $val:expr) => {
            ($ptr as *mut u32).write_volatile($val)
        };
    }
    macro_rules! mmio_read_32 {
        ($ptr:expr) => {
            ($ptr as *const u32).read_volatile()
        };
    }
    // enable rtc wdt reset
    mmio_write_32!(0x050260E0, 0x0001); //enable rtc_core wathdog reset enable
    mmio_write_32!(0x050260C8, 0x0001); //enable rtc_core power cycle   enable

    // mmio_write_32(0x05025018,0x00FFFFFF); //Mercury rtcsys_rstn_src_sel
    mmio_write_32!(0x050250AC, 0x00000000); //cv181x rtcsys_rstn_src_sel
    mmio_write_32!(0x05025004, 0x0000AB18);
    mmio_write_32!(0x05025008, 0x00400040); //enable rtc_ctrl wathdog reset enable

    mmio_write_32!(0x03010004, 0x00000066); //config watch dog 166ms
    mmio_write_32!(0x0301001c, 0x00000020);
    mmio_write_32!(0x0301000c, 0x00000076);
    mmio_write_32!(0x03010000, 0x00000011);

    // wait pmu state to ON
    while mmio_read_32!(0x050260D4) != 0x00000003 {}
    mmio_write_32!(0x05025008, 0x00080008);

    core::hint::unreachable_unchecked()
}
