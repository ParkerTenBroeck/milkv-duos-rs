use crate::*;

pub struct Buffer {
    start: usize,
    end: usize,
    size: usize,
    data: [u8; 4096],
}

impl Buffer {
    pub fn push(&mut self, data: u8) {
        self.data[self.end] = data;

        self.end += 1;
        self.end %= self.data.len();

        self.size += 1;

        if self.size > self.data.len() {
            self.size = self.data.len();
            self.start = self.end;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            let ret = Some(self.data[self.start]);
            self.start += 1;
            self.start %= self.data.len();
            ret
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

pub static mut RX: Buffer = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
pub static mut TX: Buffer = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

pub unsafe fn init() {
    interrupt_vector::add_plic_handler(interrupt::UART0, |_, _, _, _| {
        let cause = core::ptr::addr_of_mut!((*mmio::UART0).fcr_iir).read_volatile();
        let cause = cause & 0b1111;
        if cause == 0b1100 || cause == 0b0100 {
            while uart::has_b() {
                unsafe { RX.push(uart::get_b()) }
            }
        } else if cause == 0b0010 {
            while uart::uart_has_rx_space() {
                if let Some(val) = TX.pop() {
                    uart::print_c(val);
                } else {
                    core::ptr::addr_of_mut!((*mmio::UART0).ier_dlh).write_volatile(0b000000001);
                    break;
                }
            }
        }

        // println!("{cause:04b}");
    });

    plic::set_priority(interrupt::UART0, 1);
    plic::enable_m_interrupt(interrupt::UART0);

    core::ptr::addr_of_mut!((*mmio::UART0).fcr_iir).write_volatile(0b11_01_0_0_0_1);
    core::ptr::addr_of_mut!((*mmio::UART0).ier_dlh).write_volatile(0b000000011);
}
