#![allow(warnings)]

use core::mem::MaybeUninit;

use crate::{mmap::PARAM1_BASE, mmio_read_32, mmio_write_32, platform::{fip_param1, fip_param2, BLOCK_SIZE}, uart};


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

const REMAPPING_BASE: u32 = 0;
const AXIMON_M1_WRITE: u32 = (REMAPPING_BASE + 0x0);
const AXIMON_M1_READ: u32 = (REMAPPING_BASE + 0x80);
const AXIMON_M2_WRITE: u32 = (REMAPPING_BASE + 0x100);
const AXIMON_M2_READ: u32 = (REMAPPING_BASE + 0x180);
const AXIMON_M3_WRITE: u32 = (REMAPPING_BASE + 0x200);
const AXIMON_M3_READ: u32 = (REMAPPING_BASE + 0x280);
const AXIMON_M4_WRITE: u32 = (REMAPPING_BASE + 0x300);
const AXIMON_M4_READ: u32 = (REMAPPING_BASE + 0x380);
const AXIMON_M5_WRITE: u32 = (REMAPPING_BASE + 0x400);
const AXIMON_M5_READ: u32 = (REMAPPING_BASE + 0x480);
const AXIMON_M6_WRITE: u32 = (REMAPPING_BASE + 0x500);
const AXIMON_M6_READ: u32 = (REMAPPING_BASE + 0x580);


const AXIMON_OFFSET_CYCLE: u32 = 0x24;
const AXIMON_OFFSET_BYTECNTS: u32 = 0x2C;
const AXIMON_OFFSET_LATENCYCNTS: u32 = 0x34;
const AXIMON_OFFSET_HITCNTS: u32 = 0x28;
const AXIMON_OFFSET_LAT_BIN_SIZE_SEL: u32 = 0x50;

const AXIMON_START_REGVALUE: u32 = 0x30001;
const AXIMON_STOP_REGVALUE: u32 = 0x30002;

const PHY_BASE_ADDR: u32 = 2048;
const PI_BASE_ADDR: u32 = 0;
const CADENCE_PHYD: u32 = 0x08000000;
const CADENCE_PHYD_APB: u32 = 0x08006000;
const cfg_base: u32 = 0x08004000;

const DDR_SYS_BASE: u32 = 0x08000000;
// const PI_BASE: u32 = (DDR_SYS_BASE + 0x0000);
const PHY_BASE: u32 = (DDR_SYS_BASE + 0x2000);
const DDRC_BASE: u32 = (DDR_SYS_BASE + 0x4000);
const PHYD_BASE: u32 = (DDR_SYS_BASE + 0x6000);
const CV_DDR_PHYD_APB: u32 = (DDR_SYS_BASE + 0x6000);
const AXI_MON_BASE: u32 = (DDR_SYS_BASE + 0x8000);
// const TOP_BASE: u32 = (DDR_SYS_BASE + 0xa000);
const DDR_TOP_BASE: u32 = (DDR_SYS_BASE + 0xa000);
const PHYD_BASE_ADDR: u32 = (DDR_SYS_BASE);
const DDR_BIST_BASE: u32 = 0x08010000;
const DDR_BIST_SRAM_DQ_BASE: u32 = 0x08011000;
const DDR_BIST_SRAM_DM_BASE: u32 = 0x08011800;



const DDR_PHY_REG_0_DATA: u32 = 0b00000011000000100000000100000000;
	// param_phyd_swap_ca0:[4:0]=0b00000
	// param_phyd_swap_ca1:[12:8]=0b00001
	// param_phyd_swap_ca2:[20:16]=0b00010
	// param_phyd_swap_ca3:[28:24]=0b00011
const DDR_PHY_REG_1_DATA: u32 = 0b00000111000001100000010100000100;
	// param_phyd_swap_ca4:[4:0]=0b00100
	// param_phyd_swap_ca5:[12:8]=0b00101
	// param_phyd_swap_ca6:[20:16]=0b00110
	// param_phyd_swap_ca7:[28:24]=0b00111
const DDR_PHY_REG_2_DATA: u32 = 0b00001011000010100000100100001000;
	// param_phyd_swap_ca8:[4:0]=0b01000
	// param_phyd_swap_ca9:[12:8]=0b01001
	// param_phyd_swap_ca10:[20:16]=0b01010
	// param_phyd_swap_ca11:[28:24]=0b01011
const DDR_PHY_REG_3_DATA: u32 = 0b00001111000011100000110100001100;
	// param_phyd_swap_ca12:[4:0]=0b01100
	// param_phyd_swap_ca13:[12:8]=0b01101
	// param_phyd_swap_ca14:[20:16]=0b01110
	// param_phyd_swap_ca15:[28:24]=0b01111
const DDR_PHY_REG_4_DATA: u32 = 0b00010011000100100001000100010000;
	// param_phyd_swap_ca16:[4:0]=0b10000
	// param_phyd_swap_ca17:[12:8]=0b10001
	// param_phyd_swap_ca18:[20:16]=0b10010
	// param_phyd_swap_ca19:[28:24]=0b10011
const DDR_PHY_REG_5_DATA: u32 = 0b00000000000101100001010100010100;
	// param_phyd_swap_ca20:[4:0]=0b10100
	// param_phyd_swap_ca21:[12:8]=0b10101
	// param_phyd_swap_ca22:[20:16]=0b10110
const DDR_PHY_REG_6_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_swap_cke0:[0:0]=0b0
	// param_phyd_swap_cs0:[4:4]=0b0
const DDR_PHY_REG_7_DATA: u32 = 0b00000000000000000000000100000000;
	// param_phyd_data_byte_swap_slice0:[1:0]=0b00
	// param_phyd_data_byte_swap_slice1:[9:8]=0b01
const DDR_PHY_REG_8_DATA: u32 = 0b01110110010101000011001000010000;
	// param_phyd_swap_byte0_dq0_mux:[3:0]=0b0000
	// param_phyd_swap_byte0_dq1_mux:[7:4]=0b0001
	// param_phyd_swap_byte0_dq2_mux:[11:8]=0b0010
	// param_phyd_swap_byte0_dq3_mux:[15:12]=0b0011
	// param_phyd_swap_byte0_dq4_mux:[19:16]=0b0100
	// param_phyd_swap_byte0_dq5_mux:[23:20]=0b0101
	// param_phyd_swap_byte0_dq6_mux:[27:24]=0b0110
	// param_phyd_swap_byte0_dq7_mux:[31:28]=0b0111
const DDR_PHY_REG_9_DATA: u32 = 0b00000000000000000000000000001000;
	// param_phyd_swap_byte0_dm_mux:[3:0]=0b1000
const DDR_PHY_REG_10_DATA: u32 = 0b01110110010101000011001000010000;
	// param_phyd_swap_byte1_dq0_mux:[3:0]=0b0000
	// param_phyd_swap_byte1_dq1_mux:[7:4]=0b0001
	// param_phyd_swap_byte1_dq2_mux:[11:8]=0b0010
	// param_phyd_swap_byte1_dq3_mux:[15:12]=0b0011
	// param_phyd_swap_byte1_dq4_mux:[19:16]=0b0100
	// param_phyd_swap_byte1_dq5_mux:[23:20]=0b0101
	// param_phyd_swap_byte1_dq6_mux:[27:24]=0b0110
	// param_phyd_swap_byte1_dq7_mux:[31:28]=0b0111
const DDR_PHY_REG_11_DATA: u32 = 0b00000000000000000000000000001000;
	// param_phyd_swap_byte1_dm_mux:[3:0]=0b1000
const DDR_PHY_REG_16_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_dll_rx_sw_mode:[0:0]=0b0
	// param_phyd_dll_rx_start_cal:[1:1]=0b0
	// param_phyd_dll_rx_cntr_mode:[2:2]=0b0
	// param_phyd_dll_rx_hwrst_time:[3:3]=0b0
	// param_phyd_dll_tx_sw_mode:[16:16]=0b0
	// param_phyd_dll_tx_start_cal:[17:17]=0b0
	// param_phyd_dll_tx_cntr_mode:[18:18]=0b0
	// param_phyd_dll_tx_hwrst_time:[19:19]=0b0
const DDR_PHY_REG_17_DATA: u32 = 0b00000000011111110000000000001101;
	// param_phyd_dll_slave_delay_en:[0:0]=0b1
	// param_phyd_dll_rw_en:[1:1]=0b0
	// param_phyd_dll_avg_mode:[2:2]=0b1
	// param_phyd_dll_upd_wait:[6:3]=0b0001
	// param_phyd_dll_sw_clr:[7:7]=0b0
	// param_phyd_dll_sw_code_mode:[8:8]=0b0
	// param_phyd_dll_sw_code:[23:16]=0b01111111
const DDR_PHY_REG_18_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_tx_clk_tx_dline_code_clkn0:[6:0]=0b0000000
	// param_phya_reg_tx_clk_tx_dline_code_clkp0:[14:8]=0b0000000
const DDR_PHY_REG_19_DATA: u32 = 0b00000000000000000000000000001000;
	// param_phya_reg_sel_ddr4_mode:[0:0]=0b0
	// param_phya_reg_sel_lpddr3_mode:[1:1]=0b0
	// param_phya_reg_sel_lpddr4_mode:[2:2]=0b0
	// param_phya_reg_sel_ddr3_mode:[3:3]=0b1
	// param_phya_reg_sel_ddr2_mode:[4:4]=0b0
const DDR_PHY_REG_20_DATA: u32 = 0b00000000000000000000000000000110;
	// param_phyd_dram_class:[3:0]=0b0110
const DDR_PHY_REG_21_DATA: u32 = 0b00001100000000000000101100000000;
	// param_phyd_wrlvl_start_delay_code:[6:0]=0b0000000
	// param_phyd_wrlvl_start_shift_code:[13:8]=0b001011
	// param_phyd_wrlvl_end_delay_code:[22:16]=0b0000000
	// param_phyd_wrlvl_end_shift_code:[29:24]=0b001100
const DDR_PHY_REG_22_DATA: u32 = 0b00001001000101100000000001001111;
	// param_phyd_wrlvl_capture_cnt:[3:0]=0b1111
	// param_phyd_wrlvl_dly_step:[7:4]=0b0100
	// param_phyd_wrlvl_disable:[11:8]=0b0000
	// param_phyd_wrlvl_resp_wait_cnt:[21:16]=0b010110
	// param_phyd_oenz_lead_cnt:[26:23]=0b0010
	// param_phyd_wrlvl_mode:[27:27]=0b1
const DDR_PHY_REG_23_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_wrlvl_sw:[0:0]=0b0
	// param_phyd_wrlvl_sw_upd_req:[1:1]=0b0
	// param_phyd_wrlvl_sw_resp:[2:2]=0b0
	// param_phyd_wrlvl_data_mask:[23:16]=0b00000000
const DDR_PHY_REG_24_DATA: u32 = 0b00000100000000000000001101110000;
	// param_phyd_pigtlvl_back_step:[7:0]=0b01110000
	// param_phyd_pigtlvl_capture_cnt:[11:8]=0b0011
	// param_phyd_pigtlvl_disable:[19:16]=0b0000
	// param_phyd_pigtlvl_dly_step:[27:24]=0b0100
const DDR_PHY_REG_25_DATA: u32 = 0b00010000000000000000110000000000;
	// param_phyd_pigtlvl_start_delay_code:[6:0]=0b0000000
	// param_phyd_pigtlvl_start_shift_code:[13:8]=0b001100
	// param_phyd_pigtlvl_end_delay_code:[22:16]=0b0000000
	// param_phyd_pigtlvl_end_shift_code:[29:24]=0b010000
const DDR_PHY_REG_26_DATA: u32 = 0b00000000100000000000000000000000;
	// param_phyd_pigtlvl_resp_wait_cnt:[5:0]=0b000000
	// param_phyd_pigtlvl_sw:[8:8]=0b0
	// param_phyd_pigtlvl_sw_resp:[13:12]=0b00
	// param_phyd_pigtlvl_sw_upd_req:[16:16]=0b0
	// param_phyd_rx_en_lead_cnt:[23:20]=0b1000
const DDR_PHY_REG_28_DATA: u32 = 0b00000000000000000000000100001000;
	// param_phyd_rgtrack_threshold:[4:0]=0b01000
	// param_phyd_rgtrack_dly_step:[11:8]=0b0001
	// param_phyd_rgtrack_disable:[19:16]=0b0000
const DDR_PHY_REG_29_DATA: u32 = 0b00000000000001110010000000000000;
	// param_phyd_zqcal_wait_count:[3:0]=0b0000
	// param_phyd_zqcal_cycle_count:[15:8]=0b00100000
	// param_phyd_zqcal_hw_mode:[18:16]=0b111
const DDR_PHY_REG_32_DATA: u32 = 0b00011111000000001110000000000000;
	// param_phyd_pirdlvl_dlie_code_start:[7:0]=0b00000000
	// param_phyd_pirdlvl_dlie_code_end:[15:8]=0b11100000
	// param_phyd_pirdlvl_deskew_start:[22:16]=0b0000000
	// param_phyd_pirdlvl_deskew_end:[30:24]=0b0011111
const DDR_PHY_REG_33_DATA: u32 = 0b00000001000010110000110000001010;
	// param_phyd_pirdlvl_trig_lvl_start:[4:0]=0b01010
	// param_phyd_pirdlvl_trig_lvl_end:[12:8]=0b01100
	// param_phyd_pirdlvl_rdvld_start:[20:16]=0b01011
	// param_phyd_pirdlvl_rdvld_end:[28:24]=0b00001
const DDR_PHY_REG_34_DATA: u32 = 0b00001010000000010000000100010100;
	// param_phyd_pirdlvl_dly_step:[3:0]=0b0100
	// param_phyd_pirdlvl_ds_dly_step:[7:4]=0b0001
	// param_phyd_pirdlvl_vref_step:[11:8]=0b0001
	// param_phyd_pirdlvl_disable:[15:12]=0b0000
	// param_phyd_pirdlvl_resp_wait_cnt:[21:16]=0b000001
	// param_phyd_pirdlvl_vref_wait_cnt:[31:24]=0b00001010
const DDR_PHY_REG_35_DATA: u32 = 0b10101010101010100000000010001111;
	// param_phyd_pirdlvl_rx_prebit_deskew_en:[0:0]=0b1
	// param_phyd_pirdlvl_rx_init_deskew_en:[1:1]=0b1
	// param_phyd_pirdlvl_vref_training_en:[2:2]=0b1
	// param_phyd_pirdlvl_rdvld_training_en:[3:3]=0b1
	// param_phyd_pirdlvl_capture_cnt:[7:4]=0b1000
	// param_phyd_pirdlvl_MR1520_BYTE:[15:8]=0b00000000
	// param_phyd_pirdlvl_MR3240:[31:16]=0b1010101010101010
const DDR_PHY_REG_36_DATA: u32 = 0b00000000000000000011100000000000;
	// param_phyd_pirdlvl_data_mask:[8:0]=0b000000000
	// param_phyd_pirdlvl_sw:[9:9]=0b0
	// param_phyd_pirdlvl_sw_upd_req:[10:10]=0b0
	// param_phyd_pirdlvl_sw_resp:[12:11]=0b11
	// param_phyd_pirdlvl_trig_lvl_dqs_follow_dq:[13:13]=0b1
const DDR_PHY_REG_37_DATA: u32 = 0b00000000000000000000100000000001;
	// param_phyd_pirdlvl_rdvld_offset:[3:0]=0b0001
	// param_phyd_pirdlvl_found_cnt_limite:[15:8]=0b00001000
const DDR_PHY_REG_40_DATA: u32 = 0b00000111010000000000010101000000;
	// param_phyd_piwdqlvl_start_delay_code:[6:0]=0b1000000
	// param_phyd_piwdqlvl_start_shift_code:[13:8]=0b000101
	// param_phyd_piwdqlvl_end_delay_code:[22:16]=0b1000000
	// param_phyd_piwdqlvl_end_shift_code:[29:24]=0b000111
const DDR_PHY_REG_41_DATA: u32 = 0b00000001010000100000010100000100;
	// param_phyd_piwdqlvl_tx_vref_start:[4:0]=0b00100
	// param_phyd_piwdqlvl_tx_vref_end:[12:8]=0b00101
	// param_phyd_piwdqlvl_capture_cnt:[19:16]=0b0010
	// param_phyd_piwdqlvl_dly_step:[23:20]=0b0100
	// param_phyd_piwdqlvl_vref_step:[27:24]=0b0001
	// param_phyd_piwdqlvl_disable:[31:28]=0b0000
const DDR_PHY_REG_42_DATA: u32 = 0b00000000010101010000000000001010;
	// param_phyd_piwdqlvl_vref_wait_cnt:[7:0]=0b00001010
	// param_phyd_piwdqlvl_tx_vref_training_en:[8:8]=0b0
	// param_phyd_piwdqlvl_byte_invert_0:[23:16]=0b01010101
const DDR_PHY_REG_43_DATA: u32 = 0b00000000010101010011110001011010;
	// param_phyd_piwdqlvl_dq_pattern_0:[15:0]=0b0011110001011010
	// param_phyd_piwdqlvl_byte_invert_1:[23:16]=0b01010101
const DDR_PHY_REG_44_DATA: u32 = 0b00000000101010101010010111000011;
	// param_phyd_piwdqlvl_dq_pattern_1:[15:0]=0b1010010111000011
	// param_phyd_piwdqlvl_byte_invert_2:[23:16]=0b10101010
const DDR_PHY_REG_45_DATA: u32 = 0b00000000101010101111000011110000;
	// param_phyd_piwdqlvl_dq_pattern_2:[15:0]=0b1111000011110000
	// param_phyd_piwdqlvl_byte_invert_3:[23:16]=0b10101010
const DDR_PHY_REG_46_DATA: u32 = 0b00011110000000000000111100001111;
	// param_phyd_piwdqlvl_dq_pattern_3:[15:0]=0b0000111100001111
	// param_phyd_piwdqlvl_data_mask:[24:16]=0b000000000
	// param_phyd_piwdqlvl_pattern_sel:[28:25]=0b1111
const DDR_PHY_REG_47_DATA: u32 = 0b00000000000010000011111000010011;
	// param_phyd_piwdqlvl_tdfi_phy_wrdata:[2:0]=0b011
	// param_phyd_piwdqlvl_oenz_lead_cnt:[6:3]=0b0010
	// param_phyd_piwdqlvl_sw:[7:7]=0b0
	// param_phyd_piwdqlvl_sw_upd_req:[8:8]=0b0
	// param_phyd_piwdqlvl_sw_resp:[10:9]=0b11
	// param_phyd_piwdqlvl_sw_result:[11:11]=0b1
	// param_phyd_piwdqlvl_dq_mode:[12:12]=0b1
	// param_phyd_piwdqlvl_dm_mode:[13:13]=0b1
	// param_phyd_piwdqlvl_found_cnt_limite:[23:16]=0b00001000
const DDR_PHY_REG_60_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_patch_revision:[31:0]=0b00000000000000000000000000000000
const DDR_PHY_REG_61_DATA: u32 = 0b00000000000000110000000000110011;
	// param_phyd_ca_shift_gating_en:[0:0]=0b1
	// param_phyd_cs_shift_gating_en:[1:1]=0b1
	// param_phyd_cke_shift_gating_en:[2:2]=0b0
	// param_phyd_resetz_shift_gating_en:[3:3]=0b0
	// param_phyd_tx_byte0_shift_gating_en:[4:4]=0b1
	// param_phyd_tx_byte1_shift_gating_en:[5:5]=0b1
	// param_phyd_rx_byte0_shift_gating_en:[16:16]=0b1
	// param_phyd_rx_byte1_shift_gating_en:[17:17]=0b1
const DDR_PHY_REG_62_DATA: u32 = 0b00000000001000010000000000101100;
	// param_phyd_lb_lfsr_seed0:[8:0]=0b000101100
	// param_phyd_lb_lfsr_seed1:[24:16]=0b000100001
const DDR_PHY_REG_63_DATA: u32 = 0b00000000001101110000000000010110;
	// param_phyd_lb_lfsr_seed2:[8:0]=0b000010110
	// param_phyd_lb_lfsr_seed3:[24:16]=0b000110111
const DDR_PHY_REG_64_DATA: u32 = 0b00000100000000000000000000000000;
	// param_phyd_lb_dq_en:[0:0]=0b0
	// param_phyd_lb_dq_go:[1:1]=0b0
	// param_phyd_lb_sw_en:[2:2]=0b0
	// param_phyd_lb_sw_rx_en:[3:3]=0b0
	// param_phyd_lb_sw_rx_mask:[4:4]=0b0
	// param_phyd_lb_sw_odt_en:[5:5]=0b0
	// param_phyd_lb_sw_ca_clkpattern:[6:6]=0b0
	// param_phyd_lb_sync_len:[31:16]=0b0000010000000000
const DDR_PHY_REG_65_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_lb_sw_dout0:[8:0]=0b000000000
	// param_phyd_lb_sw_dout1:[24:16]=0b000000000
const DDR_PHY_REG_67_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_lb_sw_oenz_dout0:[0:0]=0b0
	// param_phyd_lb_sw_oenz_dout1:[1:1]=0b0
	// param_phyd_lb_sw_dqsn0:[4:4]=0b0
	// param_phyd_lb_sw_dqsn1:[5:5]=0b0
	// param_phyd_lb_sw_dqsp0:[8:8]=0b0
	// param_phyd_lb_sw_dqsp1:[9:9]=0b0
	// param_phyd_lb_sw_oenz_dqs_dout0:[12:12]=0b0
	// param_phyd_lb_sw_oenz_dqs_dout1:[13:13]=0b0
const DDR_PHY_REG_68_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_lb_sw_ca_dout:[22:0]=0b00000000000000000000000
const DDR_PHY_REG_69_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_lb_sw_clkn0_dout:[0:0]=0b0
	// param_phyd_lb_sw_clkp0_dout:[4:4]=0b0
	// param_phyd_lb_sw_cke0_dout:[8:8]=0b0
	// param_phyd_lb_sw_resetz_dout:[12:12]=0b0
	// param_phyd_lb_sw_csb0_dout:[16:16]=0b0
const DDR_PHY_REG_70_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_clkctrl_init_complete:[0:0]=0b0
const DDR_PHY_REG_71_DATA: u32 = 0b00000000000000000110101000010000;
	// param_phyd_reg_resetz_dqs_rd_en:[4:4]=0b1
	// param_phyd_rx_upd_tx_sel:[9:8]=0b10
	// param_phyd_tx_upd_rx_sel:[11:10]=0b10
	// param_phyd_rx_en_ext_win:[15:12]=0b0110
	// param_phyd_fifo_rst_sel:[18:16]=0b000
	// param_phyd_fifo_sw_rst:[20:20]=0b0
const DDR_PHY_REG_72_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_phy_int_ack:[31:0]=0b00000000000000000000000000000000
const DDR_PHY_REG_73_DATA: u32 = 0b11111111111111111111111011110111;
	// param_phyd_phy_int_mask:[31:0]=0b11111111111111111111111011110111
const DDR_PHY_REG_74_DATA: u32 = 0b00000000000000000000000000011111;
	// param_phyd_calvl_rst_n:[0:0]=0b1
	// param_phyd_pigtlvl_rst_n:[1:1]=0b1
	// param_phyd_pirdlvl_rst_n:[2:2]=0b1
	// param_phyd_piwdqlvl_rst_n:[3:3]=0b1
	// param_phyd_wrlvl_rst_n:[4:4]=0b1
const DDR_PHY_REG_75_DATA: u32 = 0b00000000000000000000000000000001;
	// param_phyd_clk0_pol:[0:0]=0b1
const DDR_PHY_REG_76_DATA: u32 = 0b00000000000000000000000100000001;
	// param_phyd_tx_ca_oenz:[0:0]=0b1
	// param_phyd_tx_ca_clk0_oenz:[8:8]=0b1
