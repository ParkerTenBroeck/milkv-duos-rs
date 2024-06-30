use core::ptr::addr_of_mut;

pub const GPIO0: usize = 0x03020000;
pub const GPIO1: usize = 0x03021000;
pub const GPIO2: usize = 0x03022000;
pub const GPIO3: usize = 0x03023000;

#[repr(C)]
pub struct GPIO {
    /// data register
    dr: u32,
    /// Data direction register (0 input, 1 output)
    ddr: u32,
    _space0: [u8; 0x28],
    /// Interrupt enable (0 regular, 1 interrupts enabled)
    int_en: u32,
    /// Interrupt mask (1 will mask the interrupt)
    int_mask: u32,
    /// Interrupt type level(0 level sensitive, 1 edge sensitive)
    int_ty_lev: u32,
    /// Interrupt polarity (0 active low, 1 active high)
    int_pol: u32,
    /// Interrupt status
    int_stat: u32,
    /// Raw interrupt status (no mask)
    raw_int_stat: u32,
    /// Debounce enable (0 No debounce, 1 enable debounce)
    debounce: u32,
    /// Clear interrupt (1 Clear interrupt)
    cl_int: u32,
    /// External port
    ext_port: u32,
    _space1: [u8; 0xC],
    /// Level sensitive synchronization enable (0 no synchronization to pclk_intr, 1 synchronized to pclk_intr)
    ls_sync: u32,
}

pub enum Direction {
    Input = 0,
    Output = 1,
}

pub fn set_gpio0_direction(pin: u8, direction: Direction) {
    let ptr = GPIO0 as *mut GPIO;
    unsafe {
        let ddr = addr_of_mut!((*ptr).ddr);
        let curr = ddr.read_volatile();
        match direction {
            Direction::Input => ddr.write_volatile(curr & !(1 << pin)),
            Direction::Output => ddr.write_volatile(curr | (1 << pin)),
        }
    }
}

pub fn set_gpio0(pin: u8, val: bool) {
    let ptr = GPIO0 as *mut GPIO;
    unsafe {
        let dr = addr_of_mut!((*ptr).dr);
        let curr = dr.read_volatile();
        match val {
            false => dr.write_volatile(curr & !(1 << pin)),
            true => dr.write_volatile(curr | (1 << pin)),
        }
    }
}

pub fn read_gpio0(pin: u8) -> bool {
    let ptr = GPIO0 as *mut GPIO;
    unsafe {
        let ext = addr_of_mut!((*ptr).ext_port);
        ext.read_volatile() & (1 << pin) != 0
    }
}
