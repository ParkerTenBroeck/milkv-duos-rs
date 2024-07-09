#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]

use crate::{
    // ddr::{AxiCtrl, AxiMons, Ddrc}, 
    gpio::GPIO, mmap::PARAM1_BASE, platform::fip_param1, plic::Plic, system::SystemControl, timer::mm::{Timer, Timers}, uart::Uart, watchdog::WatchDog};

#[macro_export]
macro_rules! mmio_write_32 {
    ($ptr:expr, $val:expr) => {
        ($ptr as *mut u32).write_volatile($val)
    };
}
#[macro_export]
macro_rules! mmio_read_32 {
    ($ptr:expr) => {
        ($ptr as *const u32).read_volatile()
    };
}

/*
 * SoC memory map
 */
pub const SEC_SUBSYS_BASE: usize = 0x02000000;
pub const SEC_CRYPTODMA_BASE: usize = SEC_SUBSYS_BASE + 0x00060000;
pub const SEC_FAB_FIREWALL: usize = SEC_SUBSYS_BASE + 0x00090000;
pub const SEC_DDR_FIREWALL: usize = SEC_SUBSYS_BASE + 0x000A0000;
pub const SEC_SYS_BASE: usize = SEC_SUBSYS_BASE + 0x000B0000;
pub const SEC_EFUSE_BASE: usize = SEC_SUBSYS_BASE + 0x000C0000;

pub const TOP_BASE: usize = 0x03000000;

/* eFuse  */
pub const EFUSE_BASE: usize = TOP_BASE + 0x00050000;







pub const SYSTEM_CONTROL: *mut SystemControl = 0x03000000 as *mut SystemControl;

pub const WDT0: *mut WatchDog = 0x03010000 as *mut WatchDog;
pub const WDT1: *mut WatchDog = 0x03011000 as *mut WatchDog;
pub const WDT2: *mut WatchDog = 0x03012000 as *mut WatchDog;
pub const RTCYS_WDT: *mut WatchDog = 0x0502D000 as *mut WatchDog;

pub const GPIO0: *mut GPIO = 0x03020000 as *mut GPIO;
pub const GPIO1: *mut GPIO = 0x03021000 as *mut GPIO;
pub const GPIO2: *mut GPIO = 0x03022000 as *mut GPIO;
pub const GPIO3: *mut GPIO = 0x03023000 as *mut GPIO;

pub const TIMERS: *mut Timers = 0x030A0000 as *mut Timers;    
pub const TIMER_BASE: usize = 0x030A0000;

pub const TIMER0: *mut Timer = (TIMER_BASE + 0 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER1: *mut Timer = (TIMER_BASE + 1 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER2: *mut Timer = (TIMER_BASE + 2 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER3: *mut Timer = (TIMER_BASE + 3 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER4: *mut Timer = (TIMER_BASE + 4 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER5: *mut Timer = (TIMER_BASE + 5 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER6: *mut Timer = (TIMER_BASE + 6 * core::mem::size_of::<Timer>()) as *mut Timer;
pub const TIMER7: *mut Timer = (TIMER_BASE + 7 * core::mem::size_of::<Timer>()) as *mut Timer;


pub const UART0: *mut Uart = 0x04140000 as *mut Uart;
pub const UART1: *mut Uart = 0x04150000 as *mut Uart;
pub const UART2: *mut Uart = 0x04160000 as *mut Uart;
pub const UART3: *mut Uart = 0x04170000 as *mut Uart;
pub const UART4: *mut Uart = 0x041C0000 as *mut Uart;
pub const RTCSYS_UART: *mut Uart = 0x05022000 as *mut Uart;

pub const PARAM1: *const fip_param1 = PARAM1_BASE as *const fip_param1;



pub const PLIC: *mut Plic = 0x70000000 as *mut Plic;

// pub const DDRC: *mut Ddrc =  0x0800_4000 as *mut Ddrc;
// pub const AXI_CTRL: *mut AxiCtrl =  0x0800_4000 as *mut AxiCtrl;
// pub const AXI_MON: *mut AxiMons =  0x0800_8000 as *mut AxiMons;
// pub const DDR_GLOBAL: *mut () =  0x0800_A000 as *mut ();

