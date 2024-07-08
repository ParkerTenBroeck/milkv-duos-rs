#![allow(unused_parens)]

pub const RTC_SYS_BASE: u32 = 0x05000000;
pub const RTC_MACRO_BASE: u32 = (RTC_SYS_BASE + 0x00026400);
pub const RTC_CORE_SRAM_BASE: u32 = (RTC_SYS_BASE + 0x00026800);
pub const RTC_CORE_SRAM_SIZE: u32 = 0x0800; // 2KB
pub const RTC_IO_BASE: u32 = (RTC_SYS_BASE + 0x00027000);

pub const REG_RTC_CTRL_BASE: u32 = (RTC_SYS_BASE + 0x00025000);
pub const RTC_CTRL0_UNLOCKKEY: u32 = 0x4;
pub const RTC_CTRL0: u32 = 0x8;
pub const RTC_CTRL0_STATUS0: u32 = 0xC;
pub const RTCSYS_RST_CTRL: u32 = 0x18;
pub const RTC_FC_COARSE_EN: u32 = 0x40;
pub const RTC_FC_COARSE_CAL: u32 = 0x44;
pub const RTC_FC_FINE_EN: u32 = 0x48;
pub const RTC_FC_FINE_CAL: u32 = 0x50;
pub const RTC_POR_RST_CTRL: u32 = 0xAC;

pub const REG_RTC_BASE: u32 = (RTC_SYS_BASE + 0x00026000);
pub const RTC_ANA_CALIB: u32 = 0x0;
pub const RTC_SEC_PULSE_GEN: u32 = 0x4;
pub const RTC_EN_PWR_WAKEUP: u32 = 0xBC;
pub const RTC_EN_SHDN_REQ: u32 = 0xC0;
pub const RTC_EN_PWR_CYC_REQ: u32 = 0xC8;
pub const RTC_EN_WARM_RST_REQ: u32 = 0xCC;
pub const RTC_EN_PWR_VBAT_DET: u32 = 0xD0;
pub const RTC_EN_WDT_RST_REQ: u32 = 0xE0;
pub const RTC_EN_SUSPEND_REQ: u32 = 0xE4;
pub const RTC_PG_REG: u32 = 0xF0;
pub const RTC_ST_ON_REASON: u32 = 0xF8;
pub const RTC_ST_OFF_REASON: u32 = 0xFC;

pub const RTC_INFO0: u32 = (REG_RTC_BASE + 0x1C);
pub const RTC_INFO1: u32 = (REG_RTC_BASE + 0x20);
pub const RTC_INFO2: u32 = (REG_RTC_BASE + 0x24);
pub const RTC_INFO3: u32 = (REG_RTC_BASE + 0x28);

pub const REG_RTC_ST_ON_REASON: u32 = (REG_RTC_BASE + RTC_ST_ON_REASON);

pub const RTCSYS_F32KLESS_BASE: u32 = (RTC_SYS_BASE + 0x0002A000);

pub const RTC_INTERNAL_32K: u32 = 0;
pub const RTC_EXTERNAL_32K: u32 = 1;