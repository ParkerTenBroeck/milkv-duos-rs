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