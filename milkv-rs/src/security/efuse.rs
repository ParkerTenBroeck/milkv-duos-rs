use crate::{
    mmio::{EFUSE_BASE, SEC_SUBSYS_BASE},
    mmio_read_32, mmio_write_32,
};

pub const SEC_EFUSE_BASE: usize = SEC_SUBSYS_BASE + 0x000C0000;
pub const SEC_EFUSE_SHADOW_REG: usize = SEC_EFUSE_BASE + 0x100;
pub const EFUSE_SIZE: usize = 0x100;
pub const EFUSE_STATUS: usize = 0x010;
pub const EFUSE_MODE: usize = 0x000;
pub const EFUSE_ADR: usize = 0x004;
pub const EFUSE_RD_DATA: usize = 0x00c;

pub const EFUSE_LDR_DES_KEY_REG: usize = SEC_EFUSE_SHADOW_REG + 0xD8;

pub const EFUSE_KPUB_HASH_REG: usize = SEC_EFUSE_SHADOW_REG + 0xA8;

pub const EFUSE_SCS_CONFIG_REG: usize = SEC_EFUSE_SHADOW_REG + 0xA0;
pub const BIT_SCS_ENABLE: usize = 0;
pub const BIT_TEE_SCS_ENABLE: usize = 2;
pub const BIT_BOOT_LOADER_ENCRYPTION: usize = 6;

pub const EFUSE_W_LOCK0_REG: usize = EFUSE_BASE + 0x198;
pub const BIT_FTSN3_LOCK: usize = 3;
pub const BIT_FTSN4_LOCK: usize = 4;

pub enum EfuseError{
    EfusePowerOnFail,
    EfuseFtsn3LockFailed,
    EfuseFtsn4LockFailed,
    EfuseRefreshShadowFail,
    LockEfuseChipsnFail,
    EfusePowerOffFail
}

pub unsafe fn lock_efuse() -> Result<(), EfuseError> {
    let value = mmio_read_32!(EFUSE_W_LOCK0_REG);

    if let Err(_) = efuse_power_on() {
        return Err(EfuseError::EfusePowerOnFail);
    }

    if (value & (0x1 << BIT_FTSN3_LOCK)) == 0 {
        if let Err(_) = efuse_program_bit(0x26, BIT_FTSN3_LOCK as u32) {
            return Err(EfuseError::EfuseFtsn3LockFailed);
        }
    }

    if (value & (0x1 << BIT_FTSN4_LOCK)) == 0 {
        if let Err(_) = efuse_program_bit(0x26, BIT_FTSN4_LOCK as u32) {
            return Err(EfuseError::EfuseFtsn4LockFailed);
        }
    }

    if let Err(_) = efuse_refresh_shadow() {
        return Err(EfuseError::EfuseRefreshShadowFail);
    }

    let value = mmio_read_32!(EFUSE_W_LOCK0_REG);
    if ((value & (0x3 << BIT_FTSN3_LOCK)) >> BIT_FTSN3_LOCK) != 0x3 {
        return Err(EfuseError::LockEfuseChipsnFail)
    }

    if let Err(_) = efuse_power_off() {
        return Err(EfuseError::EfusePowerOffFail);
    }

    Ok(())
}

unsafe fn efuse_program_bit(addr: u32, bit: u32) -> Result<(), ()> {
    let w_addr = (bit << 7) | ((addr & 0x3F) << 1);
    let mut w_addr = w_addr as u16;
    efuse_wait_idle()?;

    mmio_write_32!(EFUSE_BASE + EFUSE_ADR, (w_addr & 0xFFF) as u32);
    mmio_write_32!(EFUSE_BASE + EFUSE_MODE, 0x14);

    efuse_wait_idle()?;

    w_addr |= 0x1;
    mmio_write_32!(EFUSE_BASE + EFUSE_ADR, (w_addr & 0xFFF) as u32);
    mmio_write_32!(EFUSE_BASE + EFUSE_MODE, 0x14);
    Ok(())
}

unsafe fn efuse_power_on() -> Result<(), ()> {
    efuse_wait_idle()?;
    mmio_write_32!(EFUSE_BASE + EFUSE_MODE, 0x10);
    Ok(())
}

unsafe fn efuse_wait_idle() -> Result<(), ()> {
    let start = crate::timer::get_mtimer();
    while {
        let status = mmio_read_32!(EFUSE_BASE + EFUSE_STATUS);

        if crate::timer::get_mtimer().wrapping_sub(start)
            > 250 * 1000 * crate::timer::SYS_COUNTER_FREQ_IN_US
        {
            return Err(());
        }

        status & 0x1 != 0
    } {}
    Ok(())
}

unsafe fn efuse_power_off() -> Result<(), ()> {
    efuse_wait_idle()?;
    mmio_write_32!(EFUSE_BASE + EFUSE_MODE, 0x18);
    Ok(())
}

unsafe fn efuse_refresh_shadow() -> Result<(), ()> {
    efuse_wait_idle()?;
    mmio_write_32!(EFUSE_BASE + EFUSE_MODE, 0x30);
    efuse_wait_idle()?;
    Ok(())
}
