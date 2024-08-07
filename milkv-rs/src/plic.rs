use core::ptr::addr_of_mut;

use crate::mmio::PLIC;

pub const MAX_INT_ID: usize = 128;

#[repr(C)]
pub struct Plic {
    prio: [u32; 1024],
    pend: [u32; 32],

    _res: [u32; 3972 / 4 - 1],

    h0_mie: [u32; 32],
    h0_sie: [u32; 32],

    h1_mie: [u32; 32],
    h1_sie: [u32; 32],

    h2_mie: [u32; 32],
    h2_sie: [u32; 32],

    h3_mie: [u32; 32],
    h3_sie: [u32; 32],

    _reserved1: [u32; (0x01FFFFC - 0x00023FC) / 4 - 1],

    perms: u32,

    h0_mth: u32,
    h0_mclaim: u32,

    _reserved2: [u32; 0xFFC / 4 - 1],

    h0_sth: u32,
    h0_sclaim: u32,

    _reserved3: [u32; 0xFFC / 4 - 1],

    h1_mth: u32,
    h1_mclaim: u32,

    _reserved4: [u32; 0xFFC / 4 - 1],

    h1_sth: u32,
    h1_sclaim: u32,

    _reserved5: [u32; 0xFFC / 4 - 1],

    h2_mth: u32,
    h2_mclaim: u32,

    _reserved6: [u32; 0xFFC / 4 - 1],

    h2_sth: u32,
    h2_sclaim: u32,

    _reserved7: [u32; 0xFFC / 4 - 1],

    h3_mth: u32,
    h3_mclaim: u32,

    _reserved8: [u32; 0xFFC / 4 - 1],

    h3_sth: u32,
    h3_sclaim: u32,

    _reserved9: [u32; 0xFFC / 4 - 1],
}

pub unsafe fn clear() {
    for i in 1..MAX_INT_ID{
        disable_m_interrupt(i as u32);
        disable_s_interrupt(i as u32);
        set_priority(i as u32, 0);
    }

    loop{
        let v = mclaim_int();
        if v != 0{
            mint_complete(v);
        }else{
            break;
        }
    }
    loop{
        let v = sclaim_int();
        if v != 0{
            sint_complete(v);
        }else{
            break;
        }
    }
}

pub unsafe fn mclaim_int() -> u32 {
    addr_of_mut!((*PLIC).h0_mclaim).read_unaligned()
}

pub unsafe fn enable_m_interrupt(int: u32) {
    let val = addr_of_mut!((*PLIC).h0_mie[(int as usize) / 32]);
    val.write_volatile(val.read_volatile() | (1 << (int % 32)));
}

pub unsafe fn disable_m_interrupt(int: u32) {
    let val = addr_of_mut!((*PLIC).h0_mie[(int as usize) / 32]);
    val.write_volatile(val.read_volatile() & !(1 << (int % 32)));
}

pub unsafe fn mint_threshhold(threshhold: u32) {
    addr_of_mut!((*PLIC).h0_mth).write_volatile(threshhold);
}

pub unsafe fn mint_complete(int: u32) {
    addr_of_mut!((*PLIC).h0_mclaim).write_volatile(int);
}

pub unsafe fn sclaim_int() -> u32 {
    addr_of_mut!((*PLIC).h0_sclaim).read_unaligned()
}

pub unsafe fn enable_s_interrupt(int: u32) {
    let val = addr_of_mut!((*PLIC).h0_sie[(int as usize) / 32]);
    val.write_volatile(val.read_volatile() | (1 << (int % 32)));
}

pub unsafe fn disable_s_interrupt(int: u32) {
    let val = addr_of_mut!((*PLIC).h0_sie[(int as usize) / 32]);
    val.write_volatile(val.read_volatile() & !(1 << (int % 32)));
}

pub unsafe fn sint_threshhold(threshhold: u32) {
    addr_of_mut!((*PLIC).h0_sth).write_volatile(threshhold);
}

pub unsafe fn sint_complete(int: u32) {
    addr_of_mut!((*PLIC).h0_sclaim).write_volatile(int);
}

pub unsafe fn set_ctrl(s_perm: bool) {
    addr_of_mut!((*PLIC).perms).write_volatile(if s_perm { 1 } else { 0 });
}

pub unsafe fn set_priority(int: u32, pri: u32) {
    addr_of_mut!((*PLIC).prio[int as usize]).write_volatile(pri);
}

pub unsafe fn disp(mut out: impl core::fmt::Write) {
    writeln!(
        out,
        "m thresh: {}",
        addr_of_mut!((*PLIC).h0_mth).read_volatile()
    )
    .unwrap();
    writeln!(
        out,
        "s thresh: {}",
        addr_of_mut!((*PLIC).h0_sth).read_volatile()
    )
    .unwrap();
    for i in 1..MAX_INT_ID {
        let menabled =
            (addr_of_mut!((*PLIC).h0_mie[(i as usize) / 32]).read_volatile() & 1 << (i % 32)) != 0;
        let senabled =
            (addr_of_mut!((*PLIC).h0_sie[(i as usize) / 32]).read_volatile() & 1 << (i % 32)) != 0;
        let pending =
            (addr_of_mut!((*PLIC).pend[(i as usize) / 32]).read_volatile() & 1 << (i % 32)) != 0;

        if menabled | senabled | pending {
            write!(out, "int: {i} ").unwrap();
        }
        if menabled {
            write!(out, "menabled ").unwrap()
        }
        if senabled {
            write!(out, "senabled ").unwrap()
        }
        if menabled | senabled {
            write!(
                out,
                "priority({}) ",
                addr_of_mut!((*PLIC).prio[i as usize]).read_volatile()
            )
            .unwrap()
        }
        if pending {
            write!(out, "pending").unwrap()
        }
        if menabled | senabled | pending {
            writeln!(out).unwrap()
        }
    }
}
