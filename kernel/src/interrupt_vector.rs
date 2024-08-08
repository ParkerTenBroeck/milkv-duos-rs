use core::{ptr::NonNull, sync::atomic::AtomicPtr};

use milkv_rs::{csr, mmio, plic, timer, uart};

use crate::{mem, println};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrapFrame {
    pub pc: usize,
    pub regs: [usize; 31],
    pub sstatus: milkv_rs::riscv::register::sstatus::Sstatus,
}

core::arch::global_asm!(
    r#"
    .globl  strap_vector

    .section .text.strap_vector,"ax",@progbits
    .globl strap_vector

    .balign 4
    strap_vector:
        
        mv x3,sp
        csrw sscratch, sp
        la sp, KERNEL_CTX
        ld sp, 8(sp)
        addi sp, sp, -8 * (31 + 1 + 1)

        sd x1, 1 * 8( sp )
        
        csrr x1, sscratch
        sd x1, 2 * 8( sp )
        
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

        csrr t0, sstatus
        sd t0, 32 * 8( sp )

        addi a0, sp, 0
        csrr a1, scause
        csrr a2, sepc
        csrr a3, stval

        # test if asynchronous
        srli t0, a1, 64 - 1		/* MSB of scause is 1 if handing an asynchronous interrupt - shift to LSB to clear other bits. */
        beq t0, x0, handle_synchronous		/* Branch past interrupt handing if not asynchronous. */
        	

    handle_asynchronous:
        sd a2, 0( sp )
        jal strap_handler
        j return

    handle_synchronous:
        addi t0, a2, 4
        sd t0, 0( sp )
        jal strap_handler


    return:

        ld t0, 0(sp)
        csrw sepc, t0

        ld t0, 32 * 8(sp)
        csrw sstatus, t0

        
        ld x1, 1 * 8( sp )
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
        ld x2, 2 * 8( sp )

        sret
    "#
);

#[repr(C)]
struct InterruptHandler {
    func: AtomicPtr<()>,
    ctx: AtomicPtr<()>,
}

impl InterruptHandler {
    pub const fn new() -> Self {
        Self {
            func: AtomicPtr::new(0 as *mut ()),
            ctx: AtomicPtr::new(0 as *mut ()),
        }
    }
}

static PLIC_HANDLERS: [InterruptHandler; milkv_rs::plic::MAX_INT_ID] =
    [const { InterruptHandler::new() }; milkv_rs::plic::MAX_INT_ID];

static TIMER_INT_HANDLER: InterruptHandler = InterruptHandler::new();
static ECALL_HANDLERS: [InterruptHandler; 500] = [const { InterruptHandler::new() }; 500];

use milkv_rs::riscv::register::*;

