pub const TOP_BASE: u32 = 0x03000000;
pub const REG_GP_REG2: u32 = TOP_BASE + 0x88;

pub fn odelay(clk: u64){
    let start = get_mtimer();
    let delta_d = clk;
    while get_mtimer().wrapping_sub(start) < delta_d {}
}

pub fn udelay(usec: u64) {
    let start = get_mtimer();
    let delta_d = usec * SYS_COUNTER_FREQ_IN_US;
    while get_mtimer().wrapping_sub(start) < delta_d {}
}

#[inline(never)]
pub fn mdelay(msec: u64) {
    udelay(msec * 1000);
}

pub const SYS_COUNTER_FREQ_IN_SECOND: u64 = 25000000;
pub const SYS_COUNTER_FREQ_IN_US: u64 = SYS_COUNTER_FREQ_IN_SECOND / 1000000;

pub fn get_mtimer() -> u64 {
    let val: u64;
    unsafe {
        core::arch::asm!(
          "csrr  {val}, 0xc01",
          val = out(reg) val,
        );
    }
    val
}

pub const CLINT_BASE: u32 = 0x74000000;
/* CLINT */
pub const CLINT_MTIMECMPL0: u32 = CLINT_BASE + 0x4000;
pub const CLINT_MTIMECMPH0: u32 = CLINT_BASE + 0x4004;


pub const CLINT_STIMECMPL0: u32 = CLINT_BASE + 0xD000;
pub const CLINT_STIMECMPH0: u32 = CLINT_BASE + 0xD004;

pub fn get_mtimercmp() -> u64 {
    unsafe {
        let low = (CLINT_MTIMECMPL0 as *mut u32).read_volatile() as u64;
        let high = (CLINT_MTIMECMPH0 as *mut u32).read_volatile() as u64;
        low | (high << 32)
    }
}

pub unsafe fn set_mtimercmp(val: u64) {
    (CLINT_MTIMECMPH0 as *mut u32).write_volatile(u32::MAX);
    (CLINT_MTIMECMPL0 as *mut u32).write_volatile(val as u32);
    (CLINT_MTIMECMPH0 as *mut u32).write_volatile((val >> 32) as u32);
}

pub unsafe fn add_mtimercmp(val: u64) {
    set_mtimercmp(get_mtimercmp().wrapping_add(val));
}

pub fn get_stimercmp() -> u64 {
    unsafe {
        let low = (CLINT_STIMECMPL0 as *mut u32).read_volatile() as u64;
        let high = (CLINT_STIMECMPH0 as *mut u32).read_volatile() as u64;
        low | (high << 32)
    }
}

pub unsafe fn set_stimercmp(val: u64) {
    (CLINT_STIMECMPH0 as *mut u32).write_volatile(u32::MAX);
    (CLINT_STIMECMPL0 as *mut u32).write_volatile(val as u32);
    (CLINT_STIMECMPH0 as *mut u32).write_volatile((val >> 32) as u32);
}

pub unsafe fn add_stimercmp(val: u64) {
    set_stimercmp(get_stimercmp().wrapping_add(val));
}

pub mod mm {
    use core::ptr::addr_of_mut;

    use crate::mmio::TIMERS;


    #[repr(C)]
    pub struct Timers{
        pub timers: [Timer; 8],
        pub int_status: u32,
        pub eio: u32,
        pub raw_int_status: u32,
    }

    #[repr(C)]
    pub struct Timer {
        pub load: u32,
        pub curr: u32,
        pub ctrl: u32,
        pub eio: u32,
        pub int_stat: u32,
    }

    pub enum TimerMode {
        Free,
        Count,
    }


    pub unsafe fn set_load_value(timer: *mut Timer, value: u32) {
        addr_of_mut!((*timer).load).write_volatile(value);
    }

    pub unsafe fn get_load_value(timer: *mut Timer) -> u32 {
        addr_of_mut!((*timer).load).read_volatile()
    }

    pub unsafe fn get_curr_value(timer: *mut Timer) -> u32 {
        addr_of_mut!((*timer).curr).read_volatile()
    }

    pub unsafe fn get_masked(timer: *mut Timer) -> bool {
        addr_of_mut!((*timer).ctrl).read_volatile() & 0b100 != 0
    }

    pub unsafe fn set_masked(timer: *mut Timer, masked: bool) {
        let val = addr_of_mut!((*timer).ctrl).read_volatile();
        if masked {
            addr_of_mut!((*timer).ctrl).write_volatile(val | 0b100);
        } else {
            addr_of_mut!((*timer).ctrl).write_volatile(val & !0b100);
        }
    }

    pub unsafe fn get_mode(timer: *mut Timer) -> TimerMode {
        if addr_of_mut!((*timer).ctrl).read_volatile() & 0b10 == 0 {
            TimerMode::Free
        } else {
            TimerMode::Count
        }
    }

    pub unsafe fn set_mode(timer: *mut Timer, mode: TimerMode) {
        let val = addr_of_mut!((*timer).ctrl).read_volatile();
        match mode {
            TimerMode::Free => addr_of_mut!((*timer).ctrl).write_volatile(val & !0b10),
            TimerMode::Count => addr_of_mut!((*timer).ctrl).write_volatile(val | 0b10),
        }
    }

    pub unsafe fn get_enabled(timer: *mut Timer) -> bool {
        addr_of_mut!((*timer).ctrl).read_volatile() & 0b1 != 0
    }

    pub unsafe fn set_enabled(timer: *mut Timer, enabled: bool) {
        let val = addr_of_mut!((*timer).ctrl).read_volatile();
        if enabled {
            addr_of_mut!((*timer).ctrl).write_volatile(val | 0b1);
        } else {
            addr_of_mut!((*timer).ctrl).write_volatile(val & !0b1);
        }
    }

    pub unsafe fn clear_int(timer: *mut Timer) {
        addr_of_mut!((*timer).eio).read_volatile();
    }

    pub unsafe fn get_int_status(timer: *mut Timer) -> u32 {
        addr_of_mut!((*timer).int_stat).read_volatile()
    }

    pub unsafe fn get_all_int_status() -> u32 {
        addr_of_mut!((*TIMERS).int_status).read_volatile()
    }

    pub unsafe fn clear_all_int() {
        addr_of_mut!((*TIMERS).eio).read_volatile();
    }

    pub unsafe fn get_all_raw_int_status() -> u32 {
        addr_of_mut!((*TIMERS).raw_int_status).read_volatile()
    }
}
