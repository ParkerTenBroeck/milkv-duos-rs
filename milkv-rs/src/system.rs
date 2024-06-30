

#[repr(C)]
pub struct SystemControl{
    _reserved0: [u8; 4],

    /// Boot select \[2:0] <br>
    ///     0: SPI NAND <br>
    ///     1: reserved <br>
    ///     2: SPI_NOR <br>
    ///     3: EEMC <br>
    /// 
    /// IO status from USBID PAD \[8] <br>
    /// IO status from USB VBUS DET PAD \[9]
    /// 
    /// io_sta_trap \[31:24] <br>
    ///     io_sta_trap\[0] : io_boot_rom_din <br>
    ///     io_sta_trap\[1] : io_boot_dev0_din <br>
    ///     io_sta_trap\[2] : io_boot_dev1_din <br>
    ///     io_sta_trap\[3] : io_trap_sd0_pwr_din <br>
    ///     io_sta_trap\[4] : io_pkg_type0_din <br>
    ///     io_sta_trap\[5] : io_pkg_type1_din <br>
    ///     io_sta_trap\[6] : io_pkg_type2_din <br>
    ///     io_sta_trap\[7] : io_trap_zq_din <br>
    pub conf_info: u32,

    /// \[5:2] <br>
    ///     bit0 : wdt reset enabled <br>
    ///     bit 1 : cdbgrstreq enable <br>
    ///     bit 3 : soft reset x system enabled <br>
    pub sys_ctrl_reg: u32,

    _reserved1: [u8; 0x3c],

    pub usb_phy_ctrl_reg: u32,

    _reserved2: [u8; 0x108],
    
    pub sdma_dma_ch_remap0: u32,
    pub sdma_dma_ch_remap1: u32,

    _reserved3: [u8; 0x44],
    
    pub top_timer_clk_sel: u32,

    _reserved4: [u8; 0x4],

    pub top_wdt_ctl: u32,

    _reserved5: [u8; 0xc],

    pub ddr_axi_urgent_ow: u32,
    pub ddr_axi_urgent: u32,

    _reserved6: [u8; 0x18],
    
    pub ddr_axi_qos_0: u32,
    pub ddr_axi_qos_1: u32,

    _reserved7: [u8; 0x14],
    
    pub sd_pwrsw_ctrl: u32,
    pub sd_pwrsw_time: u32,
    
    _reserved8: [u8; 0x040],

    pub ddr_axi_qos_ow: u32,

    _reserved9: [u8; 0x54],

    pub sd_ctrl_opt: u32,
    pub sdma_dma_int_mux: u32,
}