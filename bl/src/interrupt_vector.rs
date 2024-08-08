use core::{ptr::NonNull, sync::atomic::AtomicPtr};

use milkv_rs::{csr, mem, plic, riscv::register::satp::Mode, timer};

use crate::{print, println};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrapFrame {
    pub ret: usize,
    pub regs: [usize; 31],
    pub mstatus: usize,
}

core::arch::global_asm!(
    r#"
    .globl  mtrap_vector

    .section .text.mtrap_vector,"ax",@progbits
    .globl mtrap_vector

    .balign 4
    mtrap_vector:
        csrrw sp, mscratch, sp
        addi sp, sp, -8 * (31 + 1 + 1)
        sd x1, 1 * 8( sp )
        # sd x2, 2 * 8( sp )
        sd x3, 3 * 8( sp )
        sd x4, 4 * 8( sp )
        sd x5, 5 * 8( sp )
        sd x6, 6 * 8( sp )
        sd x7, 7 * 8( sp )
        sd x8, 8 * 8( sp )
        sd x9, 9 * 8( sp )
        sd x10, 10 * 8( sp )
        sd x11, 11 * 8( sp )
        sd x12, 12 * 8( sp )
        sd x13, 13 * 8( sp )
        sd x14, 14 * 8( sp )
        sd x15, 15 * 8( sp )
        sd x16, 16 * 8( sp )
        sd x17, 17 * 8( sp )
        sd x18, 18 * 8( sp )
        sd x19, 19 * 8( sp )
        sd x20, 20 * 8( sp )
        sd x21, 21 * 8( sp )
        sd x22, 22 * 8( sp )
        sd x23, 23 * 8( sp )
        sd x24, 24 * 8( sp )
        sd x25, 25 * 8( sp )
        sd x26, 26 * 8( sp )
        sd x27, 27 * 8( sp )
        sd x28, 28 * 8( sp )
        sd x29, 29 * 8( sp )
        sd x30, 30 * 8( sp )
        sd x31, 31 * 8( sp )

        csrr t0, mstatus
        sd t0, 32 * 8( sp )

        addi a0, sp, 0
        csrr a1, mcause
        csrr a2, mepc
        csrr a3, mtval

        # test if asynchronous
        srli t0, a1, 64 - 1		/* MSB of mcause is 1 if handing an asynchronous interrupt - shift to LSB to clear other bits. */
        beq t0, x0, handle_synchronous		/* Branch past interrupt handing if not asynchronous. */
        	

    handle_asynchronous:
        sd a2, 0( sp )
        jal mtrap_handler
        j return

    handle_synchronous:
        addi a2, a2, 4
        sd a2, 0( sp )
        jal mtrap_handler


    return:

        ld t0, 0(sp)
        csrw mepc, t0

        ld t0, 32 * 8(sp)
        csrw mstatus, t0

        
        ld x1, 1 * 8( sp )
        # ld x2, 2 * 8( sp )
        ld x3, 3 * 8( sp )
        ld x4, 4 * 8( sp )
        ld x5, 5 * 8( sp )
        ld x6, 6 * 8( sp )
        ld x7, 7 * 8( sp )
        ld x8, 8 * 8( sp )
        ld x9, 9 * 8( sp )
        ld x10, 10 * 8( sp )
        ld x11, 11 * 8( sp )
        ld x12, 12 * 8( sp )
        ld x13, 13 * 8( sp )
        ld x14, 14 * 8( sp )
        ld x15, 15 * 8( sp )
        ld x16, 16 * 8( sp )
        ld x17, 17 * 8( sp )
        ld x18, 18 * 8( sp )
        ld x19, 19 * 8( sp )
        ld x20, 20 * 8( sp )
        ld x21, 21 * 8( sp )
        ld x22, 22 * 8( sp )
        ld x23, 23 * 8( sp )
        ld x24, 24 * 8( sp )
        ld x25, 25 * 8( sp )
        ld x26, 26 * 8( sp )
        ld x27, 27 * 8( sp )
        ld x28, 28 * 8( sp )
        ld x29, 29 * 8( sp )
        ld x30, 30 * 8( sp )
        ld x31, 31 * 8( sp )
        addi sp, sp, 8 * (31 + 1 + 1)
        csrrw sp, mscratch, sp

        mret
    "#
);

