use core::ptr::addr_of_mut;

#[repr(C)]
pub struct GPIO {
    /// data register
    pub dr: u32,
    /// Data direction register (0 input, 1 output)
    pub ddr: u32,
    _space0: [u8; 0x28],
    /// Interrupt enable (0 regular, 1 interrupts enabled)
    pub int_en: u32,
    /// Interrupt mask (1 will mask the interrupt)
    pub int_mask: u32,
    /// Interrupt type level(0 level sensitive, 1 edge sensitive)
    pub int_ty_lev: u32,
    /// Interrupt polarity (0 active low, 1 active high)
    pub int_pol: u32,
    /// Interrupt status
    pub int_stat: u32,
    /// Raw interrupt status (no mask)
    pub raw_int_stat: u32,
    /// Debounce enable (0 No debounce, 1 enable debounce)
    pub debounce: u32,
    /// Clear interrupt (1 Clear interrupt)
    pub cl_int: u32,
    /// External port
    pub ext_port: u32,
    _space1: [u8; 0xC],
    /// Level sensitive synchronization enable (0 no synchronization to pclk_intr, 1 synchronized to pclk_intr)
    pub ls_sync: u32,
}

pub enum Direction {
    Input = 0,
    Output = 1,
}

pub unsafe fn set_gpio_direction(gpio: *mut GPIO, pin: u8, direction: Direction) {
    let ddr = addr_of_mut!((*gpio).ddr);
    let curr = ddr.read_volatile();
    match direction {
        Direction::Input => ddr.write_volatile(curr & !(1 << pin)),
        Direction::Output => ddr.write_volatile(curr | (1 << pin)),
    }
}

pub unsafe fn set_gpio(gpio: *mut GPIO, pin: u8, val: bool) {
    let dr = addr_of_mut!((*gpio).dr);
    let curr = dr.read_volatile();
    match val {
        false => dr.write_volatile(curr & !(1 << pin)),
        true => dr.write_volatile(curr | (1 << pin)),
    }
}

pub unsafe fn read_gpio(gpio: *mut GPIO, pin: u8) -> bool {
    let ext = addr_of_mut!((*gpio).ext_port);
    ext.read_volatile() & (1 << pin) != 0
}
