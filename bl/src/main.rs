#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod entry;
pub mod interrupt_vector;
pub mod panic;
pub mod prelude;
pub mod cmd;

pub use prelude::*;


#[no_mangle]
pub extern "C" fn bl_rust_main() {
    timer::mdelay(250);
    unsafe {

        uart::console_init();

    }
    timer::mdelay(250);
    uart::print("\n\n\nBooted into firmware\nInitialized uart to 115200\n");

    unsafe {
        if let Err(_) = security::efuse::lock_efuse() {
            reset();
        } else {
            uart::print("Locked efuse\n");
        }
    }

    unsafe {
        // set pinmux to enable output of LED pin
        mmio_write_32!(0x03001074, 0x3);
        gpio::set_gpio0_direction(29, gpio::Direction::Output);
    }
    uart::print("Connfigured pinmux(LED pin 29)\n");

    uart::print("Enabling interrupts\n");
    unsafe {
        csr::enable_timer_interrupt();
        csr::enable_interrupts();
        // trigger an interrupt NOW
        timer::set_timercmp(timer::get_mtimer());


        // plic is seen as a single external interrupt source
        csr::enable_external_interrupt();
        // all enabled interrupts allowed
        plic::mint_threshhold(0);

        //--------------- timer 0 initialization ----------------------
        interrupt_vector::add_plic_handler(interrupt::TIMER0, ||{
            gpio::set_gpio0(29, !gpio::read_gpio0(29));
            timer::mm::clear_int(mmio::TIMER0);
        });
        
        // timer 0 interrupt number
        plic::set_priority(interrupt::TIMER0, 1);
        plic::enable_m_interrupt(interrupt::TIMER0);

        // initialize timer0
        timer::mm::set_mode(mmio::TIMER0, timer::mm::TimerMode::Count);
        // quarter second
        timer::mm::set_load_value(mmio::TIMER0, timer::SYS_COUNTER_FREQ_IN_SECOND as u32 / 4);
        timer::mm::set_enabled(mmio::TIMER0, true);
        //-------------------------------------


        plic::set_priority(interrupt::UART0, 1);
        plic::enable_m_interrupt(interrupt::UART0);
    }
    uart::print("Interrupts enabled\n");

    unsafe {
        ddr::init_ddr();
    }
    uart::print("DDR initialized\n");

    uart::print("Starting console\n");

    cmd::run();
}


