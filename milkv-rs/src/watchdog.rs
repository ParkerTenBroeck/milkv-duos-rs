

#[repr(C)]
pub struct WatchDog{
    /// Control register
    cr: u32,
    /// Timeout range register
    torr: u32,
    /// Current counter value register
    ccvr: u32,
    /// Counter restart register
    crr: u32,
    /// Interrupt status register
    stat: u32,
    /// Interrupt clear register
    eoi: u32,
    /// Timeout count
    toc: u32,
}