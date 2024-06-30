
#[repr(C)]
pub struct Ddrc{
    _reserved0: [u8; 0x64],
    /// DRAM refresh parameter
    pub ref_ctrl: u32,
    _reserved1: [u8; 0x74],
    /// DRAM MR value
    /// 
    /// \[15:0] mr1 write value
    /// 
    /// \[31:16] m0 write value
    pub mrd0: u32,
    /// DRAM MR value
    /// 
    /// \[15:0] mr3 write value
    /// 
    /// \[31:16] mr2 write value
    pub mrd1: u32,
}

#[repr(C)]
pub struct AxiCtrl{
    _reserved0: [u8; 0x4b4],
    /// AXI 1 read timeout control
    pub ctrl0_1: u32,
    /// AXI 1 write timeout control
    pub ctrl1_1: u32,

    _reserved1: [u8; 0xA8],

    /// AXI 2 read timeout control
    pub ctrl0_2: u32,
    /// AXI 2 write timeout control
    pub ctrl1_2: u32,

    _reserved2: [u8; 0xA8],

    /// AXI 3 read timeout control
    pub ctrl0_3: u32,
    /// AXI 3 write timeout control
    pub ctrl1_3: u32,
}

#[repr(C)]
pub struct AxiMon{
    pub ctrl: u32,
    pub input: u32,

    _reserved0: [u32; 2],

    pub filters: [u32; 9],

    _reserved1: [u32; 3],

    /// Cycle count
    pub rpt0: u32,
    /// Hit count
    pub rpt1: u32,
    /// Byte count
    pub rpt2: u32,
    /// latency count
    pub rpt3: u32,

    _reserved2: [u32; 12],
}

#[repr(C)]
pub struct AxiMons{
    pub mons: [AxiMon; 12]
}



pub unsafe fn init_ddr() {
    ddrc_init();
}

unsafe fn ddrc_init(){

}