const DDR_PHY_REG_77_DATA: u32 = 0b00000000000000000000000100000000;
	// param_phya_reg_en_test:[0:0]=0b0
	// param_phya_reg_tx_ca_test_en:[1:1]=0b0
	// param_phya_reg_tx_ca_en_ca_loop_back:[2:2]=0b0
	// param_phya_reg_tx_sel_4bit_mode_tx:[8:8]=0b1
	// param_phya_reg_tx_gpio_in:[16:16]=0b0
const DDR_PHY_REG_78_DATA: u32 = 0b00000000000000000000000000010100;
	// param_phya_reg_rx_en_ca_train_mode:[0:0]=0b0
	// param_phya_reg_rx_en_pream_train_mode:[1:1]=0b0
	// param_phya_reg_rx_sel_dqs_wo_pream_mode:[2:2]=0b1
	// param_phya_reg_rx_en_rec_offset:[3:3]=0b0
	// param_phya_reg_rx_sel_4bit_mode_rx:[4:4]=0b1
const DDR_PHY_REG_80_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_rx_ddrdll_enautok:[0:0]=0b0
	// param_phya_reg_rx_ddrdll_rstb:[1:1]=0b0
	// param_phya_reg_rx_ddrdll_selckout:[5:4]=0b00
	// param_phya_reg_rx_ddrdll_test:[7:6]=0b00
	// param_phya_reg_rx_ddrdll_sel:[15:8]=0b00000000
	// param_phya_reg_tx_ddrdll_enautok:[16:16]=0b0
	// param_phya_reg_tx_ddrdll_rstb:[17:17]=0b0
	// param_phya_reg_tx_ddrdll_selckout:[21:20]=0b00
	// param_phya_reg_tx_ddrdll_test:[23:22]=0b00
	// param_phya_reg_tx_ddrdll_sel:[31:24]=0b00000000
const DDR_PHY_REG_81_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_tx_zq_cmp_en:[0:0]=0b0
	// param_phya_reg_tx_zq_cmp_offset_cal_en:[1:1]=0b0
	// param_phya_reg_tx_zq_ph_en:[2:2]=0b0
	// param_phya_reg_tx_zq_pl_en:[3:3]=0b0
	// param_phya_reg_tx_zq_step2_en:[4:4]=0b0
	// param_phya_reg_tx_zq_cmp_offset:[12:8]=0b00000
	// param_phya_reg_tx_zq_sel_vref:[20:16]=0b00000
const DDR_PHY_REG_82_DATA: u32 = 0b00000000000000000000100000001000;
	// param_phya_reg_tx_zq_golden_drvn:[4:0]=0b01000
	// param_phya_reg_tx_zq_golden_drvp:[12:8]=0b01000
	// param_phya_reg_tx_zq_drvn:[20:16]=0b00000
	// param_phya_reg_tx_zq_drvp:[28:24]=0b00000
const DDR_PHY_REG_83_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_tx_zq_en_test_aux:[0:0]=0b0
	// param_phya_reg_tx_zq_en_test_mux:[1:1]=0b0
	// param_phya_reg_sel_zq_high_swing:[2:2]=0b0
	// param_phya_reg_zq_sel_test_out0:[7:4]=0b0000
	// param_phya_reg_zq_sel_test_out1:[11:8]=0b0000
	// param_phya_reg_tx_zq_sel_test_ana_in:[15:12]=0b0000
	// param_phya_reg_tx_zq_sel_gpio_in:[17:16]=0b00
const DDR_PHY_REG_84_DATA: u32 = 0b00000000000000000000000000000101;
	// param_phya_reg_tune_damp_r:[3:0]=0b0101
const DDR_PHY_REG_85_DATA: u32 = 0b00000000000000000000000100000001;
	// param_phyd_sel_cke_oenz:[0:0]=0b1
	// param_phyd_tx_dqsn_default_value:[8:8]=0b1
	// param_phyd_tx_dqsp_default_value:[12:12]=0b0
	// param_phyd_ddr4_2t_preamble:[16:16]=0b0
const DDR_PHY_REG_86_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_zqcal_done:[0:0]=0b0
const DDR_PHY_REG_87_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_dbg_sel:[7:0]=0b00000000
	// param_phyd_dbg_sel_en:[16:16]=0b0
const DDR_PHY_REG_89_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_reg_dfs_sel:[0:0]=0b0
const DDR_PHY_REG_90_DATA: u32 = 0b00000000001100110011001100110001;
	// param_phyd_ca_sw_dline_en:[0:0]=0b1
	// param_phyd_byte0_wr_sw_dline_en:[4:4]=0b1
	// param_phyd_byte1_wr_sw_dline_en:[5:5]=0b1
	// param_phyd_byte0_wdqs_sw_dline_en:[8:8]=0b1
	// param_phyd_byte1_wdqs_sw_dline_en:[9:9]=0b1
	// param_phyd_byte0_rd_sw_dline_en:[12:12]=0b1
	// param_phyd_byte1_rd_sw_dline_en:[13:13]=0b1
	// param_phyd_byte0_rdg_sw_dline_en:[16:16]=0b1
	// param_phyd_byte1_rdg_sw_dline_en:[17:17]=0b1
	// param_phyd_byte0_rdqs_sw_dline_en:[20:20]=0b1
	// param_phyd_byte1_rdqs_sw_dline_en:[21:21]=0b1
const DDR_PHY_REG_91_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_ca_raw_dline_upd:[0:0]=0b0
	// param_phyd_byte0_wr_raw_dline_upd:[4:4]=0b0
	// param_phyd_byte1_wr_raw_dline_upd:[5:5]=0b0
	// param_phyd_byte0_wdqs_raw_dline_upd:[8:8]=0b0
	// param_phyd_byte1_wdqs_raw_dline_upd:[9:9]=0b0
	// param_phyd_byte0_rd_raw_dline_upd:[12:12]=0b0
	// param_phyd_byte1_rd_raw_dline_upd:[13:13]=0b0
	// param_phyd_byte0_rdg_raw_dline_upd:[16:16]=0b0
	// param_phyd_byte1_rdg_raw_dline_upd:[17:17]=0b0
	// param_phyd_byte0_rdqs_raw_dline_upd:[20:20]=0b0
	// param_phyd_byte1_rdqs_raw_dline_upd:[21:21]=0b0
const DDR_PHY_REG_92_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_sw_dline_upd_req:[0:0]=0b0
const DDR_PHY_REG_93_DATA: u32 = 0b00000000000000000000000100000000;
	// param_phyd_sw_dfi_phyupd_req:[0:0]=0b0
	// param_phyd_sw_dfi_phyupd_req_clr:[4:4]=0b0
	// param_phyd_sw_phyupd_dline:[8:8]=0b1
const DDR_PHY_REG_96_DATA: u32 = 0b00000000000000100000000000010000;
	// param_phyd_dfi_wrlvl_req:[0:0]=0b0
	// param_phyd_dfi_wrlvl_odt_en:[4:4]=0b1
	// param_phyd_dfi_wrlvl_strobe_cnt:[19:16]=0b0010
const DDR_PHY_REG_97_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_dfi_rdglvl_req:[0:0]=0b0
	// param_phyd_dfi_rdglvl_ddr3_mpr:[4:4]=0b0
const DDR_PHY_REG_98_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_dfi_rdlvl_req:[0:0]=0b0
	// param_phyd_dfi_rdlvl_ddr3_mpr:[4:4]=0b0
const DDR_PHY_REG_99_DATA: u32 = 0b00000000000010010000010000000000;
	// param_phyd_dfi_wdqlvl_req:[0:0]=0b0
	// param_phyd_dfi_wdqlvl_bist_data_en:[4:4]=0b0
	// param_phyd_dfi_wdqlvl_vref_train_en:[10:10]=0b1
	// param_phyd_dfi_wdqlvl_vref_wait_cnt:[23:16]=0b00001001
const DDR_PHY_REG_100_DATA: u32 = 0b00000000000000100001001000001110;
	// param_phyd_dfi_wdqlvl_vref_start:[6:0]=0b0001110
	// param_phyd_dfi_wdqlvl_vref_end:[14:8]=0b0010010
	// param_phyd_dfi_wdqlvl_vref_step:[19:16]=0b0010
const DDR_PHY_REG_128_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_byte0_test_en:[0:0]=0b0
	// param_phya_reg_tx_byte0_ddr_test:[15:8]=0b00000000
	// param_phya_reg_rx_byte0_sel_test_in0:[19:16]=0b0000
	// param_phya_reg_rx_byte0_sel_test_in1:[23:20]=0b0000
const DDR_PHY_REG_129_DATA: u32 = 0b00000000000000000000010001000000;
	// param_phya_reg_tx_byte0_en_rx_awys_on:[0:0]=0b0
	// param_phya_reg_tx_byte0_sel_en_rx_dly:[5:4]=0b00
	// param_phya_reg_rx_byte0_sel_en_rx_gen_rst:[6:6]=0b1
	// param_phya_reg_byte0_mask_oenz:[8:8]=0b0
	// param_phya_reg_tx_byte0_en_mask:[10:10]=0b1
	// param_phya_reg_rx_byte0_sel_cnt_mode:[13:12]=0b00
	// param_phya_reg_tx_byte0_sel_int_loop_back:[14:14]=0b0
	// param_phya_reg_rx_byte0_sel_dqs_dly_for_gated:[17:16]=0b00
	// param_phya_reg_tx_byte0_en_extend_oenz_gated_dline:[18:18]=0b0
const DDR_PHY_REG_130_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_reg_reserved_byte0:[31:0]=0b00000000000000000000000000000000
const DDR_PHY_REG_136_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phya_reg_byte1_test_en:[0:0]=0b0
	// param_phya_reg_tx_byte1_ddr_test:[15:8]=0b00000000
	// param_phya_reg_rx_byte1_sel_test_in0:[19:16]=0b0000
	// param_phya_reg_rx_byte1_sel_test_in1:[23:20]=0b0000
const DDR_PHY_REG_137_DATA: u32 = 0b00000000000000000000010001000000;
	// param_phya_reg_tx_byte1_en_rx_awys_on:[0:0]=0b0
	// param_phya_reg_tx_byte1_sel_en_rx_dly:[5:4]=0b00
	// param_phya_reg_rx_byte1_sel_en_rx_gen_rst:[6:6]=0b1
	// param_phya_reg_byte1_mask_oenz:[8:8]=0b0
	// param_phya_reg_tx_byte1_en_mask:[10:10]=0b1
	// param_phya_reg_rx_byte1_sel_cnt_mode:[13:12]=0b00
	// param_phya_reg_tx_byte1_sel_int_loop_back:[14:14]=0b0
	// param_phya_reg_rx_byte1_sel_dqs_dly_for_gated:[17:16]=0b00
	// param_phya_reg_tx_byte1_en_extend_oenz_gated_dline:[18:18]=0b0
const DDR_PHY_REG_138_DATA: u32 = 0b00000000000000000000000000000000;
	// param_phyd_reg_reserved_byte1:[31:0]=0b00000000000000000000000000000000
const DDR_PHY_REG_0_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_ca_sel_lpddr4_pmos_ph_ca:[3:3]=0b0
	// f0_param_phya_reg_tx_clk_sel_lpddr4_pmos_ph_clk:[4:4]=0b0
	// f0_param_phya_reg_tx_sel_lpddr4_pmos_ph:[5:5]=0b0
const DDR_PHY_REG_1_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_ca_drvn_de:[1:0]=0b00
	// f0_param_phya_reg_tx_ca_drvp_de:[5:4]=0b00
	// f0_param_phya_reg_tx_clk0_drvn_de:[9:8]=0b00
	// f0_param_phya_reg_tx_clk0_drvp_de:[13:12]=0b00
	// f0_param_phya_reg_tx_csb_drvn_de:[17:16]=0b00
	// f0_param_phya_reg_tx_csb_drvp_de:[21:20]=0b00
	// f0_param_phya_reg_tx_ca_en_tx_de:[24:24]=0b0
	// f0_param_phya_reg_tx_clk0_en_tx_de:[28:28]=0b0
	// f0_param_phya_reg_tx_csb_en_tx_de:[30:30]=0b0
const DDR_PHY_REG_2_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_ca_sel_dly1t_ca:[22:0]=0b00000000000000000000000
const DDR_PHY_REG_3_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_clk_sel_dly1t_clk0:[0:0]=0b0
	// f0_param_phya_reg_tx_ca_sel_dly1t_cke0:[8:8]=0b0
	// f0_param_phya_reg_tx_ca_sel_dly1t_csb0:[16:16]=0b0
const DDR_PHY_REG_4_F0_DATA: u32 = 0b00000000000100000000000000000000;
	// f0_param_phya_reg_tx_vref_en_free_offset:[0:0]=0b0
	// f0_param_phya_reg_tx_vref_en_rangex2:[1:1]=0b0
	// f0_param_phya_reg_tx_vref_sel_lpddr4divby2p5:[2:2]=0b0
	// f0_param_phya_reg_tx_vref_sel_lpddr4divby3:[3:3]=0b0
	// f0_param_phya_reg_tx_vref_offset:[14:8]=0b0000000
	// f0_param_phya_reg_tx_vref_sel:[20:16]=0b10000
const DDR_PHY_REG_5_F0_DATA: u32 = 0b00000000000100000000000000000000;
	// f0_param_phya_reg_tx_vrefca_en_free_offset:[0:0]=0b0
	// f0_param_phya_reg_tx_vrefca_en_rangex2:[1:1]=0b0
	// f0_param_phya_reg_tx_vrefca_offset:[14:8]=0b0000000
	// f0_param_phya_reg_tx_vrefca_sel:[20:16]=0b10000
const DDR_PHY_REG_6_F0_DATA: u32 = 0b00000000000000000000000000000010;
	// f0_param_phyd_tx_byte_dqs_extend:[2:0]=0b010
const DDR_PHY_REG_7_F0_DATA: u32 = 0b00000000000000000100000001000000;
	// f0_param_phya_reg_rx_byte0_odt_reg:[4:0]=0b00000
	// f0_param_phya_reg_rx_byte0_sel_odt_reg_mode:[6:6]=0b1
	// f0_param_phya_reg_rx_byte1_odt_reg:[12:8]=0b00000
	// f0_param_phya_reg_rx_byte1_sel_odt_reg_mode:[14:14]=0b1
const DDR_PHY_REG_64_F0_DATA: u32 = 0b00000000000001000001000000000001;
	// f0_param_phya_reg_rx_byte0_en_lsmode:[0:0]=0b1
	// f0_param_phya_reg_rx_byte0_hystr:[5:4]=0b00
	// f0_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode:[8:8]=0b0
	// f0_param_phya_reg_rx_byte0_sel_odt_center_tap:[10:10]=0b0
	// f0_param_phya_reg_byte0_en_rec_vol_mode:[12:12]=0b1
	// f0_param_phya_reg_tx_byte0_force_en_lvstl_ph:[14:14]=0b0
	// f0_param_phya_reg_rx_byte0_force_en_lvstl_odt:[16:16]=0b0
	// f0_param_phya_reg_rx_byte0_en_trig_lvl_rangex2:[18:18]=0b1
	// f0_param_phya_reg_rx_byte0_trig_lvl_en_free_offset:[20:20]=0b0
const DDR_PHY_REG_65_F0_DATA: u32 = 0b00000000000100000000000000000000;
	// f0_param_phya_reg_tx_byte0_drvn_de_dq:[1:0]=0b00
	// f0_param_phya_reg_tx_byte0_drvp_de_dq:[5:4]=0b00
	// f0_param_phya_reg_tx_byte0_drvn_de_dqs:[9:8]=0b00
	// f0_param_phya_reg_tx_byte0_drvp_de_dqs:[13:12]=0b00
	// f0_param_phya_reg_tx_byte0_en_tx_de_dq:[16:16]=0b0
	// f0_param_phya_reg_tx_byte0_en_tx_de_dqs:[20:20]=0b1
const DDR_PHY_REG_66_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_byte0_sel_dly1t_dq:[8:0]=0b000000000
	// f0_param_phya_reg_tx_byte0_sel_dly1t_dqs:[12:12]=0b0
	// f0_param_phya_reg_tx_byte0_sel_dly1t_mask_ranka:[16:16]=0b0
const DDR_PHY_REG_67_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_byte0_vref_sel_lpddr4divby2p5:[0:0]=0b0
	// f0_param_phya_reg_tx_byte0_vref_sel_lpddr4divby3:[4:4]=0b0
	// f0_param_phya_reg_tx_byte0_vref_sel_lpddr4x_voh0p5:[8:8]=0b0
	// f0_param_phya_reg_tx_byte0_vref_sel_lpddr4x_voh0p6:[12:12]=0b0
const DDR_PHY_REG_68_F0_DATA: u32 = 0b00000000000000000000000000000100;
	// f0_param_phyd_reg_rx_byte0_resetz_dqs_offset:[3:0]=0b0100
const DDR_PHY_REG_69_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte0_dq0_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte0_dq1_offset:[14:8]=0b0000000
	// f0_param_phyd_reg_byte0_dq2_offset:[22:16]=0b0000000
	// f0_param_phyd_reg_byte0_dq3_offset:[30:24]=0b0000000
const DDR_PHY_REG_70_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte0_dq4_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte0_dq5_offset:[14:8]=0b0000000
	// f0_param_phyd_reg_byte0_dq6_offset:[22:16]=0b0000000
	// f0_param_phyd_reg_byte0_dq7_offset:[30:24]=0b0000000
const DDR_PHY_REG_71_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte0_dm_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte0_dqsn_offset:[19:16]=0b0000
	// f0_param_phyd_reg_byte0_dqsp_offset:[27:24]=0b0000
const DDR_PHY_REG_72_F0_DATA: u32 = 0b00000000000000000000000000000011;
	// f0_param_phyd_tx_byte0_tx_oenz_extend:[2:0]=0b011
const DDR_PHY_REG_80_F0_DATA: u32 = 0b00000000000001000001000000000001;
	// f0_param_phya_reg_rx_byte1_en_lsmode:[0:0]=0b1
	// f0_param_phya_reg_rx_byte1_hystr:[5:4]=0b00
	// f0_param_phya_reg_rx_byte1_sel_dqs_rec_vref_mode:[8:8]=0b0
	// f0_param_phya_reg_rx_byte1_sel_odt_center_tap:[10:10]=0b0
	// f0_param_phya_reg_byte1_en_rec_vol_mode:[12:12]=0b1
	// f0_param_phya_reg_tx_byte1_force_en_lvstl_ph:[14:14]=0b0
	// f0_param_phya_reg_rx_byte1_force_en_lvstl_odt:[16:16]=0b0
	// f0_param_phya_reg_rx_byte1_en_trig_lvl_rangex2:[18:18]=0b1
	// f0_param_phya_reg_rx_byte1_trig_lvl_en_free_offset:[20:20]=0b0
const DDR_PHY_REG_81_F0_DATA: u32 = 0b00000000000100000000000000000000;
	// f0_param_phya_reg_tx_byte1_drvn_de_dq:[1:0]=0b00
	// f0_param_phya_reg_tx_byte1_drvp_de_dq:[5:4]=0b00
	// f0_param_phya_reg_tx_byte1_drvn_de_dqs:[9:8]=0b00
	// f0_param_phya_reg_tx_byte1_drvp_de_dqs:[13:12]=0b00
	// f0_param_phya_reg_tx_byte1_en_tx_de_dq:[16:16]=0b0
	// f0_param_phya_reg_tx_byte1_en_tx_de_dqs:[20:20]=0b1
const DDR_PHY_REG_82_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_byte1_sel_dly1t_dq:[8:0]=0b000000000
	// f0_param_phya_reg_tx_byte1_sel_dly1t_dqs:[12:12]=0b0
	// f0_param_phya_reg_tx_byte1_sel_dly1t_mask_ranka:[16:16]=0b0
const DDR_PHY_REG_83_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_tx_byte1_vref_sel_lpddr4divby2p5:[0:0]=0b0
	// f0_param_phya_reg_tx_byte1_vref_sel_lpddr4divby3:[4:4]=0b0
	// f0_param_phya_reg_tx_byte1_vref_sel_lpddr4x_voh0p5:[8:8]=0b0
	// f0_param_phya_reg_tx_byte1_vref_sel_lpddr4x_voh0p6:[12:12]=0b0
const DDR_PHY_REG_84_F0_DATA: u32 = 0b00000000000000000000000000000100;
	// f0_param_phyd_reg_rx_byte1_resetz_dqs_offset:[3:0]=0b0100
const DDR_PHY_REG_85_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte1_dq0_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte1_dq1_offset:[14:8]=0b0000000
	// f0_param_phyd_reg_byte1_dq2_offset:[22:16]=0b0000000
	// f0_param_phyd_reg_byte1_dq3_offset:[30:24]=0b0000000
const DDR_PHY_REG_86_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte1_dq4_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte1_dq5_offset:[14:8]=0b0000000
	// f0_param_phyd_reg_byte1_dq6_offset:[22:16]=0b0000000
	// f0_param_phyd_reg_byte1_dq7_offset:[30:24]=0b0000000
const DDR_PHY_REG_87_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_byte1_dm_offset:[6:0]=0b0000000
	// f0_param_phyd_reg_byte1_dqsn_offset:[19:16]=0b0000
	// f0_param_phyd_reg_byte1_dqsp_offset:[27:24]=0b0000
const DDR_PHY_REG_88_F0_DATA: u32 = 0b00000000000000000000000000000011;
	// f0_param_phyd_tx_byte1_tx_oenz_extend:[2:0]=0b011
const DDR_PHY_REG_320_F0_DATA: u32 = 0b00000000000000000000010000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_ca0_sw:[6:0]=0b0000000
	// f0_param_phyd_tx_ca0_shift_sel:[13:8]=0b000100
const DDR_PHY_REG_331_F0_DATA: u32 = 0b00000000000000000000010000000000;
	// f0_param_phyd_tx_ca22_shift_sel:[13:8]=0b000100
const DDR_PHY_REG_332_F0_DATA: u32 = 0b00000000000000000000010000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_cke0_sw:[6:0]=0b0000000
	// f0_param_phyd_tx_cke0_shift_sel:[13:8]=0b000100
const DDR_PHY_REG_333_F0_DATA: u32 = 0b00000000000000000000010000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_csb0_sw:[6:0]=0b0000000
	// f0_param_phyd_tx_cs0_shift_sel:[13:8]=0b000100
const DDR_PHY_REG_334_F0_DATA: u32 = 0b00000000000000000000010000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_resetz_sw:[6:0]=0b0000000
	// f0_param_phyd_tx_reset_shift_sel:[13:8]=0b000100
const DDR_PHY_REG_336_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_ca0_raw:[6:0]=0b0000000
const DDR_PHY_REG_348_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_cke0_raw:[6:0]=0b0000000
const DDR_PHY_REG_349_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_csb0_raw:[6:0]=0b0000000
const DDR_PHY_REG_350_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_ca_tx_dline_code_resetz_raw:[6:0]=0b0000000
const DDR_PHY_REG_351_F0_DATA: u32 = 0b00001000000010000000010000000100;
	// f0_param_phya_reg_tx_ca_drvn_ca:[4:0]=0b00100
	// f0_param_phya_reg_tx_ca_drvp_ca:[12:8]=0b00100
	// f0_param_phya_reg_tx_ca_drvn_csb:[20:16]=0b01000
	// f0_param_phya_reg_tx_ca_drvp_csb:[28:24]=0b01000
const DDR_PHY_REG_352_F0_DATA: u32 = 0b00001000000010000000100000001000;
	// f0_param_phya_reg_tx_clk_drvn_clkn0:[4:0]=0b01000
	// f0_param_phya_reg_tx_clk_drvp_clkn0:[12:8]=0b01000
	// f0_param_phya_reg_tx_clk_drvn_clkp0:[20:16]=0b01000
	// f0_param_phya_reg_tx_clk_drvp_clkp0:[28:24]=0b01000
