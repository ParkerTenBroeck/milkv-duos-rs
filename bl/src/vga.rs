use csr::disable_interrupts;
use gpio::GPIO;
use milkv_rs::*;

// const PIXEL_SCALE: u64 = 2;
// const V_SYNC_PIN: u8 = 14;
// const H_SYNC_PIN: u8 = 15;

// pub const PIX_VIS: u64 = 640;
// pub const H_FRONT_PORCH: u64 = 14;
// pub const H_SYNC_PULSE: u64 = 90;
// pub const H_BACK_PORCH: u64 = 50;
// pub const H_TOTAL: u64 = unsafe{PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH};

// pub const H_FP_M: u64 = 1;
// pub const H_SP_M: u64 = 0;
// pub const H_BP_M: u64 = 0;

// pub const LINES_VIS: u64 = 480;
// pub const V_FRONT_PORCH: u64 = 10;
// pub const V_SYNC_PULSE: u64 = 2;
// pub const V_BACK_PORCH: u64 = 33;
// pub const V_TOTAL: u64 = unsafe{ LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH };
// pub const V_MICRO_T: u64 = 0;

// pub const PX_TIM: u64 = 1; //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
// pub const PX_TIME_DIV_FACT: u64 = 1; //10000000000;

const GPIO_SEC: *mut GPIO = mmio::GPIO1;

const V_SYNC_PIN: u8 = 21;
const H_SYNC_PIN: u8 = 20;
const C_START_PIN: u8 = 13;

const PIXEL_SCALE: u64 = 2;

pub const PIX_VIS: u64 = 640;
pub const H_FRONT_PORCH: u64 = 14;
pub const H_SYNC_PULSE: u64 = 90;
pub const H_BACK_PORCH: u64 = 50;
pub const H_TOTAL: u64 = PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH;

pub const H_FP_M: u64 = 0;
pub const H_SP_M: u64 = 0;
pub const H_BP_M: u64 = 0;

pub const LINES_VIS: u64 = 480;
pub const V_FRONT_PORCH: u64 = 10;
pub const V_SYNC_PULSE: u64 = 2;
pub const V_BACK_PORCH: u64 = 525  - LINES_VIS - V_FRONT_PORCH - V_SYNC_PULSE;
pub const V_TOTAL: u64 = LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH;

pub const PX_TIM: u64 = 1; //0_0397219464 * timer::SYS_COUNTER_FREQ_IN_US;
pub const PX_TIME_DIV_FACT: u64 = 	1 ; //10000000000;


pub const H_SYNC_PH: u32 = 0 << H_SYNC_PIN;
pub const H_SYNC_PL: u32 = 1 << H_SYNC_PIN;
pub const V_SYNC_PH: u32 = 0 << V_SYNC_PIN;
pub const V_SYNC_PL: u32 = 1 << V_SYNC_PIN;

pub const WIDTH: u64 = PIX_VIS / PIXEL_SCALE;
pub const HEIGHT: u64 = LINES_VIS / PIXEL_SCALE;