#[no_mangle]
pub extern "C" fn strap_handler(
    frame: &mut TrapFrame,
    scause: scause::Scause,
    sepc: usize,
    stval: usize,
) {
    match scause.cause() {
        scause::Trap::Exception(exc) => {
            let desc = match exc {
                scause::Exception::InstructionMisaligned => "Instruction address misaligned",
                scause::Exception::InstructionFault => "Instruction access fault",
                scause::Exception::IllegalInstruction => "Illegal instruction",
                scause::Exception::Breakpoint => "Breakpoint",
                scause::Exception::LoadMisaligned => "Load address misaligned",
                scause::Exception::LoadFault => "Load access fault",
                scause::Exception::StoreMisaligned => "Store address misaligned",
                scause::Exception::StoreFault => "Store access fault",
                scause::Exception::UserEnvCall //=> "Env call from U-mode",
                | scause::Exception::SupervisorEnvCall => {
                    if let Some(handler) = ECALL_HANDLERS.get(frame.regs[17-1]) {
                        use core::sync::atomic::Ordering;
                        let ptr = handler.func.load(Ordering::Relaxed);
                        if !ptr.is_null(){
                            let func: fn(
                                ctx: *mut (),
                                frame: &mut TrapFrame,
                                scause: scause::Scause,
                                mepc: usize,
                                mtval: usize,
                            ) = unsafe { core::mem::transmute(ptr) };
                            func(
                                handler.ctx.load(Ordering::Relaxed),
                                frame,
                                scause,
                                sepc,
                                stval,
                            );
                            return;
                        }
                    }
                    println!("\n\n\nplic: 0x{:016x}, mtval: 0x{stval:016x}\nUnknown ecall value. Cannot continue resetting\n\n", frame.regs[17-1]);
                    unsafe { milkv_rs::reset() }
                }//=> "Env call from S-mode",
                scause::Exception::InstructionPageFault => "Instruction page fault",
                scause::Exception::LoadPageFault => "Page fault on load",
                scause::Exception::StorePageFault => "Page fault on store",
                scause::Exception::Unknown => "Unknown exception",
            };
            let ins = 12;
            unsafe { (sepc as *const u32).read_unaligned() };
            println!(
                "satp: {:08x}",
                milkv_rs::riscv::register::satp::read().bits()
            );
            // let p = (milkv_rs::riscv::register::satp::read().ppn() << 12) as *mut mem::PageTable;
            // let p = unsafe{&mut *p};
            // println!("{p:#08x?}");
            println!("\n\n\n{desc}:\nmcause: 0x{:?}, mepc: 0x{sepc:016x}, mtval: 0x{stval:016x}, ins: 0x{ins:08x}\nCannot continue resetting\n\n", scause.bits());

            unsafe { reset() }
        }
        scause::Trap::Interrupt(int) => match int {
            scause::Interrupt::SupervisorSoft => {
                // use core::sync::atomic::Ordering;
                println!("\n\n\nmcause: 0x{:?}, mepc: 0x{sepc:016x}, mtval: 0x{stval:016x}\nCannot continue resetting\n\n", scause.cause());
                unsafe { milkv_rs::reset() }
            }
            scause::Interrupt::SupervisorTimer => {
                use core::sync::atomic::Ordering;
                let ptr = TIMER_INT_HANDLER.func.load(Ordering::Relaxed);
                if !ptr.is_null() {
                    let func: fn(
                        ctx: *mut (),
                        frame: &mut TrapFrame,
                        scause: scause::Scause,
                        sepc: usize,
                        stval: usize,
                    ) = unsafe { core::mem::transmute(ptr) };
                    func(
                        TIMER_INT_HANDLER.ctx.load(Ordering::Relaxed),
                        frame,
                        scause,
                        sepc,
                        stval,
                    );
                    return;
                }
                println!("\n\n\nmcause: 0x{:?}, mepc: 0x{sepc:016x}, mtval: 0x{stval:016x}\nCannot continue resetting\n\n", scause.cause());
                unsafe { milkv_rs::reset() }
            }
            scause::Interrupt::SupervisorExternal => {
                use core::sync::atomic::Ordering;
                let pending = unsafe { plic::sclaim_int() };
                if pending != 0 {
                    if let Some(handler) = PLIC_HANDLERS.get(pending as usize) {
                        let func: fn(
                            ctx: *mut (),
                            frame: &mut TrapFrame,
                            scause: scause::Scause,
                            mepc: usize,
                            mtval: usize,
                        ) = unsafe { core::mem::transmute(handler.func.load(Ordering::Relaxed)) };
                        func(
                            handler.ctx.load(Ordering::Relaxed),
                            frame,
                            scause,
                            sepc,
                            stval,
                        )
                    } else {
                        println!("\n\n\nplic: 0x{pending:016x}, mtval: 0x{stval:016x}\nUnknown plic interrupt value. Cannot continue resetting\n\n");
                        unsafe { milkv_rs::reset() }
                    }
                    unsafe {
                        plic::sint_complete(pending);
                    }
                }
            }
            scause::Interrupt::Unknown => {
                println!("\n\n\nmcause: 0x{:?}, mepc: 0x{sepc:016x}, mtval: 0x{stval:016x}\nCannot continue resetting\n\n", scause.cause());
                unsafe { milkv_rs::reset() }
            }
        },
    }
}

pub unsafe fn add_plic_handler<T>(
    int: u32,
    ctx: *mut T,
    handler: fn(
        ctx: *mut T,
        frame: &mut TrapFrame,
        scause: scause::Scause,
        sepc: usize,
        stval: usize,
    ),
) {
    use core::sync::atomic::Ordering;
    PLIC_HANDLERS[int as usize]
        .ctx
        .store(ctx.cast(), Ordering::Release);
    PLIC_HANDLERS[int as usize]
        .func
        .store(unsafe { core::mem::transmute(handler) }, Ordering::Release);
}

pub unsafe fn add_timer_handler<T>(
    ctx: *mut T,
    handler: fn(
        ctx: *mut T,
        frame: &mut TrapFrame,
        scause: scause::Scause,
        sepc: usize,
        stval: usize,
    ),
) {
    use core::sync::atomic::Ordering;
    TIMER_INT_HANDLER.ctx.store(ctx.cast(), Ordering::Release);
    TIMER_INT_HANDLER
        .func
        .store(unsafe { core::mem::transmute(handler) }, Ordering::Release);
}

pub unsafe fn add_ecall_handler<T>(
    call_id: usize,
    ctx: *mut T,
    handler: fn(
        ctx: *mut T,
        frame: &mut TrapFrame,
        scause: scause::Scause,
        sepc: usize,
        stval: usize,
    ),
) {
    use core::sync::atomic::Ordering;
    ECALL_HANDLERS[call_id].ctx.store(ctx.cast(), Ordering::Release);
    ECALL_HANDLERS[call_id]
        .func
        .store(unsafe { core::mem::transmute(handler) }, Ordering::Release);
}

pub unsafe fn reset() -> ! {
    // while let Some(v) = crate::uart_sys::TX.pop(){
    //     crate::uart::print_c(v);
    // }
    crate::uart::flush();
    // milkv_rs::csr::enable_interrupts();
    crate::timer::mdelay(500);
    // milkv_rs::csr::disable_interrupts();

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
