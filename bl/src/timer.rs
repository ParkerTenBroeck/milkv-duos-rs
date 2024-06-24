pub const TOP_BASE: u32 = 0x03000000;
pub const REG_GP_REG2: u32 = TOP_BASE + 0x88;

pub fn udelay(usec: u64){
    let start = get_timer_value();
    let delta_d = usec * SYS_COUNTER_FREQ_IN_US;
    while get_timer_value().wrapping_sub(start) < delta_d{}
}
 
#[inline(never)]
pub fn mdelay(msec: u64){
	udelay(msec * 1000);
}

pub const SYS_COUNTER_FREQ_IN_SECOND: u64 = 25000000;
pub const SYS_COUNTER_FREQ_IN_US: u64 = SYS_COUNTER_FREQ_IN_SECOND / 1000000;

pub fn get_timer(base: u64) -> u64 {
	if base == 0{
		get_timer_value()   
    }else{
		(base - get_timer_value()) / SYS_COUNTER_FREQ_IN_US / 1000 // ms
    }
}

pub fn get_timer_value() -> u64 {

     let val: u64;
     unsafe{
        core::arch::asm!(
            "csrr {val},0xc01",
            val = out(reg) val,
         );
     }

     val
}