#[allow(unused)]
pub unsafe fn run_vga() -> ! {


    unsafe fn per_line(line: u64) -> u64 {
        per_px(line * H_TOTAL)
    }
    unsafe fn per_px(pix: u64) -> u64 {
        (pix * PX_TIM) / PX_TIME_DIV_FACT
    }

    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 0, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 1, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 2, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 3, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 4, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 5, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 6, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, C_START_PIN + 7, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, V_SYNC_PIN, gpio::Direction::Output);
    gpio::set_gpio_direction(GPIO_SEC, H_SYNC_PIN, gpio::Direction::Output);

    let gpio_dr = core::ptr::addr_of_mut!((*GPIO_SEC).dr);

    unsafe { disable_interrupts() }
    // core::arch::asm!(
    //     "
    //     # invalid I-cache
    //     li x3, 0x33
    //     csrc {mcor}, x3
    //     li x3, 0x11
    //     csrs {mcor}, x3
    //     # enable I-cache
    //     li x3, 0x1
    //     csrs {mhcr}, x3
        
    //     # invalid D-cache
    //     li x3, 0x33
    //     csrc {mcor}, x3
    //     li x3, 0x12
    //     csrs {mcor}, x3
    //     # enable D-cache
    //     li x3, 0x2
    //     csrs {mhcr}, x3
    //     ",
    //     mcor = const csr::mcor,
    //     mhcr = const csr::mhcr,
    // );

    loop {
        let start = timer::get_mtimer();
        
        for l in 0..LINES_VIS {
            let mut addr = 0x80000000 + l/PIXEL_SCALE * PIX_VIS/PIXEL_SCALE;
            
            let lstart = start + per_line(l);
            
            for p in 0..(PIX_VIS / PIXEL_SCALE) {
                let pend = lstart + per_px((p+1) * PIXEL_SCALE) 
                    //hack
                    // + if p < (PIX_VIS / PIXEL_SCALE)*5/10 {1} else {0}
                    // +1
                    ;
                if (pend +1) < timer::get_mtimer(){
                    continue;
                }

                let pval = ((addr+p) as *mut u8).read_volatile() as u32;
                let pval = pval << C_START_PIN | const { H_SYNC_PL | V_SYNC_PL };
                gpio_dr.write_volatile(pval);

                while pend > timer::get_mtimer() {}
            }
            gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
            let fp = H_FP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH) };
            while fp > timer::get_mtimer() {}
            gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
            let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

            while sp > timer::get_mtimer() {}
            gpio_dr.write_volatile({ H_SYNC_PL | V_SYNC_PL });
            let bp =
            H_BP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
            // next scan line data prefetch
            if l/PIXEL_SCALE != LINES_VIS/PIXEL_SCALE-1{
                let mut addr = 0x80000000 + (l+1)/PIXEL_SCALE * PIX_VIS/PIXEL_SCALE;
                for addr in (addr..(addr + PIX_VIS/PIXEL_SCALE)).step_by(4*8*2){
                    (addr as *mut u8).read_volatile();
                }
            } 
            
            while bp > timer::get_mtimer() {}
        }
        
        
        let fp = start + per_line({ LINES_VIS + V_FRONT_PORCH });
        {
            for l in LINES_VIS..(LINES_VIS + V_FRONT_PORCH) {
                let lstart = start + per_line(l);
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
                let fp = H_FP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH) };
                while fp > timer::get_mtimer() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > timer::get_mtimer() {}
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PL });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > timer::get_mtimer() {}
            }
        }

        let sp = start + per_line({ LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE });
        {
            for l in (LINES_VIS + V_FRONT_PORCH)..(LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE) {
                let lstart = start + per_line(l);
                gpio_dr.write_volatile( { H_SYNC_PL | V_SYNC_PH });
                let fp = H_FP_M + lstart +  { per_px(PIX_VIS + H_FRONT_PORCH) };
                while fp > timer::get_mtimer() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PH });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > timer::get_mtimer() {}
                gpio_dr.write_volatile(const { H_SYNC_PL | V_SYNC_PH });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > timer::get_mtimer() {}
            }
        }

        let bp =
            start + per_line({ LINES_VIS + V_FRONT_PORCH + V_SYNC_PULSE + V_BACK_PORCH });
        

        { // scan line 0 (vis) data prefetch
            let mut addr = 0x80000000;
            for addr in (addr..(addr + PIX_VIS/PIXEL_SCALE)).step_by(4*8*2){
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
                while fp > timer::get_mtimer() {}
                gpio_dr.write_volatile(const { H_SYNC_PH | V_SYNC_PL });
                let sp = H_SP_M + lstart + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE) };

                while sp > timer::get_mtimer() {}
                gpio_dr.write_volatile({ H_SYNC_PL | V_SYNC_PL });
                let bp = H_BP_M + lstart
                    + { per_px(PIX_VIS + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH) };
                while bp > timer::get_mtimer() {}
            }
        }
    }
}

pub fn flush_frame(){
    for cl in (0..(WIDTH * HEIGHT) as usize).step_by(64){
        unsafe{
            core::arch::asm!("
                th.dcache.cpa {0}
            ",
            in(reg) 0x80000000usize + cl);
        }
    }
}

// unsafe fn vga() {
//     gpio::set_gpio_direction(mmio::GPIO1, 1, gpio::Direction::Output);
//     gpio::set_gpio_direction(mmio::GPIO1, 2, gpio::Direction::Output);
//     gpio::set_gpio_direction(mmio::GPIO1, 3, gpio::Direction::Output);
//     gpio::set_gpio_direction(mmio::GPIO1, 15, gpio::Direction::Input);
//     gpio::set_gpio_direction(mmio::GPIO1, 14, gpio::Direction::Input);

//     let ptr = core::ptr::addr_of_mut!((*mmio::GPIO1).dr);

//     let data = 0x80000000 as *mut u8;

//     unsafe { disable_interrupts() }
//     loop {

//         while !gpio::read_gpio(mmio::GPIO1, 14) {}
//         while gpio::read_gpio(mmio::GPIO1, 14) {}
//         // currently on back porch
//         timer::udelay(600);

//         while gpio::read_gpio(mmio::GPIO1, 15) {}
//         while !gpio::read_gpio(mmio::GPIO1, 15) {}
//         let start = timer::get_mtimer() + 10486 * timer::SYS_COUNTER_FREQ_IN_US / 10000;

//         let mut addr = 0x80000000usize;
//         for i in 0..480 {
//             let tgoal = i * 3338 * timer::SYS_COUNTER_FREQ_IN_US / 100 + start;
//             while timer::get_mtimer() < tgoal {}
//             let tgoal = tgoal.wrapping_add(26 * timer::SYS_COUNTER_FREQ_IN_US);

//             // let mut pval = 0;
//             // let mut ptime = pstart;
//             let pstart = timer::get_mtimer();
//             while timer::get_mtimer() < tgoal {
//                 // pval += 1;
//                 let pval = ((addr) as *mut u8).read_volatile() as u32;
//                 addr += 1;
//                 let pval = (pval & 0b111) << 1;
//                 ptr.write_volatile(pval);
//                 // ptime += 2;
//                 // while timer::get_mtimer() < ptime{}
//             }
//             addr = i as usize * 480 + 0x80000000usize;
//             ptr.write_volatile(0);
//         }
//         // unsafe { enable_interrupts() }
//     }
// }