const DDR_PHY_REG_384_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq0_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte0_bit0_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq1_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte0_bit1_data_shift:[29:24]=0b000110
const DDR_PHY_REG_385_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq2_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte0_bit2_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq3_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte0_bit3_data_shift:[29:24]=0b000110
const DDR_PHY_REG_386_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq4_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte0_bit4_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq5_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte0_bit5_data_shift:[29:24]=0b000110
const DDR_PHY_REG_387_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq6_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte0_bit6_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq7_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte0_bit7_data_shift:[29:24]=0b000110
const DDR_PHY_REG_388_F0_DATA: u32 = 0b00000000000000000000011001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq8_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte0_bit8_data_shift:[13:8]=0b000110
const DDR_PHY_REG_389_F0_DATA: u32 = 0b00001011000000000000000000000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dqsn_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dqsp_sw:[22:16]=0b0000000
	// f0_param_phyd_tx_byte0_dqs_shift:[29:24]=0b001011
const DDR_PHY_REG_390_F0_DATA: u32 = 0b00001010000000000000100100000000;
	// f0_param_phyd_tx_byte0_oenz_dqs_shift:[13:8]=0b001001
	// f0_param_phyd_tx_byte0_oenz_shift:[29:24]=0b001010
const DDR_PHY_REG_391_F0_DATA: u32 = 0b00000000000001000000011000000000;
	// f0_param_phyd_tx_byte0_oenz_dqs_extend:[11:8]=0b0110
	// f0_param_phyd_tx_byte0_oenz_extend:[19:16]=0b0100
const DDR_PHY_REG_392_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq0_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq1_raw:[22:16]=0b1000000
const DDR_PHY_REG_393_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq2_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq3_raw:[22:16]=0b1000000
const DDR_PHY_REG_394_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq4_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq5_raw:[22:16]=0b1000000
const DDR_PHY_REG_395_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq6_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq7_raw:[22:16]=0b1000000
const DDR_PHY_REG_396_F0_DATA: u32 = 0b00000000000000000000000001000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dq8_raw:[6:0]=0b1000000
const DDR_PHY_REG_397_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dqsn_raw:[6:0]=0b0000000
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_dqsp_raw:[22:16]=0b0000000
const DDR_PHY_REG_398_F0_DATA: u32 = 0b00000000000000000000100000001000;
	// f0_param_phya_reg_tx_byte0_drvn_dq:[4:0]=0b01000
	// f0_param_phya_reg_tx_byte0_drvp_dq:[12:8]=0b01000
const DDR_PHY_REG_399_F0_DATA: u32 = 0b00001000000010000000100000001000;
	// f0_param_phya_reg_tx_byte0_drvn_dqsn:[4:0]=0b01000
	// f0_param_phya_reg_tx_byte0_drvp_dqsn:[12:8]=0b01000
	// f0_param_phya_reg_tx_byte0_drvn_dqsp:[20:16]=0b01000
	// f0_param_phya_reg_tx_byte0_drvp_dqsp:[28:24]=0b01000
const DDR_PHY_REG_400_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq0_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte1_bit0_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq1_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte1_bit1_data_shift:[29:24]=0b000110
const DDR_PHY_REG_401_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq2_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte1_bit2_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq3_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte1_bit3_data_shift:[29:24]=0b000110
const DDR_PHY_REG_402_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq4_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte1_bit4_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq5_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte1_bit5_data_shift:[29:24]=0b000110
const DDR_PHY_REG_403_F0_DATA: u32 = 0b00000110010000000000011001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq6_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte1_bit6_data_shift:[13:8]=0b000110
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq7_sw:[22:16]=0b1000000
	// f0_param_phyd_tx_byte1_bit7_data_shift:[29:24]=0b000110
const DDR_PHY_REG_404_F0_DATA: u32 = 0b00000000000000000000011001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq8_sw:[6:0]=0b1000000
	// f0_param_phyd_tx_byte1_bit8_data_shift:[13:8]=0b000110
const DDR_PHY_REG_405_F0_DATA: u32 = 0b00001011000000000000000000000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dqsn_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dqsp_sw:[22:16]=0b0000000
	// f0_param_phyd_tx_byte1_dqs_shift:[29:24]=0b001011
const DDR_PHY_REG_406_F0_DATA: u32 = 0b00001010000000000000100100000000;
	// f0_param_phyd_tx_byte1_oenz_dqs_shift:[13:8]=0b001001
	// f0_param_phyd_tx_byte1_oenz_shift:[29:24]=0b001010
const DDR_PHY_REG_407_F0_DATA: u32 = 0b00000000000001000000011000000000;
	// f0_param_phyd_tx_byte1_oenz_dqs_extend:[11:8]=0b0110
	// f0_param_phyd_tx_byte1_oenz_extend:[19:16]=0b0100
const DDR_PHY_REG_408_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq0_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq1_raw:[22:16]=0b1000000
const DDR_PHY_REG_409_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq2_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq3_raw:[22:16]=0b1000000
const DDR_PHY_REG_410_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq4_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq5_raw:[22:16]=0b1000000
const DDR_PHY_REG_411_F0_DATA: u32 = 0b00000000010000000000000001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq6_raw:[6:0]=0b1000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq7_raw:[22:16]=0b1000000
const DDR_PHY_REG_412_F0_DATA: u32 = 0b00000000000000000000000001000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dq8_raw:[6:0]=0b1000000
const DDR_PHY_REG_413_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dqsn_raw:[6:0]=0b0000000
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_dqsp_raw:[22:16]=0b0000000
const DDR_PHY_REG_414_F0_DATA: u32 = 0b00000000000000000000100000001000;
	// f0_param_phya_reg_tx_byte1_drvn_dq:[4:0]=0b01000
	// f0_param_phya_reg_tx_byte1_drvp_dq:[12:8]=0b01000
const DDR_PHY_REG_415_F0_DATA: u32 = 0b00001000000010000000100000001000;
	// f0_param_phya_reg_tx_byte1_drvn_dqsn:[4:0]=0b01000
	// f0_param_phya_reg_tx_byte1_drvp_dqsn:[12:8]=0b01000
	// f0_param_phya_reg_tx_byte1_drvn_dqsp:[20:16]=0b01000
	// f0_param_phya_reg_tx_byte1_drvp_dqsp:[28:24]=0b01000
const DDR_PHY_REG_448_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_rx_byte0_rx_dq0_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq1_deskew_sw:[14:8]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq2_deskew_sw:[22:16]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq3_deskew_sw:[30:24]=0b0000000
const DDR_PHY_REG_449_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_rx_byte0_rx_dq4_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq5_deskew_sw:[14:8]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq6_deskew_sw:[22:16]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dq7_deskew_sw:[30:24]=0b0000000
const DDR_PHY_REG_450_F0_DATA: u32 = 0b00000000010000000100000000000000;
	// f0_param_phyd_reg_rx_byte0_rx_dq8_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte0_rx_dqs_dlie_code_neg_ranka_sw:[15:8]=0b01000000
	// f0_param_phyd_reg_rx_byte0_rx_dqs_dlie_code_pos_ranka_sw:[23:16]=0b01000000
const DDR_PHY_REG_451_F0_DATA: u32 = 0b00000000000000000000101101000000;
	// f0_param_phyd_reg_tx_byte0_tx_dline_code_mask_ranka_sw:[6:0]=0b1000000
	// f0_param_phyd_rx_byte0_mask_shift:[13:8]=0b001011
const DDR_PHY_REG_452_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_rx_byte0_en_shift:[13:8]=0b000000
	// f0_param_phyd_rx_byte0_odt_en_shift:[29:24]=0b000000
const DDR_PHY_REG_453_F0_DATA: u32 = 0b00000000000010000000111000001110;
	// f0_param_phyd_rx_byte0_en_extend:[3:0]=0b1110
	// f0_param_phyd_rx_byte0_odt_en_extend:[11:8]=0b1110
	// f0_param_phyd_rx_byte0_rden_to_rdvld:[20:16]=0b01000
const DDR_PHY_REG_454_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_rx_byte0_rx_dq0_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq1_deskew_raw:[12:8]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq2_deskew_raw:[20:16]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq3_deskew_raw:[28:24]=0b00000
const DDR_PHY_REG_455_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_rx_byte0_rx_dq4_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq5_deskew_raw:[12:8]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq6_deskew_raw:[20:16]=0b00000
	// f0_param_phya_reg_rx_byte0_rx_dq7_deskew_raw:[28:24]=0b00000
const DDR_PHY_REG_456_F0_DATA: u32 = 0b01000000010000000000000000000000;
	// f0_param_phya_reg_rx_byte0_rx_dq8_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_tx_byte0_tx_dline_code_mask_ranka_raw:[14:8]=0b0000000
	// f0_param_phya_reg_rx_byte0_rx_dqs_dlie_code_neg_ranka_raw:[22:16]=0b1000000
	// f0_param_phya_reg_rx_byte0_rx_dqs_dlie_code_pos_ranka_raw:[30:24]=0b1000000
const DDR_PHY_REG_457_F0_DATA: u32 = 0b00000000000100000000000000010000;
	// f0_param_phya_reg_rx_byte0_trig_lvl_dq:[4:0]=0b10000
	// f0_param_phya_reg_rx_byte0_trig_lvl_dq_offset:[14:8]=0b0000000
	// f0_param_phya_reg_rx_byte0_trig_lvl_dqs:[20:16]=0b10000
	// f0_param_phya_reg_rx_byte0_trig_lvl_dqs_offset:[30:24]=0b0000000
const DDR_PHY_REG_460_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_rx_byte1_rx_dq0_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq1_deskew_sw:[14:8]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq2_deskew_sw:[22:16]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq3_deskew_sw:[30:24]=0b0000000
const DDR_PHY_REG_461_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_reg_rx_byte1_rx_dq4_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq5_deskew_sw:[14:8]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq6_deskew_sw:[22:16]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dq7_deskew_sw:[30:24]=0b0000000
const DDR_PHY_REG_462_F0_DATA: u32 = 0b00000000010000000100000000000000;
	// f0_param_phyd_reg_rx_byte1_rx_dq8_deskew_sw:[6:0]=0b0000000
	// f0_param_phyd_reg_rx_byte1_rx_dqs_dlie_code_neg_ranka_sw:[15:8]=0b01000000
	// f0_param_phyd_reg_rx_byte1_rx_dqs_dlie_code_pos_ranka_sw:[23:16]=0b01000000
const DDR_PHY_REG_463_F0_DATA: u32 = 0b00000000000000000000101101000000;
	// f0_param_phyd_reg_tx_byte1_tx_dline_code_mask_ranka_sw:[6:0]=0b1000000
	// f0_param_phyd_rx_byte1_mask_shift:[13:8]=0b001011
const DDR_PHY_REG_464_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phyd_rx_byte1_en_shift:[13:8]=0b000000
	// f0_param_phyd_rx_byte1_odt_en_shift:[29:24]=0b000000
const DDR_PHY_REG_465_F0_DATA: u32 = 0b00000000000010000000111000001110;
	// f0_param_phyd_rx_byte1_en_extend:[3:0]=0b1110
	// f0_param_phyd_rx_byte1_odt_en_extend:[11:8]=0b1110
	// f0_param_phyd_rx_byte1_rden_to_rdvld:[20:16]=0b01000
const DDR_PHY_REG_466_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_rx_byte1_rx_dq0_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq1_deskew_raw:[12:8]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq2_deskew_raw:[20:16]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq3_deskew_raw:[28:24]=0b00000
const DDR_PHY_REG_467_F0_DATA: u32 = 0b00000000000000000000000000000000;
	// f0_param_phya_reg_rx_byte1_rx_dq4_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq5_deskew_raw:[12:8]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq6_deskew_raw:[20:16]=0b00000
	// f0_param_phya_reg_rx_byte1_rx_dq7_deskew_raw:[28:24]=0b00000
const DDR_PHY_REG_468_F0_DATA: u32 = 0b01000000010000000000000000000000;
	// f0_param_phya_reg_rx_byte1_rx_dq8_deskew_raw:[4:0]=0b00000
	// f0_param_phya_reg_tx_byte1_tx_dline_code_mask_ranka_raw:[14:8]=0b0000000
	// f0_param_phya_reg_rx_byte1_rx_dqs_dlie_code_neg_ranka_raw:[22:16]=0b1000000
	// f0_param_phya_reg_rx_byte1_rx_dqs_dlie_code_pos_ranka_raw:[30:24]=0b1000000
const DDR_PHY_REG_469_F0_DATA: u32 = 0b00000000000100000000000000010000;
	// f0_param_phya_reg_rx_byte1_trig_lvl_dq:[4:0]=0b10000
	// f0_param_phya_reg_rx_byte1_trig_lvl_dq_offset:[14:8]=0b0000000
	// f0_param_phya_reg_rx_byte1_trig_lvl_dqs:[20:16]=0b10000
	// f0_param_phya_reg_rx_byte1_trig_lvl_dqs_offset:[30:24]=0b0000000