static PLIC_HANDLERS: [AtomicPtr<()>; milkv_rs::plic::MAX_INT_ID] =
    [const { AtomicPtr::new(core::ptr::null_mut()) }; milkv_rs::plic::MAX_INT_ID];

#[no_mangle]
pub extern "C" fn mtrap_handler(frame: &mut TrapFrame, mcause: usize, mepc: usize, mtval: usize) {
    let sync = mcause & (1 << 63) == 0;
    let code = mcause & !(1 << 63);
    if sync {
        let desc = match code {
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
                // csr::
                println!(
                    "\nEnv call from M-mode hardid: \"{}\"... returning",
                    csr::mhartid()
                );
                // println!("{:#?}", frame);
                // timer::mdelay(6000);
                return;
            }
            12 | 13 | 15 =>{
                let desc = match code{
                    12 => "Instruction page fault",
                    13 => "Page fault on load",
                    15 => "Page fault on store",
                    _ => "",
                };

                let stap = milkv_rs::riscv::register::satp::read();
                println!("{:?}, {:?}, 0x{:x?}", stap.mode(), stap.asid(), stap.ppn());

                unsafe fn print_thing(table: &milkv_rs::mem::PageTable, level: usize){
                    for entry in &table.entries{
                        if entry.valid(){
                            for _ in 0..level{
                                print!(" ");
                            }
                            println!("{:?}:{:x?}", entry as *const mem::PageTableEntry, entry);
                            if entry.perms() == 0{
                                print_thing(&*(((entry.ppn() << 12) as *const mem::PageTable)), level+1)
                            }
                        }
                    }
                }
                if stap.mode() != Mode::Bare{
                    unsafe{
                        print_thing(&*(((stap.ppn() << 12) as *const mem::PageTable)), 0);
                    }
                }

                println!("\n\n\n{desc}:\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, mtval: 0x{mtval:016x}, \nCannot continue resetting\n\n");
                unsafe { milkv_rs::reset() }
            } 
            _ => "Unknown exception",
        };
        println!("\n\n\n{desc}:\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, mtval: 0x{mtval:016x}, \nCannot continue resetting\n\n");
        unsafe { milkv_rs::reset() }
    } else {
        match code {
            0x7 => {
                use milkv_rs::*;
                unsafe{
                    gpio::set_gpio(mmio::GPIO0, 29, !gpio::read_gpio(mmio::GPIO0, 29));
                }
                println!("pc: 0x{mepc:x}");
                //mtimer > mtimercmp
                unsafe {
                    // 250 ms
                    timer::add_mtimercmp(timer::SYS_COUNTER_FREQ_IN_US * 250 * 1000);
                }
                //TODO... make this do something else LOL probably task switching or something funny
            }
            0xb => {
                let pending = unsafe { plic::mclaim_int() };
                use core::sync::atomic::Ordering;
                if pending != 0 {
                    if let Some(ptr) = PLIC_HANDLERS
                        .get(pending as usize)
                        .and_then(|v| NonNull::new(v.load(Ordering::Acquire)))
                    {
                        let func: fn() = unsafe { core::mem::transmute(ptr) };
                        func()
                    } else {
                        println!("\n\n\nplic: 0x{pending:016x}, mtval: 0x{mtval:016x}\nUnknown plic interrupt value. Cannot continue resetting\n\n");
                        unsafe { milkv_rs::reset() }
                    }
                    unsafe {
                        plic::mint_complete(pending);
                    }
                } else {
                    println!("\n\n\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, mtval: 0x{mtval:016x}\nPlic Interrupt but no pending interrupt found? Cannot continue resetting\n\n");
                    unsafe { milkv_rs::reset() }
                }
            }
            _ => {
                println!("\n\n\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, mtval: 0x{mtval:016x}\nCannot continue resetting\n\n");
                unsafe { milkv_rs::reset() }
            }
        }
    }
}

pub fn add_plic_handler(int: u32, handler: fn()) {
    use core::sync::atomic::Ordering;
    PLIC_HANDLERS[int as usize].store(unsafe { core::mem::transmute(handler) }, Ordering::Release);
}
