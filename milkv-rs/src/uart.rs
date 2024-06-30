#[allow(unused)]
#[repr(C)]
struct UartRegs {
    rbr: u32, /* 0x00 Data register */
    ier: u32, /* 0x04 Interrupt Enable Register */
    fcr: u32, /* 0x08 FIFO Control Register */
    lcr: u32, /* 0x0C Line control register */
    mcr: u32, /* 0x10 Line control register */
    lsr: u32, /* 0x14 Line Status Register */
    msr: u32, /* 0x18 Modem Status Register */
    spr: u32, /* 0x20 Scratch Register */
}

pub const UART_LCR_WLS_MSK: u32 = 0x03; /* character length select mask */
pub const UART_LCR_WLS_5: u32 = 0x00; /* 5 bit character length */
pub const UART_LCR_WLS_6: u32 = 0x01; /* 6 bit character length */
pub const UART_LCR_WLS_7: u32 = 0x02; /* 7 bit character length */
pub const UART_LCR_WLS_8: u32 = 0x03; /* 8 bit character length */
pub const UART_LCR_STB: u32 = 0x04; /* # stop Bits, off=1, on=1.5 or 2) */
pub const UART_LCR_PEN: u32 = 0x08; /* Parity eneble */
pub const UART_LCR_EPS: u32 = 0x10; /* Even Parity Select */
pub const UART_LCR_STKP: u32 = 0x20; /* Stick Parity */
pub const UART_LCR_SBRK: u32 = 0x40; /* Set Break */
pub const UART_LCR_BKSE: u32 = 0x80; /* Bank select enable */
pub const UART_LCR_DLAB: u32 = 0x80; /* Divisor latch access bit */

pub const UART_MCR_DTR: u32 = 0x01; /* DTR   */
pub const UART_MCR_RTS: u32 = 0x02; /* RTS   */

pub const UART_LSR_THRE: u32 = 0x20; /* Transmit-hold-register empty */
pub const UART_LSR_DR: u32 = 0x01; /* Receiver data ready */
pub const UART_LSR_TEMT: u32 = 0x40; /* Xmitter empty */

pub const UART_FCR_FIFO_EN: u32 = 0x01; /* Fifo enable */
pub const UART_FCR_RXSR: u32 = 0x02; /* Receiver soft reset */
pub const UART_FCR_TXSR: u32 = 0x04; /* Transmitter soft reset */

pub const UART_MCRVAL: u32 = UART_MCR_DTR | UART_MCR_RTS; /* RTS/DTR */
pub const UART_FCR_DEFVAL: u32 = UART_FCR_FIFO_EN | UART_FCR_RXSR | UART_FCR_TXSR;
pub const UART_LCR_8N1: u32 = 0x03;

const UART0: *mut UartRegs = (0x04000000 + 0x00140000) as *mut UartRegs;

#[inline(always)]
pub unsafe fn console_init() {
    // int baudrate = baud_rate;
    // int uart_clock = uart_clk;

    let divisor = 14; //uart_clock / (16 * baudrate);

    let lcr = core::ptr::addr_of_mut!((*UART0).lcr);
    let ier = core::ptr::addr_of_mut!((*UART0).ier);
    let dll = core::ptr::addr_of_mut!((*UART0).rbr);
    let dlm = core::ptr::addr_of_mut!((*UART0).ier);
    let mcr = core::ptr::addr_of_mut!((*UART0).mcr);
    let fcr = core::ptr::addr_of_mut!((*UART0).fcr);

    lcr.write_volatile(lcr.read_volatile() | UART_LCR_DLAB | UART_LCR_8N1);
    dll.write_volatile(divisor & 0xff);
    dlm.write_volatile((divisor >> 8) & 0xff);
    lcr.write_volatile(lcr.read_volatile() & (!UART_LCR_DLAB));
    ier.write_volatile(0);
    mcr.write_volatile(UART_MCRVAL);
    fcr.write_volatile(UART_FCR_DEFVAL);
    lcr.write_volatile(3);
}

#[inline(always)]
pub fn print_b(char: u8) {
    unsafe {
        let lsr = core::ptr::addr_of_mut!((*UART0).lsr);
        let rbr = core::ptr::addr_of_mut!((*UART0).rbr);

        while (lsr.read_volatile() & UART_LSR_THRE) == 0 {}
        rbr.write_volatile(char as u32);
    }
}

#[inline(never)]
pub fn print_c(char: u8) {
    if char == b'\n' {
        print_b(b'\r');
    }
    print_b(char);
}

#[inline(always)]
pub fn print(msg: &str) {
    for b in msg.bytes() {
        print_c(b);
    }
}

pub fn get_b() -> u8 {
    unsafe {
        let lsr = core::ptr::addr_of_mut!((*UART0).lsr);
        let rbr = core::ptr::addr_of_mut!((*UART0).rbr);

        while (lsr.read_volatile() & UART_LSR_DR) == 0 {}
        rbr.read_volatile() as u8
    }
}

pub fn has_b() -> bool {
    unsafe {
        let lsr = core::ptr::addr_of_mut!((*UART0).lsr);
        lsr.read_volatile() & UART_LSR_DR != 0
    }
}

pub fn flush() {
    unsafe {
        let lsr = core::ptr::addr_of_mut!((*UART0).lsr);
        while (lsr.read_volatile() & (UART_LSR_THRE | UART_LSR_TEMT))
            != (UART_LSR_THRE | UART_LSR_TEMT)
        {}
    }
}