//     #ifdef F1_TEST
// const DDR_PHY_REG_0_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_ca_sel_lpddr4_pmos_ph_ca:[3:3]=0b0
// 	// f1_param_phya_reg_tx_clk_sel_lpddr4_pmos_ph_clk:[4:4]=0b0
// 	// f1_param_phya_reg_tx_sel_lpddr4_pmos_ph:[5:5]=0b0
// const DDR_PHY_REG_1_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_ca_drvn_de:[1:0]=0b00
// 	// f1_param_phya_reg_tx_ca_drvp_de:[5:4]=0b00
// 	// f1_param_phya_reg_tx_clk0_drvn_de:[9:8]=0b00
// 	// f1_param_phya_reg_tx_clk0_drvp_de:[13:12]=0b00
// 	// f1_param_phya_reg_tx_csb_drvn_de:[17:16]=0b00
// 	// f1_param_phya_reg_tx_csb_drvp_de:[21:20]=0b00
// 	// f1_param_phya_reg_tx_ca_en_tx_de:[24:24]=0b0
// 	// f1_param_phya_reg_tx_clk0_en_tx_de:[28:28]=0b0
// 	// f1_param_phya_reg_tx_csb_en_tx_de:[30:30]=0b0
// const DDR_PHY_REG_2_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_ca_sel_dly1t_ca:[22:0]=0b00000000000000000000000
// const DDR_PHY_REG_3_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_clk_sel_dly1t_clk0:[0:0]=0b0
// 	// f1_param_phya_reg_tx_ca_sel_dly1t_cke0:[8:8]=0b0
// 	// f1_param_phya_reg_tx_ca_sel_dly1t_csb0:[16:16]=0b0
// const DDR_PHY_REG_4_F1_DATA: u32 = 0b00000000000100000000000000000000;
// 	// f1_param_phya_reg_tx_vref_en_free_offset:[0:0]=0b0
// 	// f1_param_phya_reg_tx_vref_en_rangex2:[1:1]=0b0
// 	// f1_param_phya_reg_tx_vref_sel_lpddr4divby2p5:[2:2]=0b0
// 	// f1_param_phya_reg_tx_vref_sel_lpddr4divby3:[3:3]=0b0
// 	// f1_param_phya_reg_tx_vref_offset:[14:8]=0b0000000
// 	// f1_param_phya_reg_tx_vref_sel:[20:16]=0b10000
// const DDR_PHY_REG_5_F1_DATA: u32 = 0b00000000000100000000000000000000;
// 	// f1_param_phya_reg_tx_vrefca_en_free_offset:[0:0]=0b0
// 	// f1_param_phya_reg_tx_vrefca_en_rangex2:[1:1]=0b0
// 	// f1_param_phya_reg_tx_vrefca_offset:[14:8]=0b0000000
// 	// f1_param_phya_reg_tx_vrefca_sel:[20:16]=0b10000
// const DDR_PHY_REG_6_F1_DATA: u32 = 0b00000000000000000000000000000010;
// 	// f1_param_phyd_tx_byte_dqs_extend:[2:0]=0b010
// const DDR_PHY_REG_7_F1_DATA: u32 = 0b00000000000000000100000001000000;
// 	// f1_param_phya_reg_rx_byte0_odt_reg:[4:0]=0b00000
// 	// f1_param_phya_reg_rx_byte0_sel_odt_reg_mode:[6:6]=0b1
// 	// f1_param_phya_reg_rx_byte1_odt_reg:[12:8]=0b00000
// 	// f1_param_phya_reg_rx_byte1_sel_odt_reg_mode:[14:14]=0b1
// const DDR_PHY_REG_64_F1_DATA: u32 = 0b00000000000000000001000000000001;
// 	// f1_param_phya_reg_rx_byte0_en_lsmode:[0:0]=0b1
// 	// f1_param_phya_reg_rx_byte0_hystr:[5:4]=0b00
// 	// f1_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode:[8:8]=0b0
// 	// f1_param_phya_reg_rx_byte0_sel_odt_center_tap:[10:10]=0b0
// 	// f1_param_phya_reg_byte0_en_rec_vol_mode:[12:12]=0b1
// 	// f1_param_phya_reg_tx_byte0_force_en_lvstl_ph:[14:14]=0b0
// 	// f1_param_phya_reg_rx_byte0_force_en_lvstl_odt:[16:16]=0b0
// 	// f1_param_phya_reg_rx_byte0_en_trig_lvl_rangex2:[18:18]=0b0
// 	// f1_param_phya_reg_rx_byte0_trig_lvl_en_free_offset:[20:20]=0b0
// const DDR_PHY_REG_65_F1_DATA: u32 = 0b00000000000100000000000000000000;
// 	// f1_param_phya_reg_tx_byte0_drvn_de_dq:[1:0]=0b00
// 	// f1_param_phya_reg_tx_byte0_drvp_de_dq:[5:4]=0b00
// 	// f1_param_phya_reg_tx_byte0_drvn_de_dqs:[9:8]=0b00
// 	// f1_param_phya_reg_tx_byte0_drvp_de_dqs:[13:12]=0b00
// 	// f1_param_phya_reg_tx_byte0_en_tx_de_dq:[16:16]=0b0
// 	// f1_param_phya_reg_tx_byte0_en_tx_de_dqs:[20:20]=0b1
// const DDR_PHY_REG_66_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_byte0_sel_dly1t_dq:[8:0]=0b000000000
// 	// f1_param_phya_reg_tx_byte0_sel_dly1t_dqs:[12:12]=0b0
// 	// f1_param_phya_reg_tx_byte0_sel_dly1t_mask_ranka:[16:16]=0b0
// const DDR_PHY_REG_67_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_byte0_vref_sel_lpddr4divby2p5:[0:0]=0b0
// 	// f1_param_phya_reg_tx_byte0_vref_sel_lpddr4divby3:[4:4]=0b0
// 	// f1_param_phya_reg_tx_byte0_vref_sel_lpddr4x_voh0p5:[8:8]=0b0
// 	// f1_param_phya_reg_tx_byte0_vref_sel_lpddr4x_voh0p6:[12:12]=0b0
// const DDR_PHY_REG_68_F1_DATA: u32 = 0b00000000000000000000000000000100;
// 	// f1_param_phyd_reg_rx_byte0_resetz_dqs_offset:[3:0]=0b0100
// const DDR_PHY_REG_69_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte0_dq0_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq1_offset:[14:8]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq2_offset:[22:16]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq3_offset:[30:24]=0b0000000
// const DDR_PHY_REG_70_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte0_dq4_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq5_offset:[14:8]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq6_offset:[22:16]=0b0000000
// 	// f1_param_phyd_reg_byte0_dq7_offset:[30:24]=0b0000000
// const DDR_PHY_REG_71_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte0_dm_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte0_dqsn_offset:[19:16]=0b0000
// 	// f1_param_phyd_reg_byte0_dqsp_offset:[27:24]=0b0000
// const DDR_PHY_REG_72_F1_DATA: u32 = 0b00000000000000000000000000000011;
// 	// f1_param_phyd_tx_byte0_tx_oenz_extend:[2:0]=0b011
// const DDR_PHY_REG_80_F1_DATA: u32 = 0b00000000000000000001000000000001;
// 	// f1_param_phya_reg_rx_byte1_en_lsmode:[0:0]=0b1
// 	// f1_param_phya_reg_rx_byte1_hystr:[5:4]=0b00
// 	// f1_param_phya_reg_rx_byte1_sel_dqs_rec_vref_mode:[8:8]=0b0
// 	// f1_param_phya_reg_rx_byte1_sel_odt_center_tap:[10:10]=0b0
// 	// f1_param_phya_reg_byte1_en_rec_vol_mode:[12:12]=0b1
// 	// f1_param_phya_reg_tx_byte1_force_en_lvstl_ph:[14:14]=0b0
// 	// f1_param_phya_reg_rx_byte1_force_en_lvstl_odt:[16:16]=0b0
// 	// f1_param_phya_reg_rx_byte1_en_trig_lvl_rangex2:[18:18]=0b0
// 	// f1_param_phya_reg_rx_byte1_trig_lvl_en_free_offset:[20:20]=0b0
// const DDR_PHY_REG_81_F1_DATA: u32 = 0b00000000000100000000000000000000;
// 	// f1_param_phya_reg_tx_byte1_drvn_de_dq:[1:0]=0b00
// 	// f1_param_phya_reg_tx_byte1_drvp_de_dq:[5:4]=0b00
// 	// f1_param_phya_reg_tx_byte1_drvn_de_dqs:[9:8]=0b00
// 	// f1_param_phya_reg_tx_byte1_drvp_de_dqs:[13:12]=0b00
// 	// f1_param_phya_reg_tx_byte1_en_tx_de_dq:[16:16]=0b0
// 	// f1_param_phya_reg_tx_byte1_en_tx_de_dqs:[20:20]=0b1
// const DDR_PHY_REG_82_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_byte1_sel_dly1t_dq:[8:0]=0b000000000
// 	// f1_param_phya_reg_tx_byte1_sel_dly1t_dqs:[12:12]=0b0
// 	// f1_param_phya_reg_tx_byte1_sel_dly1t_mask_ranka:[16:16]=0b0
// const DDR_PHY_REG_83_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_tx_byte1_vref_sel_lpddr4divby2p5:[0:0]=0b0
// 	// f1_param_phya_reg_tx_byte1_vref_sel_lpddr4divby3:[4:4]=0b0
// 	// f1_param_phya_reg_tx_byte1_vref_sel_lpddr4x_voh0p5:[8:8]=0b0
// 	// f1_param_phya_reg_tx_byte1_vref_sel_lpddr4x_voh0p6:[12:12]=0b0
// const DDR_PHY_REG_84_F1_DATA: u32 = 0b00000000000000000000000000000100;
// 	// f1_param_phyd_reg_rx_byte1_resetz_dqs_offset:[3:0]=0b0100
// const DDR_PHY_REG_85_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte1_dq0_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq1_offset:[14:8]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq2_offset:[22:16]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq3_offset:[30:24]=0b0000000
// const DDR_PHY_REG_86_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte1_dq4_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq5_offset:[14:8]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq6_offset:[22:16]=0b0000000
// 	// f1_param_phyd_reg_byte1_dq7_offset:[30:24]=0b0000000
// const DDR_PHY_REG_87_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_byte1_dm_offset:[6:0]=0b0000000
// 	// f1_param_phyd_reg_byte1_dqsn_offset:[19:16]=0b0000
// 	// f1_param_phyd_reg_byte1_dqsp_offset:[27:24]=0b0000
// const DDR_PHY_REG_88_F1_DATA: u32 = 0b00000000000000000000000000000011;
// 	// f1_param_phyd_tx_byte1_tx_oenz_extend:[2:0]=0b011
// const DDR_PHY_REG_320_F1_DATA: u32 = 0b00000000000000000000010000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_ca0_sw:[6:0]=0b0000000
// 	// f1_param_phyd_tx_ca0_shift_sel:[13:8]=0b000100
// const DDR_PHY_REG_331_F1_DATA: u32 = 0b00000000000000000000010000000000;
// 	// f1_param_phyd_tx_ca22_shift_sel:[13:8]=0b000100
// const DDR_PHY_REG_332_F1_DATA: u32 = 0b00000000000000000000010000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_cke0_sw:[6:0]=0b0000000
// 	// f1_param_phyd_tx_cke0_shift_sel:[13:8]=0b000100
// const DDR_PHY_REG_333_F1_DATA: u32 = 0b00000000000000000000010000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_csb0_sw:[6:0]=0b0000000
// 	// f1_param_phyd_tx_cs0_shift_sel:[13:8]=0b000100
// const DDR_PHY_REG_334_F1_DATA: u32 = 0b00000000000000000000010000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_resetz_sw:[6:0]=0b0000000
// 	// f1_param_phyd_tx_reset_shift_sel:[13:8]=0b000100
// const DDR_PHY_REG_336_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_ca0_raw:[6:0]=0b0000000
// const DDR_PHY_REG_348_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_cke0_raw:[6:0]=0b0000000
// const DDR_PHY_REG_349_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_csb0_raw:[6:0]=0b0000000
// const DDR_PHY_REG_350_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_ca_tx_dline_code_resetz_raw:[6:0]=0b0000000
// const DDR_PHY_REG_351_F1_DATA: u32 = 0b00001000000010000000010000000100;
// 	// f1_param_phya_reg_tx_ca_drvn_ca:[4:0]=0b00100
// 	// f1_param_phya_reg_tx_ca_drvp_ca:[12:8]=0b00100
// 	// f1_param_phya_reg_tx_ca_drvn_csb:[20:16]=0b01000
// 	// f1_param_phya_reg_tx_ca_drvp_csb:[28:24]=0b01000
// const DDR_PHY_REG_352_F1_DATA: u32 = 0b00001000000010000000100000001000;
// 	// f1_param_phya_reg_tx_clk_drvn_clkn0:[4:0]=0b01000
// 	// f1_param_phya_reg_tx_clk_drvp_clkn0:[12:8]=0b01000
// 	// f1_param_phya_reg_tx_clk_drvn_clkp0:[20:16]=0b01000
// 	// f1_param_phya_reg_tx_clk_drvp_clkp0:[28:24]=0b01000
// const DDR_PHY_REG_384_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq0_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit0_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq1_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit1_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_385_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq2_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit2_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq3_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit3_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_386_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq4_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit4_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq5_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit5_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_387_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq6_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit6_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq7_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit7_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_388_F1_DATA: u32 = 0b00000000000000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq8_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte0_bit8_data_shift:[13:8]=0b000110
// const DDR_PHY_REG_389_F1_DATA: u32 = 0b00001011000000000000000000000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dqsn_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dqsp_sw:[22:16]=0b0000000
// 	// f1_param_phyd_tx_byte0_dqs_shift:[29:24]=0b001011
// const DDR_PHY_REG_390_F1_DATA: u32 = 0b00001010000000000000100100000000;
// 	// f1_param_phyd_tx_byte0_oenz_dqs_shift:[13:8]=0b001001
// 	// f1_param_phyd_tx_byte0_oenz_shift:[29:24]=0b001010
// const DDR_PHY_REG_391_F1_DATA: u32 = 0b00000000000001000000011000000000;
// 	// f1_param_phyd_tx_byte0_oenz_dqs_extend:[11:8]=0b0110
// 	// f1_param_phyd_tx_byte0_oenz_extend:[19:16]=0b0100
// const DDR_PHY_REG_392_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq0_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq1_raw:[22:16]=0b1000000
// const DDR_PHY_REG_393_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq2_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq3_raw:[22:16]=0b1000000
// const DDR_PHY_REG_394_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq4_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq5_raw:[22:16]=0b1000000
// const DDR_PHY_REG_395_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq6_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq7_raw:[22:16]=0b1000000
// const DDR_PHY_REG_396_F1_DATA: u32 = 0b00000000000000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dq8_raw:[6:0]=0b1000000
// const DDR_PHY_REG_397_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dqsn_raw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_dqsp_raw:[22:16]=0b0000000
// const DDR_PHY_REG_398_F1_DATA: u32 = 0b00000000000000000000100000001000;
// 	// f1_param_phya_reg_tx_byte0_drvn_dq:[4:0]=0b01000
// 	// f1_param_phya_reg_tx_byte0_drvp_dq:[12:8]=0b01000
// const DDR_PHY_REG_399_F1_DATA: u32 = 0b00001000000010000000100000001000;
// 	// f1_param_phya_reg_tx_byte0_drvn_dqsn:[4:0]=0b01000
// 	// f1_param_phya_reg_tx_byte0_drvp_dqsn:[12:8]=0b01000
// 	// f1_param_phya_reg_tx_byte0_drvn_dqsp:[20:16]=0b01000
// 	// f1_param_phya_reg_tx_byte0_drvp_dqsp:[28:24]=0b01000
// const DDR_PHY_REG_400_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq0_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit0_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq1_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit1_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_401_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq2_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit2_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq3_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit3_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_402_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq4_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit4_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq5_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit5_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_403_F1_DATA: u32 = 0b00000110010000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq6_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit6_data_shift:[13:8]=0b000110
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq7_sw:[22:16]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit7_data_shift:[29:24]=0b000110
// const DDR_PHY_REG_404_F1_DATA: u32 = 0b00000000000000000000011001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq8_sw:[6:0]=0b1000000
// 	// f1_param_phyd_tx_byte1_bit8_data_shift:[13:8]=0b000110
// const DDR_PHY_REG_405_F1_DATA: u32 = 0b00001011000000000000000000000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dqsn_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dqsp_sw:[22:16]=0b0000000
// 	// f1_param_phyd_tx_byte1_dqs_shift:[29:24]=0b001011
// const DDR_PHY_REG_406_F1_DATA: u32 = 0b00001010000000000000100100000000;
// 	// f1_param_phyd_tx_byte1_oenz_dqs_shift:[13:8]=0b001001
// 	// f1_param_phyd_tx_byte1_oenz_shift:[29:24]=0b001010
// const DDR_PHY_REG_407_F1_DATA: u32 = 0b00000000000001000000011000000000;
// 	// f1_param_phyd_tx_byte1_oenz_dqs_extend:[11:8]=0b0110
// 	// f1_param_phyd_tx_byte1_oenz_extend:[19:16]=0b0100
// const DDR_PHY_REG_408_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq0_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq1_raw:[22:16]=0b1000000
// const DDR_PHY_REG_409_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq2_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq3_raw:[22:16]=0b1000000
// const DDR_PHY_REG_410_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq4_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq5_raw:[22:16]=0b1000000
// const DDR_PHY_REG_411_F1_DATA: u32 = 0b00000000010000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq6_raw:[6:0]=0b1000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq7_raw:[22:16]=0b1000000
// const DDR_PHY_REG_412_F1_DATA: u32 = 0b00000000000000000000000001000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dq8_raw:[6:0]=0b1000000
// const DDR_PHY_REG_413_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dqsn_raw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_dqsp_raw:[22:16]=0b0000000
// const DDR_PHY_REG_414_F1_DATA: u32 = 0b00000000000000000000100000001000;
// 	// f1_param_phya_reg_tx_byte1_drvn_dq:[4:0]=0b01000
// 	// f1_param_phya_reg_tx_byte1_drvp_dq:[12:8]=0b01000
// const DDR_PHY_REG_415_F1_DATA: u32 = 0b00001000000010000000100000001000;
// 	// f1_param_phya_reg_tx_byte1_drvn_dqsn:[4:0]=0b01000
// 	// f1_param_phya_reg_tx_byte1_drvp_dqsn:[12:8]=0b01000
// 	// f1_param_phya_reg_tx_byte1_drvn_dqsp:[20:16]=0b01000
// 	// f1_param_phya_reg_tx_byte1_drvp_dqsp:[28:24]=0b01000
// const DDR_PHY_REG_448_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_rx_byte0_rx_dq0_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq1_deskew_sw:[14:8]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq2_deskew_sw:[22:16]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq3_deskew_sw:[30:24]=0b0000000
// const DDR_PHY_REG_449_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_rx_byte0_rx_dq4_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq5_deskew_sw:[14:8]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq6_deskew_sw:[22:16]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dq7_deskew_sw:[30:24]=0b0000000
// const DDR_PHY_REG_450_F1_DATA: u32 = 0b00000000010000000100000000000000;
// 	// f1_param_phyd_reg_rx_byte0_rx_dq8_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dqs_dlie_code_neg_ranka_sw:[15:8]=0b01000000
// 	// f1_param_phyd_reg_rx_byte0_rx_dqs_dlie_code_pos_ranka_sw:[23:16]=0b01000000
// const DDR_PHY_REG_451_F1_DATA: u32 = 0b00000000000000000000101101000000;
// 	// f1_param_phyd_reg_tx_byte0_tx_dline_code_mask_ranka_sw:[6:0]=0b1000000
// 	// f1_param_phyd_rx_byte0_mask_shift:[13:8]=0b001011
// const DDR_PHY_REG_452_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_rx_byte0_en_shift:[13:8]=0b000000
// 	// f1_param_phyd_rx_byte0_odt_en_shift:[29:24]=0b000000
// const DDR_PHY_REG_453_F1_DATA: u32 = 0b00000000000010000000111000001110;
// 	// f1_param_phyd_rx_byte0_en_extend:[3:0]=0b1110
// 	// f1_param_phyd_rx_byte0_odt_en_extend:[11:8]=0b1110
// 	// f1_param_phyd_rx_byte0_rden_to_rdvld:[20:16]=0b01000
// const DDR_PHY_REG_454_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_rx_byte0_rx_dq0_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq1_deskew_raw:[12:8]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq2_deskew_raw:[20:16]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq3_deskew_raw:[28:24]=0b00000
// const DDR_PHY_REG_455_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_rx_byte0_rx_dq4_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq5_deskew_raw:[12:8]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq6_deskew_raw:[20:16]=0b00000
// 	// f1_param_phya_reg_rx_byte0_rx_dq7_deskew_raw:[28:24]=0b00000
// const DDR_PHY_REG_456_F1_DATA: u32 = 0b01000000010000000000000000000000;
// 	// f1_param_phya_reg_rx_byte0_rx_dq8_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_tx_byte0_tx_dline_code_mask_ranka_raw:[14:8]=0b0000000
// 	// f1_param_phya_reg_rx_byte0_rx_dqs_dlie_code_neg_ranka_raw:[22:16]=0b1000000
// 	// f1_param_phya_reg_rx_byte0_rx_dqs_dlie_code_pos_ranka_raw:[30:24]=0b1000000
// const DDR_PHY_REG_457_F1_DATA: u32 = 0b00000000000100000000000000010000;
// 	// f1_param_phya_reg_rx_byte0_trig_lvl_dq:[4:0]=0b10000
// 	// f1_param_phya_reg_rx_byte0_trig_lvl_dq_offset:[14:8]=0b0000000
// 	// f1_param_phya_reg_rx_byte0_trig_lvl_dqs:[20:16]=0b10000
// 	// f1_param_phya_reg_rx_byte0_trig_lvl_dqs_offset:[30:24]=0b0000000
// const DDR_PHY_REG_460_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_rx_byte1_rx_dq0_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq1_deskew_sw:[14:8]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq2_deskew_sw:[22:16]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq3_deskew_sw:[30:24]=0b0000000
// const DDR_PHY_REG_461_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_reg_rx_byte1_rx_dq4_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq5_deskew_sw:[14:8]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq6_deskew_sw:[22:16]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dq7_deskew_sw:[30:24]=0b0000000
// const DDR_PHY_REG_462_F1_DATA: u32 = 0b00000000010000000100000000000000;
// 	// f1_param_phyd_reg_rx_byte1_rx_dq8_deskew_sw:[6:0]=0b0000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dqs_dlie_code_neg_ranka_sw:[15:8]=0b01000000
// 	// f1_param_phyd_reg_rx_byte1_rx_dqs_dlie_code_pos_ranka_sw:[23:16]=0b01000000
// const DDR_PHY_REG_463_F1_DATA: u32 = 0b00000000000000000000101101000000;
// 	// f1_param_phyd_reg_tx_byte1_tx_dline_code_mask_ranka_sw:[6:0]=0b1000000
// 	// f1_param_phyd_rx_byte1_mask_shift:[13:8]=0b001011
// const DDR_PHY_REG_464_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phyd_rx_byte1_en_shift:[13:8]=0b000000
// 	// f1_param_phyd_rx_byte1_odt_en_shift:[29:24]=0b000000
// const DDR_PHY_REG_465_F1_DATA: u32 = 0b00000000000010000000111000001110;
// 	// f1_param_phyd_rx_byte1_en_extend:[3:0]=0b1110
// 	// f1_param_phyd_rx_byte1_odt_en_extend:[11:8]=0b1110
// 	// f1_param_phyd_rx_byte1_rden_to_rdvld:[20:16]=0b01000
// const DDR_PHY_REG_466_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_rx_byte1_rx_dq0_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq1_deskew_raw:[12:8]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq2_deskew_raw:[20:16]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq3_deskew_raw:[28:24]=0b00000
// const DDR_PHY_REG_467_F1_DATA: u32 = 0b00000000000000000000000000000000;
// 	// f1_param_phya_reg_rx_byte1_rx_dq4_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq5_deskew_raw:[12:8]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq6_deskew_raw:[20:16]=0b00000
// 	// f1_param_phya_reg_rx_byte1_rx_dq7_deskew_raw:[28:24]=0b00000
// const DDR_PHY_REG_468_F1_DATA: u32 = 0b01000000010000000000000000000000;
// 	// f1_param_phya_reg_rx_byte1_rx_dq8_deskew_raw:[4:0]=0b00000
// 	// f1_param_phya_reg_tx_byte1_tx_dline_code_mask_ranka_raw:[14:8]=0b0000000
// 	// f1_param_phya_reg_rx_byte1_rx_dqs_dlie_code_neg_ranka_raw:[22:16]=0b1000000
// 	// f1_param_phya_reg_rx_byte1_rx_dqs_dlie_code_pos_ranka_raw:[30:24]=0b1000000
// const DDR_PHY_REG_469_F1_DATA: u32 = 0b00000000000100000000000000010000;
// 	// f1_param_phya_reg_rx_byte1_trig_lvl_dq:[4:0]=0b10000
// 	// f1_param_phya_reg_rx_byte1_trig_lvl_dq_offset:[14:8]=0b0000000
// 	// f1_param_phya_reg_rx_byte1_trig_lvl_dqs:[20:16]=0b10000
// 	// f1_param_phya_reg_rx_byte1_trig_lvl_dqs_offset:[30:24]=0b0000000
// #endif //F1_TEST

unsafe fn cvx16_pinmux(){
    mmio_write_32!(0x0000 + PHYD_BASE_ADDR, 0x12141013);
    mmio_write_32!(0x0004 + PHYD_BASE_ADDR, 0x0C041503);
    mmio_write_32!(0x0008 + PHYD_BASE_ADDR, 0x06050001);
    mmio_write_32!(0x000C + PHYD_BASE_ADDR, 0x08070B02);
    mmio_write_32!(0x0010 + PHYD_BASE_ADDR, 0x0A0F0E09);
    mmio_write_32!(0x0014 + PHYD_BASE_ADDR, 0x0016110D);
    mmio_write_32!(0x0018 + PHYD_BASE_ADDR, 0x00000000);
    mmio_write_32!(0x001C + PHYD_BASE_ADDR, 0x00000100);
    mmio_write_32!(0x0020 + PHYD_BASE_ADDR, 0x02136574);
    mmio_write_32!(0x0024 + PHYD_BASE_ADDR, 0x00000008);
    mmio_write_32!(0x0028 + PHYD_BASE_ADDR, 0x76512308);
    mmio_write_32!(0x002C + PHYD_BASE_ADDR, 0x00000004);

    let mut rddata;
    rddata = 0x00000100;
	mmio_write_32!(0x001C + PHYD_BASE_ADDR, rddata);
	rddata = 0x02136574;
	mmio_write_32!(0x0020 + PHYD_BASE_ADDR, rddata);
	rddata = 0x00000008;
	mmio_write_32!(0x0024 + PHYD_BASE_ADDR, rddata);
	rddata = 0x76512308;
	mmio_write_32!(0x0028 + PHYD_BASE_ADDR, rddata);
	rddata = 0x00000004;
	mmio_write_32!(0x002C + PHYD_BASE_ADDR, rddata);
	rddata = 0x12141013;
	mmio_write_32!(0x0000 + PHYD_BASE_ADDR, rddata);
	rddata = 0x0C041503;
	mmio_write_32!(0x0004 + PHYD_BASE_ADDR, rddata);
	rddata = 0x06050001;
	mmio_write_32!(0x0008 + PHYD_BASE_ADDR, rddata);
	rddata = 0x08070B02;
	mmio_write_32!(0x000C + PHYD_BASE_ADDR, rddata);
	rddata = 0x0A0F0E09;
	mmio_write_32!(0x0010 + PHYD_BASE_ADDR, rddata);
	rddata = 0x0016110D;
	mmio_write_32!(0x0014 + PHYD_BASE_ADDR, rddata);
	rddata = 0x00000000;
	mmio_write_32!(0x0018 + PHYD_BASE_ADDR, rddata);
}

const SSC_EN: bool = true;
const SSC_BYPASS: bool = true;
const DDR2: bool = false;
const DDR3: bool = true;
const DDR2_3: bool = false;

static mut freq_in: u64 = 0;
static mut tar_freq: u64 = 0;
static mut mod_freq: u64 = 0;
static mut dev_freq: u64 = 0;
static mut reg_set: u32 = 0;
static mut reg_span: u32 = 0;
static mut reg_step: u32 = 0;

