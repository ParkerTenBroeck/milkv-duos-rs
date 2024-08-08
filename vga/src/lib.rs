#![no_std]

use csr::disable_interrupts;
use gpio::GPIO;
use milkv_rs::*;

pub const GPIO_SEC: *mut GPIO = mmio::GPIO1;

pub const V_SYNC_PIN: u8 = 21;
pub const H_SYNC_PIN: u8 = 20;
pub const D_PINS: [u8; 8] = [11, 12, 13, 14, 15, 16, 18, 19];

pub const VERT_SCALE: u64 = 1;
pub const PIXEL_SCALE: u64 = 2;
pub const PIXEL_SCALE_DIV: u64 = 1;
// const PIXEL_SCALE: u64 = 2;
// const PIXEL_SCALE_DIV: u64 = 1;

pub const PIX_VIS: u64 = 640;
pub const H_FRONT_PORCH: u64 = 16;
pub const H_SYNC_PULSE: u64 = 96;
pub const H_BACK_PORCH: u64 = 48;
pub const H_TOTAL: u64 = PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH;

pub const H_FP_M: u64 = 0;
pub const H_SP_M: u64 = 0;
pub const H_BP_M: u64 = 0;

pub const LINES_VIS: u64 = 480;
pub const V_FRONT_PORCH: u64 = 10;
pub const V_SYNC_PULSE: u64 = 2;
pub const V_BACK_PORCH: u64 = 525  - LINES_VIS - V_FRONT_PORCH - V_SYNC_PULSE;
pub const V_TOTAL: u64 = LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH;

// pub const PX_TIM: u64 = 786_432 + 15000; //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
// pub const PX_TIM: u64 = 786_376196; //1049_925865 //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
// pub const PX_TIM: u64 = 1049_925865; //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
// pub const PX_TIME_DIV_FACT: u64 = 25_175000; //10000000000;

pub const PX_TIM: u64 = 1; //1049_925865 //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
// pub const PX_TIM: u64 = 1049_925865; //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
pub const PX_TIME_DIV_FACT: u64 = 1; //10000000000;


pub const H_SYNC_PH: u32 = 0 << H_SYNC_PIN;
pub const H_SYNC_PL: u32 = 1 << H_SYNC_PIN;
pub const V_SYNC_PH: u32 = 0 << V_SYNC_PIN;
pub const V_SYNC_PL: u32 = 1 << V_SYNC_PIN;

pub const WIDTH: u64 = PIX_VIS * PIXEL_SCALE_DIV / PIXEL_SCALE;
pub const HEIGHT: u64 = LINES_VIS / VERT_SCALE;

#[inline(always)]
pub fn time() -> u64{
    timer::get_mtimer()
}

#[allow(unused)]
pub unsafe fn run_vga(addr: usize) -> ! {

    let start_addr = addr as u64;

    unsafe fn per_line(line: u64) -> u64 {
        per_px(line * H_TOTAL)
    }
    unsafe fn per_px(pix: u64) -> u64 {
        (pix * PX_TIM) / PX_TIME_DIV_FACT
    }

    for pin in D_PINS{
        gpio::set_gpio_direction(GPIO_SEC, pin, gpio::Direction::Output);
        gpio::set_gpio_direction(GPIO_SEC, pin, gpio::Direction::Output);
    }
    gpio::set_gpio_direction(GPIO_SEC, V_SYNC_PIN, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, H_SYNC_PIN, gpio::Direction::Output);

    let gpio_dr = core::ptr::addr_of_mut!((*GPIO_SEC).dr);

    unsafe { disable_interrupts() }


    // let mut goal = time();
    // loop{
    //     goal += 1000000000;
    //     while goal > time(){}
    //     gpio_dr.write_volatile(1<<V_SYNC_PIN);
    //     goal += 1000000000;
    //     while goal > time(){}
    //     gpio_dr.write_volatile(0);
    // }

    // loop{
    //     gpio_dr.write_volatile(1<<V_SYNC_PIN);
    //     gpio_dr.write_volatile(0);
    // }

    loop {
        let start = time();
        
        for l in 0..LINES_VIS {
            let mut addr = start_addr + l/VERT_SCALE * WIDTH;
            
            let lstart = start + per_line(l);
            
            for p in 0..WIDTH {
                // const V: u64 = PX_TIM*PIXEL_SCALE/PIXEL_SCALE_DIV/PX_TIME_DIV_FACT;
                let pend = lstart + ((p+1) * PIXEL_SCALE / PIXEL_SCALE_DIV * PX_TIM) / PX_TIME_DIV_FACT;
                if (pend + PX_TIM*PIXEL_SCALE/PIXEL_SCALE_DIV/PX_TIME_DIV_FACT) < time(){
                    continue;
                }

                let pval = ((addr+p) as *mut u8).read_volatile() as u32;
                let pval = (pval & 0b00111111) << 11 | ((pval & 0b11000000) << 12) | const { H_SYNC_PL | V_SYNC_PL };
                gpio_dr.write_volatile(pval);

                while pend > time() {}
            }
            gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
            let fp = H_FP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH) };
            while fp > time() {}
            gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
            

            let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };
            // next scan line data prefetch
            if l/VERT_SCALE != LINES_VIS/VERT_SCALE-1{
                let mut addr = start_addr + (l+1)/VERT_SCALE * WIDTH;
                for addr in (addr..(addr + WIDTH + 63) & !63).step_by(4*8*2){
                    (addr as *mut u8).read_volatile();
                    if sp < time(){break}
                }
            }

            while sp > time() {}
            gpio_dr.write_volatile({ H_SYNC_PL | V_SYNC_PL });

            let bp =
            H_BP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) }; 
            
            while bp > time() {}
        }
        
        
        let fp = start + per_line({ LINES_VIS + V_FRONT_PORCH });
        {
            for l in LINES_VIS..(LINES_VIS + V_FRONT_PORCH) {
                let lstart = start + per_line(l);
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
                let fp = H_FP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH) };
                while fp > time() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > time() {}
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > time() {}
            }
        }

        let sp = start + per_line({ LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE });
        {
            for l in (LINES_VIS + V_FRONT_PORCH)..(LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE) {
                let lstart = start + per_line(l);
                gpio_dr.write_volatile( { H_SYNC_PL | V_SYNC_PH });
                let fp = H_FP_M + lstart +  { per_px(PIX_VIS + H_FRONT_PORCH) };
                while fp > time() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PH });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > time() {}
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PH });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > time() {}
            }
        }

        let bp =
            start + per_line({ LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH });
        

        { // scan line 0 (vis) data prefetch
            for addr in (start_addr..(start_addr + WIDTH + 63) & !63).step_by(4*8*2){
                (addr as *mut u8).read_volatile();
            }
        }
        {
            for l in (LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE)
                ..(LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH)
            {
                let lstart = start + per_line(l);
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
                let fp = H_FP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH) };
                while fp > time() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > time() {}
                gpio_dr.write_volatile({ H_SYNC_PL | V_SYNC_PL });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > time() {}
            }
        }
    }
}

pub fn flush_frame(addr: usize){
    for cl in (0..(WIDTH * HEIGHT) as usize).step_by(64){
        unsafe{
            core::arch::asm!("
                th.dcache.cpa {0}
            ",
            in(reg) addr + cl);
        }
    }
}

pub fn flush_frame_virt(addr: usize){
    for cl in (0..(WIDTH * HEIGHT) as usize).step_by(64){
        unsafe{
            core::arch::asm!("
                th.dcache.cva {0}
            ",
            in(reg) addr + cl);
        }
    }
}