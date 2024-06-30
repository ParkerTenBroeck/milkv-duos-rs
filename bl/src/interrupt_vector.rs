use core::{ptr::NonNull, sync::atomic::AtomicPtr};

use milkv_rs::{gpio, mmio, plic, timer, uart};

use crate::println;

core::arch::global_asm!(
    r#"
    .globl  mtrap_vector

    .section .text.mtrap_vector,"ax",@progbits
    .globl mtrap_vector

    .balign 4
    mtrap_vector:
        # j trap_handler
        addi sp, sp, -8 * 29
        sd x1, 1 * 8( sp )
        sd x5, 2 * 8( sp )
        sd x6, 3 * 8( sp )
        sd x7, 4 * 8( sp )
        sd x8, 5 * 8( sp )
        sd x9, 6 * 8( sp )
        sd x10, 7 * 8( sp )
        sd x11, 8 * 8( sp )
        sd x12, 9 * 8( sp )
        sd x13, 10 * 8( sp )
        sd x14, 11 * 8( sp )
        sd x15, 12 * 8( sp )
        sd x16, 13 * 8( sp )
        sd x17, 14 * 8( sp )
        sd x18, 15 * 8( sp )
        sd x19, 16 * 8( sp )
        sd x20, 17 * 8( sp )
        sd x21, 18 * 8( sp )
        sd x22, 19 * 8( sp )
        sd x23, 20 * 8( sp )
        sd x24, 21 * 8( sp )
        sd x25, 22 * 8( sp )
        sd x26, 23 * 8( sp )
        sd x27, 24 * 8( sp )
        sd x28, 25 * 8( sp )
        sd x29, 26 * 8( sp )
        sd x30, 27 * 8( sp )
        sd x31, 28 * 8( sp )

        csrr t0, mstatus
        sd t0, 29 * 8( sp )

        csrr a0, mcause
        csrr a1, mepc
        csrr a2, mtval

        # test if asynchronous
        srli a2, a0, 64 - 1		/* MSB of mcause is 1 if handing an asynchronous interrupt - shift to LSB to clear other bits. */
        beq a2, x0, handle_synchronous		/* Branch past interrupt handing if not asynchronous. */
        	

    handle_asynchronous:
        sd a1, 0( sp )
        jal mtrap_handler
        j return

    handle_synchronous:
        addi a1, a1, 4
        sd a1, 0( sp )
        jal mtrap_handler


    return:

        ld t0, 0(sp)
        csrw mepc, t0

        ld t0, 29 * 8(sp)
        csrw mstatus, t0

        
        ld x1, 1 * 8( sp )
        ld x5, 2 * 8( sp )
        ld x6, 3 * 8( sp )
        ld x7, 4 * 8( sp )
        ld x8, 5 * 8( sp )
        ld x9, 6 * 8( sp )
        ld x10, 7 * 8( sp )
        ld x11, 8 * 8( sp )
        ld x12, 9 * 8( sp )
        ld x13, 10 * 8( sp )
        ld x14, 11 * 8( sp )
        ld x15, 12 * 8( sp )
        ld x16, 13 * 8( sp )
        ld x17, 14 * 8( sp )
        ld x18, 15 * 8( sp )
        ld x19, 16 * 8( sp )
        ld x20, 17 * 8( sp )
        ld x21, 18 * 8( sp )
        ld x22, 19 * 8( sp )
        ld x23, 20 * 8( sp )
        ld x24, 21 * 8( sp )
        ld x25, 22 * 8( sp )
        ld x26, 23 * 8( sp )
        ld x27, 24 * 8( sp )
        ld x28, 25 * 8( sp )
        ld x29, 26 * 8( sp )
        ld x30, 27 * 8( sp )
        ld x31, 28 * 8( sp ) 
        addi sp, sp, 8 * 29

        mret
    "#
);

static PLIC_HANDLERS: [AtomicPtr<()>; milkv_rs::plic::MAX_INT_ID] =
    [const { AtomicPtr::new(core::ptr::null_mut()) }; milkv_rs::plic::MAX_INT_ID];

#[no_mangle]
pub extern "C" fn mtrap_handler(mcause: usize, mepc: usize, mtval: usize) {
    let sync = mcause & (1 << 63) == 0;
    let code = mcause & !(1 << 63);
    if sync {
        let mepc = mepc - 1;
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
                println!("\nEnv call from M-mode... returning");
                return;
            }
            12 => "Instruction page fault",
            13 => "Page fault on load",
            15 => "Page fault on store",
            _ => "Unknown exception",
        };
        let ins = unsafe { (mepc as *const u32).read() };
        println!("\n\n\n{desc}:\nmcause: 0x{mcause:016x}, mepc: 0x{mepc:016x}, mtval: 0x{mtval:016x}, ins: 0x{ins:08x}\nCannot continue resetting\n\n");
        unsafe { milkv_rs::reset() }
    } else {
        match code {
            0x7 => {
                //mtimer > mtimercmp
                unsafe {
                    // 250 ms
                    timer::add_timercmp(timer::SYS_COUNTER_FREQ_IN_US * 250 * 1000);
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