unsafe fn phy_init() {


    freq_in = 752;
	mod_freq = 100;
	dev_freq = 15;
	// NOTICE("Data rate=%d.\n", ddr_data_rate);
// #ifdef SSC_EN
// 	tar_freq = (ddr_data_rate >> 4) * 0.985;
// #else
	tar_freq = (ddr_data_rate >> 4) as u64;
// #endif
	reg_set = ((freq_in as u64) * 67108864 / tar_freq) as u32;
	reg_span = ((tar_freq * 250) / (mod_freq as u64)) as u32;
	reg_step = (reg_set as u64 * dev_freq / (reg_span as u64 * 1000)) as u32;

	// io::print("ddr_data_rate = %d, freq_in = %d reg_set = %lx tar_freq = %x reg_span = %lx reg_step = %lx\n",
	// 	ddr_data_rate, freq_in, reg_set, tar_freq, reg_span, reg_step);


    let mut rddata;

    rddata = 0x00000000;
	mmio_write_32!(0x28 + CV_DDR_PHYD_APB, rddata);
	// ZQ_240 OPTION
	rddata = 0x00080001;
	mmio_write_32!(0x54 + CV_DDR_PHYD_APB, rddata);

    rddata = 0x01010808; // TOP_REG_TX_DDR3_GPO_IN =1
	mmio_write_32!(0x58 + CV_DDR_PHYD_APB, rddata);


    if SSC_EN {
        //==============================================================
        // Enable SSC
        //==============================================================
        rddata = reg_set; // TOP_REG_SSC_SET
        mmio_write_32!(0x54 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
        mmio_write_32!(0x58 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
        mmio_write_32!(0x5C + 0x03002900, rddata);
        // io::print("reg_step = %lx\n", reg_step);

        rddata = mmio_read_32!(0x50 + 0x03002900);
        rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
        rddata = modified_bits_by_value(rddata, 1, 1, 1); // TOP_REG_SSC_EN_SSC
        rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
        rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
        rddata = modified_bits_by_value(rddata, 1, 5, 5); // extpulse
        rddata = modified_bits_by_value(rddata, 0, 6, 6); // ssc_syn_fix_div
        mmio_write_32!(0x50 + 0x03002900, rddata);
        io::print("SSC_EN\n");
    }else{
        if SSC_BYPASS{
            rddata = (reg_set & 0xfc000000) + 0x04000000; // TOP_REG_SSC_SET
            mmio_write_32!(0x54 + 0x03002900, rddata);
            rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
            mmio_write_32!(0x58 + 0x03002900, rddata);
            rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
            mmio_write_32!(0x5C + 0x03002900, rddata);
            rddata = mmio_read_32!(0x50 + 0x03002900);
            rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
            rddata = modified_bits_by_value(rddata, 0, 1, 1); // TOP_REG_SSC_EN_SSC
            rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
            rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
            rddata = modified_bits_by_value(rddata, 1, 5, 5); // TOP_REG_SSC_EXTPULSE
            rddata = modified_bits_by_value(rddata, 1, 6, 6); // ssc_syn_fix_div
            mmio_write_32!(0x50 + 0x03002900, rddata);
            io::print("SSC_BYPASS\n");
        }else{
            //==============================================================
            // SSC_EN =0
            //==============================================================
            io::print("SSC_EN =0\n");
            rddata = reg_set; // TOP_REG_SSC_SET
            mmio_write_32!(0x54 + 0x03002900, rddata);
            rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
            mmio_write_32!(0x58 + 0x03002900, rddata);
            rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
            mmio_write_32!(0x5C + 0x03002900, rddata);
            rddata = mmio_read_32!(0x50 + 0x03002900);
            rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
            rddata = modified_bits_by_value(rddata, 0, 1, 1); // TOP_REG_SSC_EN_SSC
            rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
            rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
            rddata = modified_bits_by_value(rddata, 1, 5, 5); // TOP_REG_SSC_EXTPULSE
            rddata = modified_bits_by_value(rddata, 0, 6, 6); // ssc_syn_fix_div
            mmio_write_32!(0x50 + 0x03002900, rddata);
            io::print("SSC_OFF\n");
        } // SSC_BYPASS
    } // SSC_EN




    // opdelay(1000);
	// DDRPLL setting
	rddata = mmio_read_32!(0x0C + CV_DDR_PHYD_APB);
	//[0]    = 1;      //TOP_REG_DDRPLL_EN_DLLCLK
	//[1]    = 1;      //TOP_REG_DDRPLL_EN_LCKDET
	//[2]    = 0;      //TOP_REG_DDRPLL_EN_TST
	//[5:3]  = 0b001; //TOP_REG_DDRPLL_ICTRL
	//[6]    = 0;      //TOP_REG_DDRPLL_MAS_DIV_SEL
	//[7]    = 0;      //TOP_REG_DDRPLL_MAS_RSTZ_DIV
	//[8]    = 1;      //TOP_REG_DDRPLL_SEL_4BIT
	//[10:9] = 0b01;  //TOP_REG_DDRPLL_SEL_MODE
	//[12:11]= 0b00;  //Rev
	//[13]   = 0;      //TOP_REG_DDRPLL_SEL_LOW_SPEED
	//[14]   = 0;      //TOP_REG_DDRPLL_MAS_DIV_OUT_SEL
	//[15]   = 0;      //TOP_REG_DDRPLL_PD
	rddata = modified_bits_by_value(rddata, 0x030b, 15, 0);
	mmio_write_32!(0x0C + CV_DDR_PHYD_APB, rddata);
	rddata = mmio_read_32!(0x10 + CV_DDR_PHYD_APB);
	//[7:0] = 0x0;   //TOP_REG_DDRPLL_TEST
	rddata = modified_bits_by_value(rddata, 0, 7, 0); // TOP_REG_DDRPLL_TEST
	mmio_write_32!(0x10 + CV_DDR_PHYD_APB, rddata);
	//[0]   = 1;    //TOP_REG_RESETZ_DIV
	rddata = 0x1;
	mmio_write_32!(0x04 + CV_DDR_PHYD_APB, rddata);
	io::print("RSTZ_DIV=1\n");
	rddata = mmio_read_32!(0x0C + CV_DDR_PHYD_APB);
	//[7]   = 1;    //TOP_REG_DDRPLL_MAS_RSTZ_DIV
	rddata = modified_bits_by_value(rddata, 1, 7, 7);
	mmio_write_32!(0x0C + CV_DDR_PHYD_APB, rddata);
	io::print("Wait for DRRPLL LOCK=1... pll init\n");



    loop {
		rddata = mmio_read_32!(0x10 + CV_DDR_PHYD_APB);
		if get_bits_from_value(rddata, 15, 15) != 0 {
			break;
		}
	}
}

#[derive(Clone, Copy)]
pub struct ddr_param {
	pub data: [u8; 1024 * 16]
}

#[repr(align(512))]
union sram_union {
	ddr_param: ddr_param,
	loader_2nd_header: crate::platform::loader_2nd_header,
	buf: [u8; BLOCK_SIZE as usize],
}
pub const fip_param1: *mut fip_param1 = PARAM1_BASE as *mut fip_param1;
pub static mut sram_union_buf: sram_union = unsafe { MaybeUninit::zeroed().assume_init() };
pub static mut fip_param2: fip_param2  = unsafe { MaybeUninit::zeroed().assume_init() };

struct UART0;

impl core::fmt::Write for UART0{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        io::print(s);
        Ok(())
    }
}

use core::fmt::Write;

unsafe fn load_ddr_param(retry: core::ffi::c_int) -> core::ffi::c_int {
	let crc: u32;
	let mut ret: core::ffi::c_int = -1;


	writeln!(UART0, "DPS/0x{:x}/0x{:x}.\n", fip_param2.ddr_param_loadaddr, fip_param2.ddr_param_size);

	if fip_param2.ddr_param_size as usize >= core::mem::size_of::<ddr_param>(){
		fip_param2.ddr_param_size = core::mem::size_of::<ddr_param>() as u32;
    }

	ret = crate::rom_api::p_rom_api_load_image(core::ptr::addr_of_mut!(sram_union_buf.ddr_param).cast(), fip_param2.ddr_param_loadaddr, fip_param2.ddr_param_size as usize,
				   retry);
	if ret < 0 {
		return ret;
	}

	crc = crate::rom_api::p_rom_api_image_crc(core::ptr::addr_of!(sram_union_buf.ddr_param).cast(), fip_param2.ddr_param_size as i32);
	if crc != fip_param2.ddr_param_cksum {
		// ERROR("ddr_param_cksum (0x%x/0x%x)\n", crc, fip_param2.ddr_param_cksum);
		return -1;
	}

	// NOTICE("DPE.\n");

	return 0;
}

unsafe fn load_param2(retry: core::ffi::c_int) -> core::ffi::c_int{
	let crc: u32;
	let mut ret: core::ffi::c_int = -1;

    use crate::mmap::*;

	writeln!(UART0, "P2S/0x{:x}/{:p}.\n", core::mem::size_of::<fip_param2>(), core::ptr::addr_of!(fip_param2));

	ret = crate::rom_api::p_rom_api_load_image(core::ptr::addr_of_mut!(fip_param2).cast(), (*fip_param1).param2_loadaddr, crate::platform::PARAM2_SIZE, retry);
	if ret < 0 {
		return ret;
	}

	if fip_param2.magic1 != FIP_PARAM2_MAGIC1 {
		// WARN("LP2_NOMAGIC\n");
		return -1;
	}

	crc = crate::rom_api::p_rom_api_image_crc(core::ptr::addr_of!(fip_param2.reserved1).cast(), (core::mem::size_of::<fip_param2>() - 12) as i32);
	if crc != fip_param2.param2_cksum {
		// ERROR("param2_cksum (0x%x/0x%x)\n", crc, fip_param2.param2_cksum);
		return -1;
	}

	io::print("P2E.\n");

	return 0;
}

pub unsafe fn load_ddr(){

    loop{
        
        let mut retry = 0;
        for r in 0..crate::rom_api::p_rom_api_get_number_of_retries() {
            retry = r;
            if load_param2(retry) < 0{
                continue;
            }
    
            if load_ddr_param(retry) < 0{
                continue;
            }
            
            break;
        }
    
        if retry >= crate::rom_api::p_rom_api_get_number_of_retries() {
            match crate::rom_api::p_rom_api_get_boot_src(){
                crate::platform::boot_src::BOOT_SRC_UART
                | crate::platform::boot_src::BOOT_SRC_SD
                | crate::platform::boot_src::BOOT_SRC_USB => {
                    crate::rom_api::p_rom_api_flash_init();   
                }
                _ => {
                    panic!("Failed to load DDR params");
                }
            }
        }
        break;
    }
  
}

pub unsafe fn init_ddr() {
    load_ddr();

	crate::io::print("phy init\n");
    phy_init();
	crate::io::print("ddrc init\n");

    let mut rddata;

    ddrc_init();
	crate::io::print("ddrc finished\n");

    mmio_write_32!(0x0800A000 + 0x20, 0x0);

	// set axi QOS
	// M1 = 0xA (VIP realtime)
	// M2 = 0x8 (VIP offline)
	// M3 = 0x7 (CPU)
	// M4 = 0x0 (TPU)
	// M5 = 0x9 (Video codec)
	// M6 = 0x2 (high speed peri)
	mmio_write_32!(0x030001D8, 0x007788aa);
	mmio_write_32!(0x030001DC, 0x00002299);

    // cvx16_setting_check();
	// crate::io::print("cvx16_setting_check  finish\n");

	// pinmux
	cvx16_pinmux();
	crate::io::print("cvx16_pinmux finish\n");

	ddr_patch_set();

	cvx16_en_rec_vol_mode();
	crate::io::print("cvx16_en_rec_vol_mode finish\n");

	// set_dfi_init_start
	cvx16_set_dfi_init_start();
	crate::io::print("set_dfi_init_start finish\n");

	// ddr_phy_power_on_seq1
	cvx16_ddr_phy_power_on_seq1();
	crate::io::print("ddr_phy_power_on_seq1 finish\n");

	// first dfi_init_start
	crate::io::print("first dfi_init_start\n");
	cvx16_polling_dfi_init_start();
	crate::io::print("cvx16_polling_dfi_init_start finish\n");

	cvx16_INT_ISR_08();
	crate::io::print("cvx16_INT_ISR_08 finish\n");

	// ddr_phy_power_on_seq3
	cvx16_ddr_phy_power_on_seq3();
	crate::io::print("ddr_phy_power_on_seq3 finish\n");

	// wait_for_dfi_init_complete
	cvx16_wait_for_dfi_init_complete();
	crate::io::print("wait_for_dfi_init_complete finish\n");

	// polling_synp_normal_mode
	cvx16_polling_synp_normal_mode();
	crate::io::print("polling_synp_normal_mode finish\n");


    ctrl_init_low_patch();
	crate::io::print("ctrl_low_patch finish\n");


	// cvx16_rdglvl_req
	cvx16_rdglvl_req();
	crate::io::print("cvx16_rdglvl_req finish\n");







    	// cvx16_wdqlvl_req
        crate::io::print("wdqlvl_M1_ALL_DQ_DM\n");
	// sso_8x1_c(5, 15, 0, 1, &sram_sp);//mode = write, input int fmin = 5, input int fmax = 15,
					    //input int sram_st = 0, output int sram_sp

	// data_mode = 'h0 : phyd pattern
	// data_mode = 'h1 : bist read/write
	// data_mode = 'h11: with Error enject,  multi- bist write/read
	// data_mode = 'h12: with Error enject,  multi- bist write/read
	// lvl_mode  = 'h0 : wdmlvl
	// lvl_mode  = 'h1 : wdqlvl
	// lvl_mode  = 'h2 : wdqlvl and wdmlvl
	// cvx16_wdqlvl_req(data_mode, lvl_mode);
	cvx16_wdqlvl_req(1, 2);
	crate::io::print("cvx16_wdqlvl_req dq/dm finish\n");

	cvx16_wdqlvl_req(1, 1);
	crate::io::print("cvx16_wdqlvl_req dq finish\n");

	cvx16_wdqlvl_req(1, 0);
	crate::io::print("cvx16_wdqlvl_req dm finish\n");









    	// cvx16_rdlvl_req
	// mode = 'h0  : MPR mode, DDR3 only.
	// mode = 'h1  : sram write/read continuous goto
	// mode = 'h2  : multi- bist write/read
	// mode = 'h10 : with Error enject,  multi- bist write/read
	// mode = 'h12 : with Error enject,  multi- bist write/read
	rddata = mmio_read_32!(0x008c + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 7, 4); // param_phyd_pirdlvl_capture_cnt
	mmio_write_32!(0x008c + PHYD_BASE_ADDR, rddata);

	crate::io::print("mode multi- bist write/read\n");
	// cvx16_rdlvl_req(2); // mode multi- PRBS bist write/read
	cvx16_rdlvl_req(1); // mode multi- SRAM bist write/read
	crate::io::print("cvx16_rdlvl_req finish\n");








    	// ctrl_high_patch
	ctrl_init_high_patch();

	let dram_cap_in_mbyte = ctrl_init_detect_dram_size();
	crate::io::print("ctrl_init_detect_dram_size finish\n");

	ctrl_init_update_by_dram_size(dram_cap_in_mbyte);
	crate::io::print("ctrl_init_update_by_dram_size finish\n");

	// crate::io::print("dram_cap_in_mbyte = %x\n", dram_cap_in_mbyte);
	cvx16_dram_cap_check(dram_cap_in_mbyte);
	crate::io::print("cvx16_dram_cap_check finish\n");

	// clk_gating_enable
	cvx16_clk_gating_enable();
	crate::io::print("cvx16_clk_gating_enable finish\n");





	//NOTICE("AXI mon setting for latency histogram.\n");
	axi_mon_latency_setting(0x5);

	//NOTICE("AXI mon 0 register dump before start.\n");
	//dump_axi_mon_reg(AXIMON_M1_WRITE);
	//NOTICE("AXI mon 1 register dump before start.\n");
	//dump_axi_mon_reg(AXIMON_M1_READ);

	axi_mon_start_all();
}

unsafe fn cvx16_set_dfi_init_start() {
    let mut rddata;
    	// synp setting
	// phy is ready for initial dfi_init_start request
	// set umctl2 to tigger dfi_init_start
	io::print("cvx16_set_dfi_init_start\n");
	// ddr_debug_wr32(0x0d);
	// ddr_debug_num_write();
	mmio_write_32!(cfg_base + 0x00000320, 0x00000000);
	rddata = mmio_read_32!(cfg_base + 0x000001b0); // dfi_init_start @ rddata[5];
	rddata = modified_bits_by_value(rddata, 1, 5, 5);
	mmio_write_32!(cfg_base + 0x000001b0, rddata);
	mmio_write_32!(cfg_base + 0x00000320, 1);
	io::print("dfi_init_start finish\n");
}

unsafe fn cvx16_ddr_phy_power_on_seq1() {
    let mut rddata;
	// power_seq_1
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x0e);
	// ddr_debug_num_write();
	// RESETZ/CKE PD=0
	rddata = mmio_read_32!(0x40 + CV_DDR_PHYD_APB);
	// TOP_REG_TX_CA_PD_CKE0
	rddata = modified_bits_by_value(rddata, 0, 24, 24);
	// TOP_REG_TX_CA_PD_RESETZ
	rddata = modified_bits_by_value(rddata, 0, 30, 30);
	mmio_write_32!(0x40 + CV_DDR_PHYD_APB, rddata);
	io::print("RESET PD !!!\n");

	// CA PD=0
	// All PHYA CA PD=0
	rddata = mmio_read_32!(0x40 + CV_DDR_PHYD_APB);
	rddata = modified_bits_by_value(rddata, 0, 31, 0);
	mmio_write_32!(0x40 + CV_DDR_PHYD_APB, rddata);
	io::print("All PHYA CA PD=0 ...\n");

	// TOP_REG_TX_SEL_GPIO = 1 (DQ)
	rddata = mmio_read_32!(0x1c + CV_DDR_PHYD_APB);
	rddata = modified_bits_by_value(rddata, 1, 7, 7);
	mmio_write_32!(0x1c + CV_DDR_PHYD_APB, rddata);
	io::print("TOP_REG_TX_SEL_GPIO = 1\n");

	// DQ PD=0
	// TOP_REG_TX_BYTE0_PD
	// TOP_REG_TX_BYTE1_PD
	rddata = 0x00000000;
	mmio_write_32!(0x00 + CV_DDR_PHYD_APB, rddata);
	io::print("TX_BYTE PD=0 ...\n");

	// TOP_REG_TX_SEL_GPIO = 0 (DQ)
	rddata = mmio_read_32!(0x1c + CV_DDR_PHYD_APB);
	rddata = modified_bits_by_value(rddata, 0, 7, 7);
	mmio_write_32!(0x1c + CV_DDR_PHYD_APB, rddata);
	io::print("TOP_REG_TX_SEL_GPIO = 0\n");
}

unsafe fn cvx16_polling_dfi_init_start() {
    // io::print("%s\n", __func__);
	// ddr_debug_wr32(0x11);
	// ddr_debug_num_write();
	loop {
		let rddata = mmio_read_32!(0x3028 + PHYD_BASE_ADDR);
		if get_bits_from_value(rddata, 8, 8) == 1 {
			break;
		}
	}
}

unsafe fn cvx16_INT_ISR_08() {
    let EN_PLL_SPEED_CHG: u32;
	let CUR_PLL_SPEED: u32;
	let NEXT_PLL_SPEED: u32;

	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x1c);
	// ddr_debug_num_write();
	// param_phyd_clkctrl_init_complete   <= int_regin[0];
	let mut rddata = 0x00000000;
	mmio_write_32!(0x0118 + PHYD_BASE_ADDR, rddata);
	//----------------------------------------------------
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	EN_PLL_SPEED_CHG = get_bits_from_value(rddata, 0, 0);
	CUR_PLL_SPEED = get_bits_from_value(rddata, 5, 4);
	NEXT_PLL_SPEED = get_bits_from_value(rddata, 9, 8);
	// io::print("CUR_PLL_SPEED = %x, NEXT_PLL_SPEED = %x, EN_PLL_SPEED_CHG=%x\n", CUR_PLL_SPEED, NEXT_PLL_SPEED,
	//        EN_PLL_SPEED_CHG);

	//----------------------------------------------------
	cvx16_ddr_phy_power_on_seq2();
	cvx16_set_dfi_init_complete();
}

unsafe fn cvx16_ddr_phy_power_on_seq2() {
    let mut rddata;
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x0f);
	// ddr_debug_num_write();
	// Change PLL frequency
	io::print("Change PLL frequency if necessary ...\n");

	cvx16_chg_pll_freq();
	// OEN
	// param_phyd_sel_cke_oenz        <= `PI_SD int_regin[0];
	rddata = mmio_read_32!(0x0154 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 0, 0, 0);
	mmio_write_32!(0x0154 + PHYD_BASE_ADDR, rddata);
	// param_phyd_tx_ca_oenz          <= `PI_SD int_regin[0];
	// param_phyd_tx_ca_clk0_oenz     <= `PI_SD int_regin[8];
	// param_phyd_tx_ca_clk1_oenz     <= `PI_SD int_regin[16];
	rddata = 0x00000000;
	mmio_write_32!(0x0130 + PHYD_BASE_ADDR, rddata);
	// Do DLLCAL if necessary
	io::print("Do DLLCAL if necessary ...\n");

	cvx16_dll_cal();
	io::print("Do DLLCAL done\n");

	//    io::print("Do ZQCAL if necessary ...\n");

	// cvx16_ddr_zqcal_hw_isr8(0x7);//zqcal hw mode, bit0: offset_cal, bit1:pl_en, bit2:step2_en
	// io::print("Do ZQCAL done\n");

	io::print("cv181x without ZQ Calibration ...\n");

	// cvx16_ddr_zq240_cal();//zq240_cal
	// io::print("Do cvx16_ddr_zq240_cal done\n");

	io::print("cv181x without ZQ240 Calibration ...\n");

	// zq calculate variation
	//  zq_cal_var();
	io::print("zq calculate variation not run\n");

	// CA PD =0
	// All PHYA CA PD=0
	rddata = 0x80000000;
	mmio_write_32!(0x40 + CV_DDR_PHYD_APB, rddata);
	io::print("All PHYA CA PD=0 ...\n");

	// BYTE PD =0
	rddata = 0x00000000;
	mmio_write_32!(0x00 + CV_DDR_PHYD_APB, rddata);
	io::print("TX_BYTE PD=0 ...\n");

	// power_on_2
}

unsafe fn cvx16_clk_div2() {
	io::print("div2 original frequency !!!\n\n");

	let mut rddata = mmio_read_32!(0x0c + CV_DDR_PHYD_APB);
	// rddata[14] = 1  ;  // TOP_REG_DDRPLL_MAS_DIV_OUT_SEL 1
	rddata = modified_bits_by_value(rddata, 1, 14, 14);
	mmio_write_32!(0x0c + CV_DDR_PHYD_APB, rddata);
	io::print("div2 original frequency\n");
}

unsafe fn cvx16_chg_pll_freq(){
    let mut rddata;
    let mut EN_PLL_SPEED_CHG: u32;
	let mut CUR_PLL_SPEED: u32;
	let mut NEXT_PLL_SPEED: u32;

	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x04);
	// ddr_debug_num_write();
	// Change PLL frequency
	// TOP_REG_RESETZ_DIV =0
	rddata = 0x00000000;
	mmio_write_32!(0x04 + CV_DDR_PHYD_APB, rddata);
	// TOP_REG_RESETZ_DQS =0
	mmio_write_32!(0x08 + CV_DDR_PHYD_APB, rddata);
	// TOP_REG_DDRPLL_MAS_RSTZ_DIV  =0
	rddata = mmio_read_32!(0x0C + CV_DDR_PHYD_APB);
	rddata = modified_bits_by_value(rddata, 0, 7, 7);
	mmio_write_32!(0x0C + CV_DDR_PHYD_APB, rddata);
	io::print("RSTZ_DIV=0\n");
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	EN_PLL_SPEED_CHG = get_bits_from_value(rddata, 0, 0);
	CUR_PLL_SPEED = get_bits_from_value(rddata, 5, 4);
	NEXT_PLL_SPEED = get_bits_from_value(rddata, 9, 8);
	// io::print("CUR_PLL_SPEED = %x, NEXT_PLL_SPEED = %x, EN_PLL_SPEED_CHG=%x\n", CUR_PLL_SPEED, NEXT_PLL_SPEED,
	//        EN_PLL_SPEED_CHG);

	if (EN_PLL_SPEED_CHG != 0) {
		if (NEXT_PLL_SPEED == 0) { // next clk_div40
			rddata = modified_bits_by_value(rddata, NEXT_PLL_SPEED, 5, 4);
			rddata = modified_bits_by_value(rddata, CUR_PLL_SPEED, 9, 8);
			mmio_write_32!(0x4c + CV_DDR_PHYD_APB, rddata);
			cvx16_clk_div40();
			io::print("clk_div40\n");
			io::print("clk_div40\n");
		} else {
			if (NEXT_PLL_SPEED == 0x2) { // next clk normal
				rddata = modified_bits_by_value(rddata, NEXT_PLL_SPEED, 5, 4);
				rddata = modified_bits_by_value(rddata, CUR_PLL_SPEED, 9, 8);
				mmio_write_32!(0x4c + CV_DDR_PHYD_APB, rddata);
				cvx16_clk_normal();
				io::print("clk_normal\n");
				io::print("clk_normal\n");
			} else {
				if (NEXT_PLL_SPEED == 0x1) { // next clk normal div_2
					rddata = modified_bits_by_value(rddata, NEXT_PLL_SPEED, 5, 4);
					rddata = modified_bits_by_value(rddata, CUR_PLL_SPEED, 9, 8);
					mmio_write_32!(0x4c + CV_DDR_PHYD_APB, rddata);
					cvx16_clk_div2();
					io::print("clk_div2\n");
					io::print("clk_div2\n");
				}
			}
		}
		//         opdelay(100000);  //  1000ns
	}
	// TOP_REG_RESETZ_DIV  =1
	rddata = 0x00000001;
	mmio_write_32!(0x04 + CV_DDR_PHYD_APB, rddata);
	rddata = mmio_read_32!(0x0C + CV_DDR_PHYD_APB);
	// rddata[7]   = 1;    //TOP_REG_DDRPLL_MAS_RSTZ_DIV
	rddata = modified_bits_by_value(rddata, 1, 7, 7);
	mmio_write_32!(0x0C + CV_DDR_PHYD_APB, rddata);
	io::print("RSTZ_DIV=1\n");
	// rddata[0]   = 1;    //TOP_REG_RESETZ_DQS
	rddata = 0x00000001;
	mmio_write_32!(0x08 + CV_DDR_PHYD_APB, rddata);
	io::print("TOP_REG_RESETZ_DQS\n");
	io::print("Wait for DRRPLL_SLV_LOCK=1...\n");

    let REAL_LOCK = true;
 if REAL_LOCK{
	rddata = modified_bits_by_value(rddata, 0, 15, 15);
	while (get_bits_from_value(rddata, 15, 15) == 0) {
		rddata = mmio_read_32!(0x10 + CV_DDR_PHYD_APB);
		io::print("REAL_LOCK.\n");

		opdelay(200);
	}
}else{
	io::print("check PLL lock...  pll init\n");
}
	//} Change PLL frequency
}

unsafe fn cvx16_clk_div40() {
	io::print("Enter low D40 frequency !!!\n\n");

	let mut rddata = mmio_read_32!(0x0c + CV_DDR_PHYD_APB);
	// TOP_REG_DDRPLL_SEL_LOW_SPEED =1
	rddata = modified_bits_by_value(rddata, 1, 13, 13);
	mmio_write_32!(0x0c + CV_DDR_PHYD_APB, rddata);
	io::print("Enter low D40 frequency\n");
}

unsafe fn opdelay(times: usize){
    for _ in 0..times{
        core::arch::asm!("nop");
    }
}

