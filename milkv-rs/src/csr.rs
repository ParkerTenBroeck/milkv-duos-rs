use core::arch;

#[allow(non_upper_case_globals)]
pub const mxstatus: u32 = 0x7C0;
#[allow(non_upper_case_globals)]
pub const mhcr: u32 = 0x7C1;
#[allow(non_upper_case_globals)]
pub const mcor: u32 = 0x7C2;
#[allow(non_upper_case_globals)]
pub const mccr2: u32 = 0x7C3;
#[allow(non_upper_case_globals)]
pub const mcer2: u32 = 0x7C4;
#[allow(non_upper_case_globals)]
pub const mhint: u32 = 0x7C5;
#[allow(non_upper_case_globals)]
pub const mrmr: u32 = 0x7C6;
#[allow(non_upper_case_globals)]
pub const mrvbr: u32 = 0x7C7;


pub fn mcycle() -> u64 {
    unsafe{
        let val;
        core::arch::asm!("
            #mcycle
            csrr {0}, mcycle
        ", out(reg) val);
        val
    }
}

// pub fn invalidate_d_cache() {
//     unsafe {
//         core::arch::asm!("
//             li x3, 0x30013
//             csrs {mcor}, x3
//         ",
//         mcor = const mcor
//         )
//     }
// }

pub fn misa() -> u32 {
    unsafe {
        let misa;
        core::arch::asm!(
            "csrr {0}, misa",
            out(reg) misa
        );
        misa
    }
}

pub fn mvendorid() -> u32 {
    unsafe {
        let mvendorid;
        core::arch::asm!(
            "csrr {0}, mvendorid",
            out(reg) mvendorid
        );
        mvendorid
    }
}

pub fn marchid() -> usize {
    unsafe {
        let marchid;
        core::arch::asm!(
            "csrr {0}, marchid",
            out(reg) marchid
        );
        marchid
    }
}

pub fn mimpid() -> usize {
    unsafe {
        let mimpid;
        core::arch::asm!(
            "csrr {0}, mimpid",
            out(reg) mimpid
        );
        mimpid
    }
}

pub fn mhartid() -> usize {
    unsafe {
        let mhartid;
        core::arch::asm!(
            "csrr {0}, mhartid",
            out(reg) mhartid
        );
        mhartid
    }
}

pub fn read_mstatus() -> usize {
    unsafe {
        let val: usize;
        core::arch::asm!(
            "csrr {0}, mstatus",
            out(reg) val
        );
        val
    }
}

pub fn read_mip() -> usize {
    unsafe {
        let val: usize;
        core::arch::asm!(
            "csrr {0}, mip",
            out(reg) val
        );
        val
    }
}

pub fn read_mie() -> usize {
    unsafe {
        let val: usize;
        core::arch::asm!(
            "csrr {0}, mie",
            out(reg) val
        );
        val
    }
}

pub unsafe fn enable_interrupts() {
    core::arch::asm!(
        "csrs mstatus, {0}",
        in(reg) 0b1000,
    );
}

pub unsafe fn disable_interrupts() {
    core::arch::asm!(
        "csrc mstatus, {0}",
        in(reg) 0b1000,
    );
}

pub unsafe fn set_mstatus(val: usize) {
    core::arch::asm!(
        "csrs mstatus, {0}",
        in(reg) val
    )
}

pub unsafe fn clear_mstatus(val: usize) {
    core::arch::asm!(
        "csrc mstatus, {0}",
        in(reg) val
    )
}

pub unsafe fn set_mie(val: usize) {
    core::arch::asm!(
        "csrs mie, {0}",
        in(reg) val
    )
}

pub unsafe fn clear_mie(val: usize) {
    core::arch::asm!(
        "csrc mie, {0}",
        in(reg) val
    )
}

pub unsafe fn enable_timer_interrupt() {
    core::arch::asm!(
        "csrs mie, {0}",
        in(reg) 0b10000000
    )
}

pub unsafe fn disable_timer_interrupt() {
    core::arch::asm!(
        "csrc mie, {0}",
        in(reg) 0b10000000
    )
}

pub unsafe fn disable_external_interrupt() {
    core::arch::asm!(
        "csrc mie, {0}",
        in(reg) 0b100000000000
    )
}

pub unsafe fn enable_external_interrupt() {
    core::arch::asm!(
        "csrs mie, {0}",
        in(reg) 0b100000000000
    )
}

pub fn medeleg() -> usize {
    unsafe {
        let val: usize;
        core::arch::asm!(
            "csrr {0}, medeleg",
            out(reg) val
        );
        val
    }
}

pub fn mideleg() -> usize {
    unsafe {
        let val: usize;
        core::arch::asm!(
            "csrr {0}, mideleg",
            out(reg) val
        );
        val
    }
}