unsafe fn cvx16_clk_normal() {
	let mut rddata;
	io::print("back to original frequency !!!\n\n");

	rddata = mmio_read_32!(0x0c + CV_DDR_PHYD_APB);
	// rddata[13] TOP_REG_DDRPLL_SEL_LOW_SPEED 0
	// rddata[14] TOP_REG_DDRPLL_MAS_DIV_OUT_SEL 0
	rddata = modified_bits_by_value(rddata, 0, 13, 13);
	rddata = modified_bits_by_value(rddata, 0, 14, 14);
	mmio_write_32!(0x0c + CV_DDR_PHYD_APB, rddata);

	
 	if SSC_EN{
	//==============================================================
	// Enable SSC
	//==============================================================
	rddata = reg_set; // TOP_REG_SSC_SET
	mmio_write_32!(0x54 + 0x03002900, rddata);
	rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
	mmio_write_32!(0x58 + 0x03002900, rddata);
	rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
	mmio_write_32!(0x5C + 0x03002900, rddata);
	rddata = mmio_read_32!(0x50 + 0x03002900);
	rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
	rddata = modified_bits_by_value(rddata, 1, 1, 1); // TOP_REG_SSC_EN_SSC
	rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
	rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
	rddata = modified_bits_by_value(rddata, 1, 5, 5); // extpulse
	rddata = modified_bits_by_value(rddata, 0, 6, 6); // ssc_syn_fix_div
	mmio_write_32!(0x50 + 0x03002900, rddata);
	io::print("SSC_EN\n");
	}else{
		if SSC_BYPASS{
			rddata = (reg_set & 0xfc000000) + 0x04000000; // TOP_REG_SSC_SET
			mmio_write_32!(0x54 + 0x03002900, rddata);
			rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
			mmio_write_32!(0x58 + 0x03002900, rddata);
			rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
			mmio_write_32!(0x5C + 0x03002900, rddata);
			rddata = mmio_read_32!(0x50 + 0x03002900);
			rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
			rddata = modified_bits_by_value(rddata, 0, 1, 1); // TOP_REG_SSC_EN_SSC
			rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
			rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
			rddata = modified_bits_by_value(rddata, 1, 5, 5); // TOP_REG_SSC_EXTPULSE
			rddata = modified_bits_by_value(rddata, 1, 6, 6); // ssc_syn_fix_div
			io::print("SSC_BYPASS\n");
		}else{
			//==============================================================
			// SSC_EN =0
			//==============================================================
			io::print("SSC_EN =0\n");
			rddata = reg_set; // TOP_REG_SSC_SET
			mmio_write_32!(0x54 + 0x03002900, rddata);
			rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
			mmio_write_32!(0x58 + 0x03002900, rddata);
			rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
			mmio_write_32!(0x5C + 0x03002900, rddata);
			rddata = mmio_read_32!(0x50 + 0x03002900);
			rddata = modified_bits_by_value(rddata, !get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
			rddata = modified_bits_by_value(rddata, 0, 1, 1); // TOP_REG_SSC_EN_SSC
			rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
			rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
			rddata = modified_bits_by_value(rddata, 1, 5, 5); // TOP_REG_SSC_EXTPULSE
			rddata = modified_bits_by_value(rddata, 0, 6, 6); // ssc_syn_fix_div
			mmio_write_32!(0x50 + 0x03002900, rddata);
			io::print("SSC_OFF\n");
		} // SSC_BYPASS
	} // SSC_EN
	io::print("back to original frequency\n");
}

unsafe fn cvx16_dll_cal(){
    let mut rddata;
    let mut EN_PLL_SPEED_CHG: u32;
	let mut CUR_PLL_SPEED: u32;
	let mut NEXT_PLL_SPEED: u32;

	io::print("Do DLLCAL ...\n");

	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x2b);
	// ddr_debug_num_write();
	// TOP_REG_EN_PLL_SPEED_CHG
	//     <= #RD (~pwstrb_mask[0] & TOP_REG_EN_PLL_SPEED_CHG) |  pwstrb_mask_pwdata[0];
	// TOP_REG_CUR_PLL_SPEED   [1:0]
	//     <= #RD (~pwstrb_mask[5:4] & TOP_REG_CUR_PLL_SPEED[1:0]) |  pwstrb_mask_pwdata[5:4];
	// TOP_REG_NEXT_PLL_SPEED  [1:0]
	//     <= #RD (~pwstrb_mask[9:8] & TOP_REG_NEXT_PLL_SPEED[1:0]) |  pwstrb_mask_pwdata[9:8];
	rddata = mmio_read_32!(0x4c + CV_DDR_PHYD_APB);
	EN_PLL_SPEED_CHG = get_bits_from_value(rddata, 0, 0);
	CUR_PLL_SPEED = get_bits_from_value(rddata, 5, 4);
	NEXT_PLL_SPEED = get_bits_from_value(rddata, 9, 8);
	// io::print("CUR_PLL_SPEED = %x, NEXT_PLL_SPEED = %x, EN_PLL_SPEED_CHG=%x\n", CUR_PLL_SPEED, NEXT_PLL_SPEED,
	//        EN_PLL_SPEED_CHG);

	if (CUR_PLL_SPEED != 0) { // only do calibration and update when high speed
		// param_phyd_dll_rx_start_cal <= int_regin[1];
		// param_phyd_dll_tx_start_cal <= int_regin[17];
		rddata = mmio_read_32!(0x0040 + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, 0, 1, 1);
		rddata = modified_bits_by_value(rddata, 0, 17, 17);
		mmio_write_32!(0x0040 + PHYD_BASE_ADDR, rddata);
		// param_phyd_dll_rx_start_cal <= int_regin[1];
		// param_phyd_dll_tx_start_cal <= int_regin[17];
		rddata = mmio_read_32!(0x0040 + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, 1, 1, 1);
		rddata = modified_bits_by_value(rddata, 1, 17, 17);
		mmio_write_32!(0x0040 + PHYD_BASE_ADDR, rddata);
		rddata = 0x00000000;
		while (get_bits_from_value(rddata, 16, 16) == 0) {
			rddata = mmio_read_32!(0x3014 + PHYD_BASE_ADDR);
		}
		io::print("DLL lock !\n");

		io::print("DLL lock\n");
		// opdelay(1000);
		io::print("Do DLLUPD\n");
		// cvx16_dll_cal_status();
	} else { // stop calibration and update when low speed
		// param_phyd_dll_rx_start_cal <= int_regin[1];
		// param_phyd_dll_tx_start_cal <= int_regin[17];
		rddata = mmio_read_32!(0x0040 + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, 0, 1, 1);
		rddata = modified_bits_by_value(rddata, 0, 17, 17);
		mmio_write_32!(0x0040 + PHYD_BASE_ADDR, rddata);
	}
	io::print("Do DLLCAL Finish\n");

	io::print("Do DLLCAL Finish\n");
}


unsafe fn cvx16_set_dfi_init_complete(){
    let mut rddata;
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x48);
	// ddr_debug_num_write();
// #ifdef REAL_LOCK
	opdelay(20000);
// #endif
	// rddata[8] = 1;
	rddata = 0x00000010;
	mmio_write_32!(0x0120 + PHYD_BASE_ADDR, rddata);
	io::print("set init_complete = 1 ...\n");

	// param_phyd_clkctrl_init_complete   <= int_regin[0];
	rddata = 0x00000001;
	mmio_write_32!(0x0118 + PHYD_BASE_ADDR, rddata);
}


unsafe fn cvx16_ddr_phy_power_on_seq3() {
    let mut rddata;
    	// power on 3
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x10);
	// ddr_debug_num_write();
	// RESETYZ/CKE OENZ
	// param_phyd_sel_cke_oenz        <= `PI_SD int_regin[0];
	rddata = mmio_read_32!(0x0154 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 0, 0, 0);
	mmio_write_32!(0x0154 + PHYD_BASE_ADDR, rddata);
	// param_phyd_tx_ca_oenz          <= `PI_SD int_regin[0];
	// param_phyd_tx_ca_clk0_oenz     <= `PI_SD int_regin[8];
	// param_phyd_tx_ca_clk1_oenz     <= `PI_SD int_regin[16];
	rddata = 0x00000000;
	mmio_write_32!(0x0130 + PHYD_BASE_ADDR, rddata);
	io::print("[KC Info] --> ca_oenz  ca_clk_oenz !!!\n");

	// clock gated for power save
	// param_phya_reg_tx_byte0_en_extend_oenz_gated_dline <= `PI_SD int_regin[0];
	// param_phya_reg_tx_byte1_en_extend_oenz_gated_dline <= `PI_SD int_regin[1];
	// param_phya_reg_tx_byte2_en_extend_oenz_gated_dline <= `PI_SD int_regin[2];
	// param_phya_reg_tx_byte3_en_extend_oenz_gated_dline <= `PI_SD int_regin[3];
	rddata = mmio_read_32!(0x0204 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 18, 18);
	mmio_write_32!(0x0204 + PHYD_BASE_ADDR, rddata);
	rddata = mmio_read_32!(0x0224 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 18, 18);
	mmio_write_32!(0x0224 + PHYD_BASE_ADDR, rddata);
	io::print("[KC Info] --> en clock gated for power save !!!\n");

	// power on 3
}

unsafe fn cvx16_wait_for_dfi_init_complete() {
    let mut rddata;
    // io::print("%s\n", __func__);
	// ddr_debug_wr32(0x13);
	// ddr_debug_num_write();
	// synp setting
	loop {
		rddata = mmio_read_32!(cfg_base + 0x000001bc);
		//} while ((rddata & 0x00000001) != 1);
		if get_bits_from_value(rddata, 0, 0) == 1 {
			break;
		}
	}
	mmio_write_32!(cfg_base + 0x00000320, 0x00000000);
	rddata = mmio_read_32!(cfg_base + 0x000001b0);
	rddata = modified_bits_by_value(rddata, 5, 5, 0);
	mmio_write_32!(cfg_base + 0x000001b0, rddata);
	mmio_write_32!(cfg_base + 0x00000320, 0x00000001);
	io::print("dfi_init_complete finish\n");
}

unsafe fn cvx16_polling_synp_normal_mode() {
    let mut rddata;
    // io::print("%s\n", __func__);
	// ddr_debug_wr32(0x14);
	// ddr_debug_num_write();
	// synp ctrl operating_mode
	loop {
		rddata = mmio_read_32!(cfg_base + 0x00000004);
		// io::print("operating_mode = %x\n", get_bits_from_value(rddata, 2, 0));

		if get_bits_from_value(rddata, 2, 0) == 1 {
			break;
		}
	}
}

unsafe fn cvx16_rdglvl_req() {
	let mut rddata;
	let mut selfref_sw;
	let mut en_dfi_dram_clk_disable;
	let mut powerdown_en;
	let mut selfref_en;
	let mut ddr3_mpr_mode;
	let mut port_num;
	// Note: training need ctrl_low_patch first
	//  Write 0 to PCTRL_n.port_en, without port 0
	//  port number = 0,1,2,3
	port_num = 0x4;
	for i in 1..port_num {
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x0);
	}
	// Poll PSTAT.rd_port_busy_n = 0
	// Poll PSTAT.wr_port_busy_n = 0
	loop {
		rddata = mmio_read_32!(cfg_base + 0x3fc);
		io::print("Poll PSTAT.rd_port_busy_n =0\n");

		if (rddata == 0) {
			break;
		}
	}
	// disable PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	selfref_sw = get_bits_from_value(rddata, 5, 5);
	en_dfi_dram_clk_disable = get_bits_from_value(rddata, 3, 3);
	powerdown_en = get_bits_from_value(rddata, 1, 1);
	selfref_en = get_bits_from_value(rddata, 0, 0);
	rddata = modified_bits_by_value(rddata, 0, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, 0, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   // this register must not be set to 1
	rddata = modified_bits_by_value(rddata, 0, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, 0, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	cvx16_clk_gating_disable();
	// RFSHCTL3.dis_auto_refresh =1
	// rddata = mmio_read_32!(cfg_base + 0x60);
	// rddata=modified_bits_by_value(rddata, 1, 0, 0); //RFSHCTL3.dis_auto_refresh
	// mmio_write_32!(cfg_base + 0x60, rddata);
	

 if DDR3{
	rddata = mmio_read_32!(0x0184 + PHYD_BASE_ADDR);
	ddr3_mpr_mode = get_bits_from_value(rddata, 4, 4);
	if (ddr3_mpr_mode != 0) {
		// RFSHCTL3.dis_auto_refresh =1
		rddata = mmio_read_32!(cfg_base + 0x60);
		rddata = modified_bits_by_value(rddata, 1, 0, 0); // RFSHCTL3.dis_auto_refresh
		mmio_write_32!(cfg_base + 0x60, rddata);
		// MR3
		rddata = mmio_read_32!(cfg_base + 0xe0);
		rddata = modified_bits_by_value(rddata, 1, 2, 2); // Dataflow from MPR
		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
	}
}
 if DDR2_3{

	// rddata = mmio_read_32!(0x0184 + PHYD_BASE_ADDR);
	// ddr3_mpr_mode = get_bits_from_value(rddata, 4, 4);
	// if (get_ddr_type() == DDR_TYPE_DDR3) {
	// 	if (ddr3_mpr_mode) {
	// 		// RFSHCTL3.dis_auto_refresh =1
	// 		rddata = mmio_read_32!(cfg_base + 0x60);
	// 		rddata = modified_bits_by_value(rddata, 1, 0, 0); // RFSHCTL3.dis_auto_refresh
	// 		mmio_write_32!(cfg_base + 0x60, rddata);
	// 		// MR3
	// 		rddata = mmio_read_32!(cfg_base + 0xe0);
	// 		rddata = modified_bits_by_value(rddata, 1, 2, 2); // Dataflow from MPR
	// 		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
	// 	}
	// }
	}
	// bist setting for dfi rdglvl
	cvx16_bist_rdglvl_init();
	rddata = mmio_read_32!(0x0184 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 0, 0); // param_phyd_dfi_rdglvl_req
	mmio_write_32!(0x0184 + PHYD_BASE_ADDR, rddata);
	io::print("wait retraining finish ...\n");

	loop {
		//[0] param_phyd_dfi_wrlvl_done
		//[1] param_phyd_dfi_rdglvl_done
		//[2] param_phyd_dfi_rdlvl_done
		//[3] param_phyd_dfi_wdqlvl_done
		rddata = mmio_read_32!(0x3444 + PHYD_BASE_ADDR);
		if (get_bits_from_value(rddata, 1, 1) == 0x1) {
			// bist clock disable
			mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00040000);
			break;
		}
	}
 if DDR3{
	if (ddr3_mpr_mode != 0) {
		// MR3
		rddata = mmio_read_32!(cfg_base + 0xe0);
		rddata = modified_bits_by_value(rddata, 0, 2, 2); // Normal operation
		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
		// RFSHCTL3.dis_auto_refresh =0
		rddata = mmio_read_32!(cfg_base + 0x60);
		rddata = modified_bits_by_value(rddata, 0, 0, 0); // RFSHCTL3.dis_auto_refresh
		mmio_write_32!(cfg_base + 0x60, rddata);
	}
}
 if DDR2_3{
	// if (get_ddr_type() == DDR_TYPE_DDR3) {
	// 	if (ddr3_mpr_mode) {
	// 		// MR3
	// 		rddata = mmio_read_32!(cfg_base + 0xe0);
	// 		rddata = modified_bits_by_value(rddata, 0, 2, 2); // Normal operation
	// 		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
	// 		// RFSHCTL3.dis_auto_refresh =0
	// 		rddata = mmio_read_32!(cfg_base + 0x60);
	// 		rddata = modified_bits_by_value(rddata, 0, 0, 0); // RFSHCTL3.dis_auto_refresh
	// 		mmio_write_32!(cfg_base + 0x60, rddata);
	// 	}
	// }
	}
	// RFSHCTL3.dis_auto_refresh =0
	// rddata = mmio_read_32!(cfg_base + 0x60);
	// rddata=modified_bits_by_value(rddata, 0, 0, 0); //RFSHCTL3.dis_auto_refresh
	// mmio_write_32!(cfg_base + 0x60, rddata);
	// restore PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	rddata = modified_bits_by_value(rddata, selfref_sw, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, en_dfi_dram_clk_disable, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   //this register must not be set to 1
	rddata = modified_bits_by_value(rddata, powerdown_en, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, selfref_en, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	// Write 1 to PCTRL_n.port_en
	for i in 1..port_num{
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x1);
	}
	// cvx16_rdglvl_status();
	cvx16_clk_gating_enable();
}

unsafe fn cvx16_wdqlvl_req(data_mode: u32, lvl_mode: u32) {
	let mut rddata;
	let mut selfref_sw;
	let mut en_dfi_dram_clk_disable;
	let mut powerdown_en;
	let mut selfref_en;
	// uint32_t bist_data_mode; //unused
	let mut port_num;
	// Note: training need ctrl_low_patch first
	//  Write 0 to PCTRL_n.port_en, without port 0
	//  port number = 0,1,2,3
	port_num = 0x4;
	for i in 1..port_num {
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x0);
	}
	// Poll PSTAT.rd_port_busy_n = 0
	// Poll PSTAT.wr_port_busy_n = 0
	loop {
		rddata = mmio_read_32!(cfg_base + 0x3fc);
		io::print("Poll PSTAT.rd_port_busy_n =0\n");

		if (rddata == 0) {
			break;
		}
	}
	// disable PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	selfref_sw = get_bits_from_value(rddata, 5, 5);
	en_dfi_dram_clk_disable = get_bits_from_value(rddata, 3, 3);
	powerdown_en = get_bits_from_value(rddata, 1, 1);
	selfref_en = get_bits_from_value(rddata, 0, 0);
	rddata = modified_bits_by_value(rddata, 0, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, 0, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   //this register must not be set to 1
	rddata = modified_bits_by_value(rddata, 0, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, 0, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	cvx16_clk_gating_disable();
	io::print("cvx16_wdqlvl_req\n");
	// ddr_debug_wr32(0x31);
	// ddr_debug_num_write();
	cvx16_dfi_ca_park_prbs(1);
	io::print("cvx16_wdqlvl_req\n");

	// param_phyd_piwdqlvl_dq_mode
	//     <= #RD (~pwstrb_mask[12] & param_phyd_piwdqlvl_dq_mode) |  pwstrb_mask_pwdata[12];
	// param_phyd_piwdqlvl_dm_mode
	//     <= #RD (~pwstrb_mask[13] & param_phyd_piwdqlvl_dm_mode) |  pwstrb_mask_pwdata[13];
	rddata = mmio_read_32!(0x00BC + PHYD_BASE_ADDR);
	// lvl_mode =0x0, wdmlvl
	// lvl_mode =0x1, wdqlvl
	// lvl_mode =0x2, wdqlvl and wdmlvl
	if (lvl_mode == 0x0) {
		rddata = modified_bits_by_value(rddata, 0, 12, 12); // param_phyd_piwdqlvl_dq_mode
		rddata = modified_bits_by_value(rddata, 1, 13, 13); // param_phyd_piwdqlvl_dm_mode
	} else if (lvl_mode == 0x1) {
		rddata = modified_bits_by_value(rddata, 1, 12, 12); // param_phyd_piwdqlvl_dq_mode
		rddata = modified_bits_by_value(rddata, 0, 13, 13); // param_phyd_piwdqlvl_dm_mode
	} else if (lvl_mode == 0x2) {
		rddata = modified_bits_by_value(rddata, 1, 12, 12); // param_phyd_piwdqlvl_dq_mode
		rddata = modified_bits_by_value(rddata, 1, 13, 13); // param_phyd_piwdqlvl_dm_mode
	}
	mmio_write_32!(0x00BC + PHYD_BASE_ADDR, rddata);
	if (lvl_mode == 0x0) {
		rddata = mmio_read_32!(cfg_base + 0xC);
		rddata = modified_bits_by_value(rddata, 1, 7, 7);
		mmio_write_32!(cfg_base + 0xC, rddata);
		//        cvx16_bist_wdmlvl_init(sram_sp);
		cvx16_bist_wdmlvl_init();
	} else {
		// bist setting for dfi rdglvl
		// data_mode = 0x0 : phyd pattern
		// data_mode = 0x1 : bist read/write
		// data_mode = 0x11: with Error enject,  multi- bist write/read
		// data_mode = 0x12: with Error enject,  multi- bist write/read
		//         cvx16_bist_wdqlvl_init(data_mode, sram_sp);
		cvx16_bist_wdqlvl_init(data_mode);
	}
	io::print("cvx16_wdqlvl_req\n");
	// ddr_debug_wr32(0x31);
	// ddr_debug_num_write();
	rddata = mmio_read_32!(0x018C + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 0, 0); // param_phyd_dfi_wdqlvl_req
	if (lvl_mode == 0x0) {
		rddata = modified_bits_by_value(rddata, 0, 10, 10); // param_phyd_dfi_wdqlvl_vref_train_en
	} else {
		rddata = modified_bits_by_value(rddata, 1, 10, 10); // param_phyd_dfi_wdqlvl_vref_train_en
	}
	if ((data_mode == 0x1) || (data_mode == 0x11) || (data_mode == 0x12)) {
		rddata = modified_bits_by_value(rddata, 1, 4, 4); // param_phyd_dfi_wdqlvl_bist_data_en
	} else {
		rddata = modified_bits_by_value(rddata, 0, 4, 4); // param_phyd_dfi_wdqlvl_bist_data_en
	}
	mmio_write_32!(0x018C + PHYD_BASE_ADDR, rddata);
	io::print("wait retraining finish ...\n");

	loop {
		//[0] param_phyd_dfi_wrlvl_done
		//[1] param_phyd_dfi_rdglvl_done
		//[2] param_phyd_dfi_rdlvl_done
		//[3] param_phyd_dfi_wdqlvl_done
		rddata = mmio_read_32!(0x3444 + PHYD_BASE_ADDR);
		if (get_bits_from_value(rddata, 3, 3) == 0x1) {
			break;
		}
	}
	rddata = mmio_read_32!(cfg_base + 0xC);
	rddata = modified_bits_by_value(rddata, 0, 7, 7);
	mmio_write_32!(cfg_base + 0xC, rddata);
	// bist clock disable
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00040000);
	cvx16_dfi_ca_park_prbs(0);
	// restore PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	rddata = modified_bits_by_value(rddata, selfref_sw, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, en_dfi_dram_clk_disable, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   //this register must not be set to 1
	rddata = modified_bits_by_value(rddata, powerdown_en, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, selfref_en, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	// Write 1 to PCTRL_n.port_en
	for i in 1..port_num {
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x1);
	}
	// cvx16_wdqlvl_status();
	cvx16_clk_gating_enable();
}


unsafe fn cvx16_clk_gating_disable() {
	// TOP_REG_CG_EN_PHYD_TOP      0
	// TOP_REG_CG_EN_CALVL         1
	// TOP_REG_CG_EN_WRLVL         2
	// N/A                         3
	// TOP_REG_CG_EN_WRDQ          4
	// TOP_REG_CG_EN_RDDQ          5
	// TOP_REG_CG_EN_PIGTLVL       6
	// TOP_REG_CG_EN_RGTRACK       7
	// TOP_REG_CG_EN_DQSOSC        8
	// TOP_REG_CG_EN_LB            9
	// TOP_REG_CG_EN_DLL_SLAVE     10 //0:a-on
	// TOP_REG_CG_EN_DLL_MST       11 //0:a-on
	// TOP_REG_CG_EN_ZQ            12
	// TOP_REG_CG_EN_PHY_PARAM     13 //0:a-on
	// 0b01001011110101
	let mut rddata = 0x000012F5;
	mmio_write_32!(0x44 + CV_DDR_PHYD_APB, rddata);
	rddata = 0x00000000;
	mmio_write_32!(0x00F4 + PHYD_BASE_ADDR, rddata); // PHYD_SHIFT_GATING_EN
	rddata = mmio_read_32!(cfg_base + 0x30); // phyd_stop_clk
	rddata = modified_bits_by_value(rddata, 0, 9, 9);
	mmio_write_32!(cfg_base + 0x30, rddata);
	rddata = mmio_read_32!(cfg_base + 0x148); // dfi read/write clock gatting
	rddata = modified_bits_by_value(rddata, 0, 23, 23);
	rddata = modified_bits_by_value(rddata, 0, 31, 31);
	mmio_write_32!(cfg_base + 0x148, rddata);
	io::print("clk_gating_disable\n");

	// disable clock gating
	// mmio_write_32!(0x0800_a000 + 0x14 , 0x00000fff);
	// KC_MSG("axi disable clock gating\n");
}

unsafe fn cvx16_rdlvl_req(mode: u32) {
    let mut rddata;
	let mut selfref_sw: u32;
	let mut en_dfi_dram_clk_disable: u32;
	let mut powerdown_en: u32;
	let mut selfref_en: u32;
// #ifdef DDR3
	let mut ddr3_mpr_mode: u32;
// #endif //DDR3
// #ifdef DDR2_3
// 	uint32_t ddr3_mpr_mode;
// #endif
	let mut port_num: u32;
	let mut vref_training_en: u32;
	// uint32_t code_neg; //unused
	// uint32_t code_pos; //unused
	// Note: training need ctrl_low_patch first
	// mode = 0x0  : MPR mode, DDR3 only.
	// mode = 0x1  : sram write/read continuous goto
	// mode = 0x2  : multi- bist write/read
	// mode = 0x10 : with Error enject,  multi- bist write/read
	// mode = 0x12 : with Error enject,  multi- bist write/read
	//  Write 0 to PCTRL_n.port_en, without port 0
	//  port number = 0,1,2,3
	port_num = 0x4;
	for i in 1..port_num {
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x0);
	}
	// Poll PSTAT.rd_port_busy_n = 0
	// Poll PSTAT.wr_port_busy_n = 0
	loop {
		rddata = mmio_read_32!(cfg_base + 0x3fc);
		io::print("Poll PSTAT.rd_port_busy_n =0\n");

		if (rddata == 0) {
			break;
		}
	}
	// disable PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	selfref_sw = get_bits_from_value(rddata, 5, 5);
	en_dfi_dram_clk_disable = get_bits_from_value(rddata, 3, 3);
	powerdown_en = get_bits_from_value(rddata, 1, 1);
	selfref_en = get_bits_from_value(rddata, 0, 0);
	rddata = modified_bits_by_value(rddata, 0, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, 0, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   //this register must not be set to 1
	rddata = modified_bits_by_value(rddata, 0, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, 0, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	cvx16_clk_gating_disable();
	//    //RFSHCTL3.dis_auto_refresh =1
	//    rddata = mmio_read_32!(cfg_base + 0x60);
	//    rddata=modified_bits_by_value(rddata, 1, 0, 0); //RFSHCTL3.dis_auto_refresh
	//    mmio_write_32!(cfg_base + 0x60, rddata);
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x30);
	// ddr_debug_num_write();
	cvx16_dfi_ca_park_prbs(1);
	// io::print("%s\n", __func__);

	//deskew start from 0x20
	rddata = mmio_read_32!(0x0080 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 0x20, 22, 16); //param_phyd_pirdlvl_deskew_start
	rddata = modified_bits_by_value(rddata, 0x1F, 30, 24); //param_phyd_pirdlvl_deskew_end
	mmio_write_32!(0x0080 + PHYD_BASE_ADDR,  rddata);

	// save param_phyd_pirdlvl_vref_training_en
	rddata = mmio_read_32!(0x008c + PHYD_BASE_ADDR);
	vref_training_en = get_bits_from_value(rddata, 2, 2);
	rddata = modified_bits_by_value(rddata, 0, 1, 1); // param_phyd_pirdlvl_rx_init_deskew_en
	rddata = modified_bits_by_value(rddata, 0, 2, 2); // param_phyd_pirdlvl_vref_training_en
	rddata = modified_bits_by_value(rddata, 0, 3, 3); // param_phyd_pirdlvl_rdvld_training_en = 0
	mmio_write_32!(0x008c + PHYD_BASE_ADDR, rddata);
// #ifdef DDR3
	rddata = mmio_read_32!(0x0188 + PHYD_BASE_ADDR);
	ddr3_mpr_mode = get_bits_from_value(rddata, 4, 4);
	if (ddr3_mpr_mode != 0) {
		// RFSHCTL3.dis_auto_refresh =1
		rddata = mmio_read_32!(cfg_base + 0x60);
		rddata = modified_bits_by_value(rddata, 1, 0, 0); // RFSHCTL3.dis_auto_refresh
		mmio_write_32!(cfg_base + 0x60, rddata);
		// MR3
		rddata = mmio_read_32!(cfg_base + 0xe0);
		rddata = modified_bits_by_value(rddata, 1, 2, 2); // Dataflow from MPR
		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
	}
// #endif
// #ifdef DDR2_3
// 	rddata = mmio_read_32!(0x0188 + PHYD_BASE_ADDR);
// 	ddr3_mpr_mode = get_bits_from_value(rddata, 4, 4);
// 	if (get_ddr_type() == DDR_TYPE_DDR3) {
// 		if (ddr3_mpr_mode) {
// 			// RFSHCTL3.dis_auto_refresh =1
// 			rddata = mmio_read_32!(cfg_base + 0x60);
// 			rddata = modified_bits_by_value(rddata, 1, 0, 0); // RFSHCTL3.dis_auto_refresh
// 			mmio_write_32!(cfg_base + 0x60, rddata);
// 			// MR3
// 			rddata = mmio_read_32!(cfg_base + 0xe0);
// 			rddata = modified_bits_by_value(rddata, 1, 2, 2); // Dataflow from MPR
// 			cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
// 		}
// 	}
// #endif
	// bist setting for dfi rdglvl
	cvx16_bist_rdlvl_init(mode);
	rddata = mmio_read_32!(0x0188 + PHYD_BASE_ADDR);
	rddata = modified_bits_by_value(rddata, 1, 0, 0); // param_phyd_dfi_rdlvl_req
	mmio_write_32!(0x0188 + PHYD_BASE_ADDR, rddata);
	io::print("dfi_rdlvl_req 1\n");

	io::print("wait retraining finish ...\n");

	loop {
		//[0] param_phyd_dfi_wrlvl_done
		//[1] param_phyd_dfi_rdglvl_done
		//[2] param_phyd_dfi_rdlvl_done
		//[3] param_phyd_dfi_wdqlvl_done
		rddata = mmio_read_32!(0x3444 + PHYD_BASE_ADDR);
		if (get_bits_from_value(rddata, 2, 2) == 0x1) {
			break;
		}
	}
	if (vref_training_en == 0x1) {
		rddata = mmio_read_32!(0x008c + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, 0, 2, 2); // param_phyd_pirdlvl_vref_training_en
		mmio_write_32!(0x008c + PHYD_BASE_ADDR, rddata);
		// final training, keep rx trig_lvl
		io::print("final training, keep rx trig_lvl\n");

		rddata = mmio_read_32!(0x0188 + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, 1, 0, 0); // param_phyd_dfi_rdlvl_req
		mmio_write_32!(0x0188 + PHYD_BASE_ADDR, rddata);
		io::print("dfi_rdlvl_req 2\n");

		io::print("wait retraining finish ...\n");

		loop {
			//[0] param_phyd_dfi_wrlvl_done
			//[1] param_phyd_dfi_rdglvl_done
			//[2] param_phyd_dfi_rdlvl_done
			//[3] param_phyd_dfi_wdqlvl_done
			rddata = mmio_read_32!(0x3444 + PHYD_BASE_ADDR);
			if (get_bits_from_value(rddata, 2, 2) == 0x1) {
				break;
			}
		}
		rddata = mmio_read_32!(0x008c + PHYD_BASE_ADDR);
		rddata = modified_bits_by_value(rddata, vref_training_en, 2, 2); // param_phyd_pirdlvl_vref_training_en
		mmio_write_32!(0x008c + PHYD_BASE_ADDR, rddata);
	}

// #ifdef DDR3
	if (ddr3_mpr_mode != 0) {
		// MR3
		rddata = mmio_read_32!(cfg_base + 0xe0);
		rddata = modified_bits_by_value(rddata, 0, 2, 2); // Normal operation
		cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
		// RFSHCTL3.dis_auto_refresh =0
		rddata = mmio_read_32!(cfg_base + 0x60);
		rddata = modified_bits_by_value(rddata, 0, 0, 0); // RFSHCTL3.dis_auto_refresh
		mmio_write_32!(cfg_base + 0x60, rddata);
	}
	// io::print("%s\n", __func__);
	// ddr_debug_wr32(0x30);
	// ddr_debug_num_write();
// #endif
// #ifdef DDR2_3
// 	if (get_ddr_type() == DDR_TYPE_DDR3) {
// 		if (ddr3_mpr_mode) {
// 			// MR3
// 			rddata = mmio_read_32!(cfg_base + 0xe0);
// 			rddata = modified_bits_by_value(rddata, 0, 2, 2); // Normal operation
// 			cvx16_synp_mrw(0x3, get_bits_from_value(rddata, 15, 0));
// 			// RFSHCTL3.dis_auto_refresh =0
// 			rddata = mmio_read_32!(cfg_base + 0x60);
// 			rddata = modified_bits_by_value(rddata, 0, 0, 0); // RFSHCTL3.dis_auto_refresh
// 			mmio_write_32!(cfg_base + 0x60, rddata);
// 		}
// 		io::print("%s\n", __func__);
// 		ddr_debug_wr32(0x30);
// 		ddr_debug_num_write();
// 	}
// #endif

	cvx16_rdvld_train();

	//    //RFSHCTL3.dis_auto_refresh =0
	//    rddata = mmio_read_32!(cfg_base + 0x60);
	//    rddata=modified_bits_by_value(rddata, 0, 0, 0); //RFSHCTL3.dis_auto_refresh
	//    mmio_write_32!(cfg_base + 0x60, rddata);
	// bist clock disable
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00040000);
	cvx16_dfi_ca_park_prbs(0);
	// restore PWRCTL.powerdown_en, PWRCTL.selfref_en
	rddata = mmio_read_32!(cfg_base + 0x30);
	rddata = modified_bits_by_value(rddata, selfref_sw, 5, 5); // PWRCTL.selfref_sw
	rddata = modified_bits_by_value(rddata, en_dfi_dram_clk_disable, 3, 3); // PWRCTL.en_dfi_dram_clk_disable
	// rddata=modified_bits_by_value(rddata, 0, 2, 2); //PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
							   //this register must not be set to 1
	rddata = modified_bits_by_value(rddata, powerdown_en, 1, 1); // PWRCTL.powerdown_en
	rddata = modified_bits_by_value(rddata, selfref_en, 0, 0); // PWRCTL.selfref_en
	mmio_write_32!(cfg_base + 0x30, rddata);
	// Write 1 to PCTRL_n.port_en
	for i in 1..port_num {
		mmio_write_32!(cfg_base + 0x490 + 0xb0 * i, 0x1);
	}
	// cvx16_rdlvl_status();
	cvx16_clk_gating_enable();
}

unsafe fn cvx16_dram_cap_check(dram_cap_in_mbyte: u8) {
    if (dram_cap_in_mbyte == 9) {
		io::print("dram_cap_check = 4Gb (512MB)\n");
	} else {
		// io::print("dram_cap_check ERROR !!! size = %x\n", size);
	}
}

unsafe fn ctrl_init_detect_dram_size() -> u8 {
    let mut rddata;
    let mut cap_in_mbyte: u8 = 0;
    
	let mut cmd: [u32; 6] = [0; 6];
	let mut i: u8;

	// dram_cap_in_mbyte = 4;
	cap_in_mbyte = 4;


	// Axsize = 3, axlen = 4, cgen
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x000e0006);

	// DDR space
	mmio_write_32!(DDR_BIST_BASE + 0x10, 0x00000000);
	mmio_write_32!(DDR_BIST_BASE + 0x14, 0xffffffff);

	// specified AXI address step
	mmio_write_32!(DDR_BIST_BASE + 0x18, 0x00000004);

	// write PRBS to 0x0 as background {{{

	cmd[0] = (1 << 30) + (0 << 21) + (3 << 12) + (5 << 9) + (0 << 8) + (0 << 0); // write 16 UI prbs

	for (i, cmd) in cmd.iter().enumerate() {
        let i = i as u32;
		mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, *cmd);
	}

	// bist_enable
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);

	// polling BIST done

    while{
        rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
        get_bits_from_value(rddata, 2, 2) == 0
    }{}

	// bist disable
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
	// }}}

	while {
		// *dram_cap_in_mbyte++;
		cap_in_mbyte += 1;
		// io::print("cap_in_mbyte =  %x\n", cap_in_mbyte);

		// write ~PRBS to (0x1 << *dram_cap_in_mbyte) {{{

		// DDR space
		mmio_write_32!(DDR_BIST_BASE + 0x10, 1 << (cap_in_mbyte + 20 - 4));

		cmd[0] = (1 << 30) + (0 << 21) + (3 << 12) + (5 << 9) + (1 << 8) + (0 << 0); // write 16 UI ~prbs

        for (i, cmd) in cmd.iter().enumerate() {
            let i = i as u32;
            mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, *cmd);
        }

		// bist_enable
		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);
		// polling BIST done

		while {
			rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
            get_bits_from_value(rddata, 2, 2) == 0
        }{}

		// bist disable
		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
		// }}}

		// check PRBS at 0x0 {{{

		// DDR space
		mmio_write_32!(DDR_BIST_BASE + 0x10, 0x00000000);
		cmd[0] = (2 << 30) + (0 << 21) + (3 << 12) + (5 << 9) + (0 << 8) + (0 << 0); // read 16 UI prbs

        for (i, cmd) in cmd.iter().enumerate() {
            let i = i as u32;
            mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, *cmd);
        }

		// bist_enable
		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);
		// polling BIST done

		while {
			rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
            get_bits_from_value(rddata, 2, 2) == 0
		} {}

		// bist disable
		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
		// }}}

        (get_bits_from_value(rddata, 3, 3) == 0) && (cap_in_mbyte < 15)
	} {}// BIST fail stop the loop

// #ifdef DDR2
// 	// fix size for DDR2
// 	cap_in_mbyte = 6;
// #endif
// #ifdef DDR2_3
// 	if (get_ddr_type() == DDR_TYPE_DDR3) {
// 		uint32_t cmd[6];
// 		uint8_t i;

// 		// dram_cap_in_mbyte = 4;
// 		cap_in_mbyte = 4;

// 		for (i = 0; i < 6; i++)
// 			cmd[i] = 0x0;

// 		// Axsize = 3, axlen = 4, cgen
// 		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x000e0006);

// 		// DDR space
// 		mmio_write_32!(DDR_BIST_BASE + 0x10, 0x00000000);
// 		mmio_write_32!(DDR_BIST_BASE + 0x14, 0xffffffff);

// 		// specified AXI address step
// 		mmio_write_32!(DDR_BIST_BASE + 0x18, 0x00000004);

// 		// write PRBS to 0x0 as background {{{

// 		cmd[0] = (1 << 30) + (0 << 21) + (3 << 12) + (5 << 9)
// 					+ (0 << 8) + (0 << 0); // write 16 UI prbs

// 		for (i = 0; i < 6; i++) {
// 			mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, cmd[i]);
// 		}

// 		// bist_enable
// 		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);

// 		// polling BIST done

// 		do {
// 			rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
// 		} while (get_bits_from_value(rddata, 2, 2) == 0);

// 		// bist disable
// 		mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
// 		// }}}

// 		do {
// 			// *dram_cap_in_mbyte++;
// 			cap_in_mbyte++;
// 			io::print("cap_in_mbyte =  %x\n", cap_in_mbyte);

// 			// write ~PRBS to (0x1 << *dram_cap_in_mbyte) {{{

// 			// DDR space
// 			mmio_write_32!(DDR_BIST_BASE + 0x10, 1 << (cap_in_mbyte + 20 - 4));

// 			cmd[0] = (1 << 30) + (0 << 21) + (3 << 12) + (5 << 9) + (1 << 8) + (0 << 0);//write 16 UI~prbs

// 			for (i = 0; i < 6; i++) {
// 				mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, cmd[i]);
// 			}

// 			// bist_enable
// 			mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);
// 			// polling BIST done

// 			do {
// 				rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
// 			} while (get_bits_from_value(rddata, 2, 2) == 0);

// 			// bist disable
// 			mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
// 			// }}}

// 			// check PRBS at 0x0 {{{

// 			// DDR space
// 			mmio_write_32!(DDR_BIST_BASE + 0x10, 0x00000000);
// 			cmd[0] = (2 << 30) + (0 << 21) + (3 << 12) + (5 << 9) + (0 << 8) + (0 << 0); // read 16 UI prbs

// 			for (i = 0; i < 6; i++) {
// 				mmio_write_32!(DDR_BIST_BASE + 0x40 + i * 4, cmd[i]);
// 			}

// 			// bist_enable
// 			mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010001);
// 			// polling BIST done

// 			do {
// 				rddata = mmio_read_32!(DDR_BIST_BASE + 0x80);
// 			} while (get_bits_from_value(rddata, 2, 2) == 0);

// 			// bist disable
// 			mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00010000);
// 			// }}}

// 		} while ((get_bits_from_value(rddata, 3, 3) == 0) && (cap_in_mbyte < 15)); // BIST fail stop the loop
// 	}
// 	if (get_ddr_type() == DDR_TYPE_DDR2) {
// 		// fix size for DDR2
// 		cap_in_mbyte = 6;
// 	}
// #endif

	// *dram_cap_in_mbyte = cap_in_mbyte;

	// save dram_cap_in_mbyte
	rddata = cap_in_mbyte as u32;
	mmio_write_32!(0x0208 + PHYD_BASE_ADDR, rddata);

	// cgen disable
	mmio_write_32!(DDR_BIST_BASE + 0x0, 0x00040000);

    cap_in_mbyte
}

unsafe fn cvx16_clk_gating_enable() {
    let mut rddata;
    // io::print("%s\n", __func__);
	// ddr_debug_wr32(0x4D);
	// ddr_debug_num_write();
	// TOP_REG_CG_EN_PHYD_TOP      0
	// TOP_REG_CG_EN_CALVL         1
	// TOP_REG_CG_EN_WRLVL         2
	// N/A                         3
	// TOP_REG_CG_EN_WRDQ          4
	// TOP_REG_CG_EN_RDDQ          5
	// TOP_REG_CG_EN_PIGTLVL       6
	// TOP_REG_CG_EN_RGTRACK       7
	// TOP_REG_CG_EN_DQSOSC        8
	// TOP_REG_CG_EN_LB            9
	// TOP_REG_CG_EN_DLL_SLAVE     10 //0:a-on
	// TOP_REG_CG_EN_DLL_MST       11 //0:a-on
	// TOP_REG_CG_EN_ZQ            12
	// TOP_REG_CG_EN_PHY_PARAM     13 //0:a-on
	// 0b10110010000001
	rddata = 0x00002C81;
	mmio_write_32!(0x44 + CV_DDR_PHYD_APB, rddata);
	//    #ifdef _mem_freq_1333
	//    #ifdef DDR2
	rddata = mmio_read_32!(cfg_base + 0x190);
	rddata = modified_bits_by_value(rddata, 6, 28, 24);
	mmio_write_32!(cfg_base + 0x190, rddata);
	//    #endif
	rddata = 0x00030033;
	mmio_write_32!(0x00F4 + PHYD_BASE_ADDR, rddata); // PHYD_SHIFT_GATING_EN
	rddata = mmio_read_32!(cfg_base + 0x30); // phyd_stop_clk
	rddata = modified_bits_by_value(rddata, 1, 9, 9);
	mmio_write_32!(cfg_base + 0x30, rddata);
	rddata = mmio_read_32!(cfg_base + 0x148); // dfi read/write clock gatting
	rddata = modified_bits_by_value(rddata, 1, 23, 23);
	rddata = modified_bits_by_value(rddata, 1, 31, 31);
	mmio_write_32!(cfg_base + 0x148, rddata);
	io::print("clk_gating_enable\n");

	// disable clock gating
	// mmio_write_32!(0x0800_a000 + 0x14 , 0x00000fff);
	// io::print("axi disable clock gating\n");
}

unsafe fn cvx16_en_rec_vol_mode() {
	if DDR2{
		let rddata = 0x00001001;
		mmio_write_32!(0x0500 + PHYD_BASE_ADDR, rddata);
		mmio_write_32!(0x0540 + PHYD_BASE_ADDR, rddata);
		io::print("cvx16_en_rec_vol_mode done\n");
	}
}


unsafe fn axi_mon_latency_setting(lat_bin_size_sel: u32){
	let mut rdata;

	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_WRITE + AXIMON_OFFSET_LAT_BIN_SIZE_SEL),
					lat_bin_size_sel);//for ddr3 1866: bin_size_sel=0d'5
	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_READ + AXIMON_OFFSET_LAT_BIN_SIZE_SEL),
					lat_bin_size_sel);

	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_WRITE + 0x00), 0x01000100);//input clk sel
	rdata = mmio_read_32!((AXI_MON_BASE + AXIMON_M1_WRITE + 0x04));//hit sel setting
	rdata = rdata & 0xfffffc00;
	rdata = rdata | 0x00000000;
	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_WRITE + 0x04), rdata);

	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_READ + 0x00), 0x01000100);
	rdata = mmio_read_32!((AXI_MON_BASE + AXIMON_M1_READ + 0x04));
	rdata = rdata & 0xfffffc00;
	rdata = rdata | 0x00000000;
	mmio_write_32!((AXI_MON_BASE + AXIMON_M1_READ + 0x04), rdata);

	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_WRITE + AXIMON_OFFSET_LAT_BIN_SIZE_SEL), lat_bin_size_sel);
	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_READ + AXIMON_OFFSET_LAT_BIN_SIZE_SEL), lat_bin_size_sel);

	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_WRITE + 0x00), 0x01000100);
	rdata = mmio_read_32!((AXI_MON_BASE + AXIMON_M5_WRITE + 0x04));
	rdata = rdata & 0xfffffc00;
	rdata = rdata | 0x00000000;
	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_WRITE + 0x04), rdata);

	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_READ + 0x00), 0x01000100);
	rdata = mmio_read_32!((AXI_MON_BASE + AXIMON_M5_READ + 0x04));
	rdata = rdata & 0xfffffc00;
	rdata = rdata | 0x00000000;
	mmio_write_32!((AXI_MON_BASE + AXIMON_M5_READ + 0x04), rdata);

	//NOTICE("mon cg en.\n");
	rdata = mmio_read_32!((DDR_TOP_BASE+0x14));
	rdata = rdata | 0x00000100;
	mmio_write_32!((DDR_TOP_BASE+0x14), rdata);
}

struct regconf {
	addr: u32,
	val: u32,
}

struct regpatch {
	addr: u32,
	mask: u32,
	val: u32,
}

impl regpatch{
    const fn new(addr: u32, mask: u32, val: u32) -> Self{
        Self { addr, mask, val }
    }
}

const ddr3_1866_patch_regs: &[regpatch] = &[
	// tune damp //////
	regpatch::new(0x08000150, 0xFFFFFFFF, 0x00000005),

	// CSB & CA driving
	regpatch::new(0x0800097c, 0xFFFFFFFF, 0x08080404),

	// CLK driving
	regpatch::new(0x08000980, 0xFFFFFFFF, 0x08080808),

	// DQ driving // BYTE0
	regpatch::new(0x08000a38, 0xFFFFFFFF, 0x00000606),
	// DQS driving // BYTE0
	regpatch::new(0x08000a3c, 0xFFFFFFFF, 0x06060606),
	// DQ driving // BYTE1
	regpatch::new(0x08000a78, 0xFFFFFFFF, 0x00000606),
	// DQS driving // BYTE1
	regpatch::new(0x08000a7c, 0xFFFFFFFF, 0x06060606),

	//trigger level //////
	// BYTE0
	regpatch::new(0x08000b24, 0xFFFFFFFF, 0x00100010),
	// BYTE1
	regpatch::new(0x08000b54, 0xFFFFFFFF, 0x00100010),

	//APHY TX VREFDQ rangex2 [1]
	//VREF DQ   //
	regpatch::new(0x08000410, 0xFFFFFFFF, 0x00120002),
	//APHY TX VREFCA rangex2 [1]
	//VREF CA  //
	regpatch::new(0x08000414, 0xFFFFFFFF, 0x00100002),

	// tx dline code
	//  BYTE0 DQ
	regpatch::new(0x08000a00, 0xFFFFFFFF, 0x06430643),
	regpatch::new(0x08000a04, 0xFFFFFFFF, 0x06430643),
	regpatch::new(0x08000a08, 0xFFFFFFFF, 0x06430643),
	regpatch::new(0x08000a0c, 0xFFFFFFFF, 0x06430643),
	regpatch::new(0x08000a10, 0xFFFFFFFF, 0x00000643),
	regpatch::new(0x08000a14, 0xFFFFFFFF, 0x0a7e007e),
	//  BYTE1 DQ
	regpatch::new(0x08000a40, 0xFFFFFFFF, 0x06480648),
	regpatch::new(0x08000a44, 0xFFFFFFFF, 0x06480648),
	regpatch::new(0x08000a48, 0xFFFFFFFF, 0x06480648),
	regpatch::new(0x08000a4c, 0xFFFFFFFF, 0x06480648),
	regpatch::new(0x08000a50, 0xFFFFFFFF, 0x00000648),
	regpatch::new(0x08000a54, 0xFFFFFFFF, 0x0a7e007e),

	//APHY RX TRIG rangex2[18] & disable lsmode[0]
	//f0_param_phya_reg_rx_byte0_en_lsmode[0]
	//f0_param_phya_reg_byte0_en_rec_vol_mode[12]
	//f0_param_phya_reg_rx_byte0_force_en_lvstl_odt[16]
	//f0_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode[8]
	//param_phya_reg_rx_byte0_en_trig_lvl_rangex2[18]
	// BYTE0 [0]
	regpatch::new(0x08000500, 0xFFFFFFFF, 0x00041001),
	//f0_param_phya_reg_rx_byte1_en_lsmode[0]
	//f0_param_phya_reg_byte1_en_rec_vol_mode[12]
	//f0_param_phya_reg_rx_byte0_force_en_lvstl_odt[16]
	//f0_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode[8]
	//param_phya_reg_rx_byte0_en_trig_lvl_rangex2[18]
	// BYTE1 [0]
	regpatch::new(0x08000540, 0xFFFFFFFF, 0x00041001),

	////////  FOR U02 ///////
	/////////// U02 enable DQS voltage mode receiver
	// f0_param_phya_reg_tx_byte0_en_tx_de_dqs[20]
	regpatch::new(0x08000504, 0xFFFFFFFF, 0x00100000),
	// f0_param_phya_reg_tx_byte1_en_tx_de_dqs[20]
	regpatch::new(0x08000544, 0xFFFFFFFF, 0x00100000),
	/////////// U02 enable MASK voltage mode receiver
	// param_phya_reg_rx_sel_dqs_wo_pream_mode[2]
	regpatch::new(0x08000138, 0xFFFFFFFF, 0x00000014),

	// BYTE0 RX DQ deskew
	regpatch::new(0x08000b00, 0xFFFFFFFF, 0x00020402),
	regpatch::new(0x08000b04, 0xFFFFFFFF, 0x05020401),
	// BYTE0  DQ8 deskew [6:0] neg DQS  [15:8]  ;  pos DQS  [23:16]
	regpatch::new(0x08000b08, 0xFFFFFFFF, 0x00313902),

	// BYTE1 RX DQ deskew
	regpatch::new(0x08000b30, 0xFFFFFFFF, 0x06000100),
	regpatch::new(0x08000b34, 0xFFFFFFFF, 0x02010303),
	// BYTE1  DQ8 deskew [6:0] neg DQS  [15:8]  ;  pos DQS  [23:16]
	regpatch::new(0x08000b38, 0xFFFFFFFF, 0x00323900),

	//Read gate TX dline + shift
	// BYTE0
	regpatch::new(0x08000b0c, 0xFFFFFFFF, 0x00000a14),
	// BYTE1
	regpatch::new(0x08000b3c, 0xFFFFFFFF, 0x00000a14),

	// CKE dline + shift CKE0 [6:0]+[13:8] ; CKE1 [22:16]+[29:24]
	regpatch::new(0x08000930, 0xFFFFFFFF, 0x04000400),
	// CSB dline + shift CSB0 [6:0]+[13:8] ; CSB1 [22:16]+[29:24]
	regpatch::new(0x08000934, 0xFFFFFFFF, 0x04000400),
];

unsafe fn ddr_patch_set(){
    for regpatch in ddr3_1866_patch_regs{
        let addr = regpatch.addr;
        let mask = regpatch.mask;
        let val = regpatch.val;

        let mut orig;

        orig = mmio_read_32!(addr);
        orig &= !mask;
        mmio_write_32!(addr, orig | (val & mask));
    }
}

unsafe fn axi_mon_start(base_register: u32){
	mmio_write_32!((AXI_MON_BASE + base_register), AXIMON_START_REGVALUE);
}

unsafe fn axi_mon_start_all() {
	axi_mon_start(AXIMON_M1_WRITE);
	axi_mon_start(AXIMON_M1_READ);
	axi_mon_start(AXIMON_M2_WRITE);
	axi_mon_start(AXIMON_M2_READ);
	axi_mon_start(AXIMON_M3_WRITE);
	axi_mon_start(AXIMON_M3_READ);
	axi_mon_start(AXIMON_M4_WRITE);
	axi_mon_start(AXIMON_M4_READ);
	axi_mon_start(AXIMON_M5_WRITE);
	axi_mon_start(AXIMON_M5_READ);
	axi_mon_start(AXIMON_M6_WRITE);
	axi_mon_start(AXIMON_M6_READ);
}

#[allow(non_upper_case_globals)]
pub const ddr_data_rate: u32 = 1866;

unsafe fn ddrc_init() {
	mmio_write_32!(0x08004000 + 0xc, 0x63746371);
	// PATCH0.use_blk_ext}:0:2:=0x1
	// PATCH0.dis_auto_ref_cnt_fix:2:1:=0x0
	// PATCH0.dis_auto_ref_algn_to_8:3:1:=0x0
	// PATCH0.starve_stall_at_dfi_ctrlupd:4:1:=0x1
	// PATCH0.starve_stall_at_abr:5:1:=0x1
	// PATCH0.dis_rdwr_switch_at_abr:6:1:=0x1
	// PATCH0.dfi_wdata_same_to_axi:7:1:=0x0
	// PATCH0.pagematch_limit_threshold:8:3=0x3
	// PATCH0.qos_sel:12:2:=0x2
	// PATCH0.burst_rdwr_xpi:16:4:=0x4
	// PATCH0.always_critical_when_urgent_hpr:20:1:=0x1
	// PATCH0.always_critical_when_urgent_lpr:21:1:=0x1
	// PATCH0.always_critical_when_urgent_wr:22:1:=0x1
	// PATCH0.disable_hif_rcmd_stall_path:24:1:=0x1
	// PATCH0.disable_hif_wcmd_stall_path:25:1:=0x1
	// PATCH0.derate_sys_en:29:1:=0x1
	// PATCH0.ref_4x_sys_high_temp:30:1:=0x1
	mmio_write_32!(0x08004000 + 0x44, 0x00000000);
	// PATCH1.ref_adv_stop_threshold:0:7:=0x0
	// PATCH1.ref_adv_dec_threshold:8:7:=0x0
	// PATCH1.ref_adv_max:16:7:=0x0
	mmio_write_32!(0x08004000 + 0x148, 0x999F0000);
	// PATCH4.t_phyd_rden:16:6=0x0
	// PATCH4.phyd_rd_clk_stop:23:1=0x0
	// PATCH4.t_phyd_wren:24:6=0x0
	// PATCH4.phyd_wr_clk_stop:31:1=0x0
	// auto gen.
	mmio_write_32!(0x08004000 + 0x0, 0x81041401);
	mmio_write_32!(0x08004000 + 0x30, 0x00000000);
	mmio_write_32!(0x08004000 + 0x34, 0x00930001);
	mmio_write_32!(0x08004000 + 0x38, 0x00020000);
	mmio_write_32!(0x08004000 + 0x50, 0x00201070);
	mmio_write_32!(0x08004000 + 0x60, 0x00000000);
	mmio_write_32!(0x08004000 + 0x64, 0x007100A4);
	mmio_write_32!(0x08004000 + 0xc0, 0x00000000);
	mmio_write_32!(0x08004000 + 0xc4, 0x00000000);

// #ifdef DDR_INIT_SPEED_UP
	mmio_write_32!(0x08004000 + 0xd0, 0x00010002);
	mmio_write_32!(0x08004000 + 0xd4, 0x00020000);
// #else
	// mmio_write_32!(0x08004000 + 0xd0, 0x000100E5);
	// mmio_write_32!(0x08004000 + 0xd4, 0x006A0000);
// #endif

	mmio_write_32!(0x08004000 + 0xdc, 0x1F140040);

// #ifdef DDR_DODT
	mmio_write_32!(0x08004000 + 0xe0, 0x04600000);
// #else
	// mmio_write_32!(0x08004000 + 0xe0, 0x00600000);
// #endif

	mmio_write_32!(0x08004000 + 0xe4, 0x000B03BF);
	mmio_write_32!(0x08004000 + 0x100, 0x0E111F10);
	mmio_write_32!(0x08004000 + 0x104, 0x00030417);
	mmio_write_32!(0x08004000 + 0x108, 0x0507060A);
	mmio_write_32!(0x08004000 + 0x10c, 0x00002007);
	mmio_write_32!(0x08004000 + 0x110, 0x07020307);
	mmio_write_32!(0x08004000 + 0x114, 0x05050303);
	mmio_write_32!(0x08004000 + 0x120, 0x00000907);
	mmio_write_32!(0x08004000 + 0x13c, 0x00000000);
	mmio_write_32!(0x08004000 + 0x180, 0xC0960026);
	mmio_write_32!(0x08004000 + 0x184, 0x00000001);
	// phyd related
	mmio_write_32!(0x08004000 + 0x190, 0x048a8305);
	// DFITMG0.dfi_t_ctrl_delay:24:5:=0x4
	// DFITMG0.dfi_rddata_use_dfi_phy_clk:23:1:=0x1
	// DFITMG0.dfi_t_rddata_en:16:7:=0xa
	// DFITMG0.dfi_wrdata_use_dfi_phy_clk:15:1:=0x1
	// DFITMG0.dfi_tphy_wrdata:8:6:=0x3
	// DFITMG0.dfi_tphy_wrlat:0:6:=0x5
	mmio_write_32!(0x08004000 + 0x194, 0x00070202);
	// DFITMG1.dfi_t_cmd_lat:28:4:=0x0
	// DFITMG1.dfi_t_parin_lat:24:2:=0x0
	// DFITMG1.dfi_t_wrdata_delay:16:5:=0x7
	// DFITMG1.dfi_t_dram_clk_disable:8:5:=0x2
	// DFITMG1.dfi_t_dram_clk_enable:0:5:=0x2
	mmio_write_32!(0x08004000 + 0x198, 0x07c13121);
	// DFILPCFG0.dfi_tlp_resp:24:5:=0x7
	// DFILPCFG0.dfi_lp_wakeup_dpd:20:4:=0xc
	// DFILPCFG0.dfi_lp_en_dpd:16:1:=0x1
	// DFILPCFG0.dfi_lp_wakeup_sr:12:4:=0x3
	// DFILPCFG0.dfi_lp_en_sr:8:1:=0x1
	// DFILPCFG0.dfi_lp_wakeup_pd:4:4:=0x2
	// DFILPCFG0.dfi_lp_en_pd:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x19c, 0x00000021);
	// DFILPCFG1.dfi_lp_wakeup_mpsm:4:4:=0x2
	// DFILPCFG1.dfi_lp_en_mpsm:0:1:=0x1
	// auto gen.
	mmio_write_32!(0x08004000 + 0x1a0, 0xC0400018);
	mmio_write_32!(0x08004000 + 0x1a4, 0x00FE00FF);
	mmio_write_32!(0x08004000 + 0x1a8, 0x80000000);
	mmio_write_32!(0x08004000 + 0x1b0, 0x000002C1);
	mmio_write_32!(0x08004000 + 0x1c0, 0x00000001);
	mmio_write_32!(0x08004000 + 0x1c4, 0x00000001);
	// address map, auto gen.
	mmio_write_32!(0x08004000 + 0x200, 0x00001F1F);
	mmio_write_32!(0x08004000 + 0x204, 0x00070707);
	mmio_write_32!(0x08004000 + 0x208, 0x00000000);
	mmio_write_32!(0x08004000 + 0x20c, 0x1F000000);
	mmio_write_32!(0x08004000 + 0x210, 0x00001F1F);
	mmio_write_32!(0x08004000 + 0x214, 0x060F0606);
	mmio_write_32!(0x08004000 + 0x218, 0x06060606);
	mmio_write_32!(0x08004000 + 0x21c, 0x00000606);
	mmio_write_32!(0x08004000 + 0x220, 0x00003F3F);
	mmio_write_32!(0x08004000 + 0x224, 0x06060606);
	mmio_write_32!(0x08004000 + 0x228, 0x06060606);
	mmio_write_32!(0x08004000 + 0x22c, 0x001F1F06);
	// auto gen.
	mmio_write_32!(0x08004000 + 0x240, 0x08000610);

// #ifdef DDR_DODT
	mmio_write_32!(0x08004000 + 0x244, 0x00000001);
// #else
	// mmio_write_32!(0x08004000 + 0x244, 0x00000000);
// #endif
// 	mmio_write_32!(0x08004000 + 0x250, 0x00003F85);
	
    // SCHED.opt_vprw_sch:31:1:=0x0
	// SCHED.rdwr_idle_gap:24:7:=0x0
	// SCHED.go2critical_hysteresis:16:8:=0x0
	// SCHED.lpddr4_opt_act_timing:15:1:=0x0
	// SCHED.lpr_num_entries:8:7:=0x1f
	// SCHED.autopre_rmw:7:1:=0x1
	// SCHED.dis_opt_ntt_by_pre:6:1:=0x0
	// SCHED.dis_opt_ntt_by_act:5:1:=0x0
	// SCHED.opt_wrcam_fill_level:4:1:=0x0
	// SCHED.rdwr_switch_policy_sel:3:1:=0x0
	// SCHED.pageclose:2:1:=0x1
	// SCHED.prefer_write:1:1:=0x0
	// SCHED.dis_opt_wrecc_collision_flush:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x254, 0x00000000);
	// SCHED1.page_hit_limit_rd:28:3:=0x0
	// SCHED1.page_hit_limit_wr:24:3:=0x0
	// SCHED1.visible_window_limit_rd:20:3:=0x0
	// SCHED1.visible_window_limit_wr:16:3:=0x0
	// SCHED1.delay_switch_write:12:4:=0x0
	// SCHED1.pageclose_timer:0:8:=0x0
	// auto gen.
	mmio_write_32!(0x08004000 + 0x25c, 0x100000F0);
	// PERFHPR1.hpr_xact_run_length:24:8:=0x20
	// PERFHPR1.hpr_max_starve:0:16:=0x6a
	mmio_write_32!(0x08004000 + 0x264, 0x100000F0);
	// PERFLPR1.lpr_xact_run_length:24:8:=0x20
	// PERFLPR1.lpr_max_starve:0:16:=0x6a
	mmio_write_32!(0x08004000 + 0x26c, 0x100000F0);
	// PERFWR1.w_xact_run_length:24:8:=0x20
	// PERFWR1.w_max_starve:0:16:=0x1a8
	mmio_write_32!(0x08004000 + 0x300, 0x00000000);
	// DBG0.dis_max_rank_wr_opt:7:1:=0x0
	// DBG0.dis_max_rank_rd_opt:6:1:=0x0
	// DBG0.dis_collision_page_opt:4:1:=0x0
	// DBG0.dis_act_bypass:2:1:=0x0
	// DBG0.dis_rd_bypass:1:1:=0x0
	// DBG0.dis_wc:0:1:=0x0
	mmio_write_32!(0x08004000 + 0x304, 0x00000000);
	// DBG1.dis_hif:1:1:=0x0
	// DBG1.dis_dq:0:1:=0x0
	mmio_write_32!(0x08004000 + 0x30c, 0x00000000);
	mmio_write_32!(0x08004000 + 0x320, 0x00000001);
	// SWCTL.sw_done:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x36c, 0x00000000);
	// POISONCFG.rd_poison_intr_clr:24:1:=0x0
	// POISONCFG.rd_poison_intr_en:20:1:=0x0
	// POISONCFG.rd_poison_slverr_en:16:1:=0x0
	// POISONCFG.wr_poison_intr_clr:8:1:=0x0
	// POISONCFG.wr_poison_intr_en:4:1:=0x0
	// POISONCFG.wr_poison_slverr_en:0:1:=0x0
	mmio_write_32!(0x08004000 + 0x400, 0x00000011);
	// PCCFG.dch_density_ratio:12:2:=0x0
	// PCCFG.bl_exp_mode:8:1:=0x0
	// PCCFG.pagematch_limit:4:1:=0x1
	// PCCFG.go2critical_en:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x404, 0x00006000);
	// PCFGR_0.rdwr_ordered_en:16:1:=0x0
	// PCFGR_0.rd_port_pagematch_en:14:1:=0x1
	// PCFGR_0.rd_port_urgent_en:13:1:=0x1
	// PCFGR_0.rd_port_aging_en:12:1:=0x0
	// PCFGR_0.read_reorder_bypass_en:11:1:=0x0
	// PCFGR_0.rd_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x408, 0x00006000);
	// PCFGW_0.wr_port_pagematch_en:14:1:=0x1
	// PCFGW_0.wr_port_urgent_en:13:1:=0x1
	// PCFGW_0.wr_port_aging_en:12:1:=0x0
	// PCFGW_0.wr_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x490, 0x00000001);
	// PCTRL_0.port_en:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x494, 0x00000007);
	// PCFGQOS0_0.rqos_map_region2:24:8:=0x0
	// PCFGQOS0_0.rqos_map_region1:20:4:=0x0
	// PCFGQOS0_0.rqos_map_region0:16:4:=0x0
	// PCFGQOS0_0.rqos_map_level2:8:8:=0x0
	// PCFGQOS0_0.rqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x498, 0x0000006a);
	// PCFGQOS1_0.rqos_map_timeoutr:16:16:=0x0
	// PCFGQOS1_0.rqos_map_timeoutb:0:16:=0x6a
	mmio_write_32!(0x08004000 + 0x49c, 0x00000e07);
	// PCFGWQOS0_0.wqos_map_region2:24:8:=0x0
	// PCFGWQOS0_0.wqos_map_region1:20:4:=0x0
	// PCFGWQOS0_0.wqos_map_region0:16:4:=0x0
	// PCFGWQOS0_0.wqos_map_level2:8:8:=0xe
	// PCFGWQOS0_0.wqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x4a0, 0x01a801a8);
	// PCFGWQOS1_0.wqos_map_timeout2:16:16:=0x1a8
	// PCFGWQOS1_0.wqos_map_timeout1:0:16:=0x1a8
	mmio_write_32!(0x08004000 + 0x4b4, 0x00006000);
	// PCFGR_1.rdwr_ordered_en:16:1:=0x0
	// PCFGR_1.rd_port_pagematch_en:14:1:=0x1
	// PCFGR_1.rd_port_urgent_en:13:1:=0x1
	// PCFGR_1.rd_port_aging_en:12:1:=0x0
	// PCFGR_1.read_reorder_bypass_en:11:1:=0x0
	// PCFGR_1.rd_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x4b8, 0x00006000);
	// PCFGW_1.wr_port_pagematch_en:14:1:=0x1
	// PCFGW_1.wr_port_urgent_en:13:1:=0x1
	// PCFGW_1.wr_port_aging_en:12:1:=0x0
	// PCFGW_1.wr_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x540, 0x00000001);
	// PCTRL_1.port_en:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x544, 0x00000007);
	// PCFGQOS0_1.rqos_map_region2:24:8:=0x0
	// PCFGQOS0_1.rqos_map_region1:20:4:=0x0
	// PCFGQOS0_1.rqos_map_region0:16:4:=0x0
	// PCFGQOS0_1.rqos_map_level2:8:8:=0x0
	// PCFGQOS0_1.rqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x548, 0x0000006a);
	// PCFGQOS1_1.rqos_map_timeoutr:16:16:=0x0
	// PCFGQOS1_1.rqos_map_timeoutb:0:16:=0x6a
	mmio_write_32!(0x08004000 + 0x54c, 0x00000e07);
	// PCFGWQOS0_1.wqos_map_region2:24:8:=0x0
	// PCFGWQOS0_1.wqos_map_region1:20:4:=0x0
	// PCFGWQOS0_1.wqos_map_region0:16:4:=0x0
	// PCFGWQOS0_1.wqos_map_level2:8:8:=0xe
	// PCFGWQOS0_1.wqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x550, 0x01a801a8);
	// PCFGWQOS1_1.wqos_map_timeout2:16:16:=0x1a8
	// PCFGWQOS1_1.wqos_map_timeout1:0:16:=0x1a8
	mmio_write_32!(0x08004000 + 0x564, 0x00006000);
	// PCFGR_2.rdwr_ordered_en:16:1:=0x0
	// PCFGR_2.rd_port_pagematch_en:14:1:=0x1
	// PCFGR_2.rd_port_urgent_en:13:1:=0x1
	// PCFGR_2.rd_port_aging_en:12:1:=0x0
	// PCFGR_2.read_reorder_bypass_en:11:1:=0x0
	// PCFGR_2.rd_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x568, 0x00006000);
	// PCFGW_2.wr_port_pagematch_en:14:1:=0x1
	// PCFGW_2.wr_port_urgent_en:13:1:=0x1
	// PCFGW_2.wr_port_aging_en:12:1:=0x0
	// PCFGW_2.wr_port_priority:0:10:=0x0
	mmio_write_32!(0x08004000 + 0x5f0, 0x00000001);
	// PCTRL_2.port_en:0:1:=0x1
	mmio_write_32!(0x08004000 + 0x5f4, 0x00000007);
	// PCFGQOS0_2.rqos_map_region2:24:8:=0x0
	// PCFGQOS0_2.rqos_map_region1:20:4:=0x0
	// PCFGQOS0_2.rqos_map_region0:16:4:=0x0
	// PCFGQOS0_2.rqos_map_level2:8:8:=0x0
	// PCFGQOS0_2.rqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x5f8, 0x0000006a);
	// PCFGQOS1_2.rqos_map_timeoutr:16:16:=0x0
	// PCFGQOS1_2.rqos_map_timeoutb:0:16:=0x6a
	mmio_write_32!(0x08004000 + 0x5fc, 0x00000e07);
	// PCFGWQOS0_2.wqos_map_region2:24:8:=0x0
	// PCFGWQOS0_2.wqos_map_region1:20:4:=0x0
	// PCFGWQOS0_2.wqos_map_region0:16:4:=0x0
	// PCFGWQOS0_2.wqos_map_level2:8:8:=0xe
	// PCFGWQOS0_2.wqos_map_level1:0:8:=0x7
	mmio_write_32!(0x08004000 + 0x600, 0x01a801a8);
	// PCFGWQOS1_2.wqos_map_timeout2:16:16:=0x1a8
	// PCFGWQOS1_2.wqos_map_timeout1:0:16:=0x1a8
}

unsafe fn ctrl_init_high_patch() {
	// enable auto PD/SR
	mmio_write_32!(0x08004000 + 0x30, 0x00000002);
	// enable auto ctrl_upd
	mmio_write_32!(0x08004000 + 0x1a0, 0x00400018);
	// enable clock gating
	mmio_write_32!(0x0800a000 + 0x14, 0x00000000);
	// change xpi to multi DDR burst
	//mmio_write_32!(0x08004000 + 0xc, 0x63786370);
}

unsafe fn ctrl_init_low_patch() {
	// disable auto PD/SR
	mmio_write_32!(0x08004000 + 0x30, 0x00000000);
	// disable auto ctrl_upd
	mmio_write_32!(0x08004000 + 0x1a0, 0xC0400018);
	// disable clock gating
	mmio_write_32!(0x0800a000 + 0x14, 0x00000fff);
	// change xpi to single DDR burst
	//mmio_write_32!(0x08004000 + 0xc, 0x63746371);
}

const FIELD: u32 = get_bits_from_value(0x18222000, 30, 28);

const fn get_bits_from_value(value: u32, msb: u32, lsb: u32) -> u32 {
    let mask = (2 << (msb))-(1 << (lsb));
	(value & mask) >> lsb
}
const fn modified_bits_by_value(mut orig: u32, value: u32, msb: u32, lsb: u32) -> u32 {
    let bitmask = (2 << (msb))-(1 << (lsb));

	orig &= !bitmask;
	return orig | ((value << lsb) & bitmask);
}


// extern "C"{
//     static mut rddata: u32;
// }  

unsafe fn ctrl_init_update_by_dram_size(dram_cap_in_mbyte: u8) {
	let mut dram_cap_in_mbyte_per_dev;

	let rddata = mmio_read_32!(0x08004000 + 0x0);
	dram_cap_in_mbyte_per_dev = dram_cap_in_mbyte;
	dram_cap_in_mbyte_per_dev >>= 1 - get_bits_from_value(rddata, 13, 12); // change sys cap to x16 cap
	dram_cap_in_mbyte_per_dev >>= 2 - get_bits_from_value(rddata, 31, 30); // change x16 cap to device cap
	match dram_cap_in_mbyte_per_dev {
        6 => {
            mmio_write_32!(0x08004000 + 0x64, 0x0071002A);
            mmio_write_32!(0x08004000 + 0x120, 0x00000903);
        }
        7 => {
            mmio_write_32!(0x08004000 + 0x64, 0x00710034);
            mmio_write_32!(0x08004000 + 0x120, 0x00000903);
        }
        8 => {
            mmio_write_32!(0x08004000 + 0x64, 0x0071004B);
            mmio_write_32!(0x08004000 + 0x120, 0x00000904);
        }
        9 => {
            mmio_write_32!(0x08004000 + 0x64, 0x0071007A);
            mmio_write_32!(0x08004000 + 0x120, 0x00000905);
        }
        10 => {
            mmio_write_32!(0x08004000 + 0x64, 0x007100A4);
            mmio_write_32!(0x08004000 + 0x120, 0x00000907);
        }
        _ => {}
	}
	// toggle refresh_update_level
	mmio_write_32!(0x08004000 + 0x60, 0x00000002);
	mmio_write_32!(0x08004000 + 0x60, 0x00000000);
}