#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use crate::{mmio_read_32, mmio_write_32};

#[repr(C, align(512))]
#[derive(Clone, Copy)]

pub struct fip_param2 {
	pub magic1: u64,
	pub param2_cksum: u32,
	pub reserved1: u32,
	pub ddr_param_cksum: u32,
	pub ddr_param_loadaddr: u32,
	pub ddr_param_size: u32,
	pub ddr_param_reserved: u32,
	pub blcp_2nd_cksum: u32,
	pub blcp_2nd_loadaddr: u32,
	pub blcp_2nd_size: u32,
	pub blcp_2nd_runaddr: u32,
    pub monitor_cksum: u32,
	pub monitor_loadaddr: u32,
	pub monitor_size: u32,
	pub monitor_runaddr: u32,
	pub loader_2nd_reserved0: u32,
	pub loader_2nd_loadaddr: u32,
	pub loader_2nd_reserved1: u32,
	pub loader_2nd_reserved2: u32,
	pub reserved4: [u8; 4016],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct loader_2nd_header {
	pub jump0: u32,
	pub magic: u32,
	pub cksum: u32,
	pub size: u32,
	pub runaddr: u64,
	pub reserved1: u32,
	pub reserved2: u32,
}

pub const BLOCK_SIZE: u32 = 512;

pub const PARAM2_SIZE: usize = 0x1000;

pub const LOADER_2ND_MAGIC_RAW: u32 = 0x20203342; // "B3  "
pub const LOADER_2ND_MAGIC_LZMA: u32 = 0x414d3342; // "B3MA"
pub const LOADER_2ND_MAGIC_LZ4: u32 = 0x345a3342; // "B3Z4"

/*
 * Copyright (c) 2015-2016, ARM Limited and Contributors. All rights reserved.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */
 
 pub const ROM_LOCATION_HSPERI_ROM: u32 = 0;
 pub const ROM_LOCATION_SPINOR1: u32 = 1;
 
//  #if TEST_FROM_SPINOR1
//  pub const ROM_LOCATION: u32 = ROM_LOCATION_SPINOR1;
//  #else
 pub const ROM_LOCATION: u32 = ROM_LOCATION_HSPERI_ROM;
//  #endif
 
 /*
  * These definition are used to verify pub struct size and offset.
  * Hard-coded value only. Do not use sizeof() or offsetof() here.
  */
 // pub struct fip_param1->nand_info must be same as the definition in u-boot
 pub const NAND_INFO_OFFSET: u32 = 16;
 
#[repr(C)]
#[derive(Clone, Copy)]
pub struct spi_nand_info_t {
    pub version: u32,
    pub id: u32,
    pub page_size: u32,
    pub spare_size: u32,
    pub block_size: u32,
    pub pages_per_block: u32,
    pub fip_block_cnt: u32,
    pub pages_per_block_shift: u8,
    pub badblock_pos: u8,
    pub dummy_data1: [u8; 2],
    pub flags: u32,
    pub ecc_en_feature_offset: u8,
    pub ecc_en_mask: u8,
    pub ecc_status_offset: u8,
    pub ecc_status_mask: u8,
    pub ecc_status_shift: u8,
    pub ecc_status_uncorr_val: u8,
    pub dummy_data2: [u8; 2],
    pub erase_count: u32, // erase count for sys base block
    pub sck_l: u8,
    pub sck_h: u8,
    pub max_freq: u16,
    pub sample_param: u32,
    pub xtal_switch: u8,
    pub dummy_data3: [u8; 71],
}
 
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct spinor_info_t {
    pub ctrl: u32,
    pub dly_ctrl: u32,
    pub tran_csr: u32,
    pub opt: u32,
    pub reserved_1: u32,
    pub reserved_2: u32,
    pub reserved_3: u32,
    pub reserved_4: u32,
    pub reserved_5: u32,
}
 
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct chip_conf {
    pub reg: u32,
    pub value: u32,
}
 
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct fip_flags {
    // pub struct {
    //     u8 rsa_size : 2;
    //     u8 scs : 2;
    //     u8 encrypted : 2;
    //     u8 reserved1 : 2;
    // };
    pub flags: u8,
    pub reserved2: [u8; 7],
}
 
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct fip_param1 {
    pub magic1: u64,
    pub magic2: u32,
    pub param_cksum: u32,
    pub nand_info: spi_nand_info_t,
    pub spinor_info: spinor_info_t,
    pub fip_flags: fip_flags,
    pub chip_conf_size: u32,
    pub blcp_img_cksum: u32,
    pub blcp_img_size: u32,
    pub blcp_img_runaddr: u32,
    pub blcp_param_loadaddr: u32,
    pub blcp_param_size: u32,
    pub bl2_img_cksum: u32,
    pub bl2_img_size: u32,
    pub bld_img_size: u32,
    pub param2_loadaddr: u32,
    pub param2_size: u32,
    pub chip_conf: [chip_conf; 95],
    pub bl_ek: [u8; 32],
    pub root_pk: [u8; 512],
    pub bl_pk: [u8; 512],
    pub bl_pk_sig: [u8; 512],
    pub chip_conf_sig: [u8; 512],
    pub bl2_img_sig: [u8; 512],
    pub blcp_img_sig: [u8; 512],
}
 
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct blcp_param_head {
    pub magic: u32,
    pub cksum: u32,
}
 
 pub const BLCP_PARAM_MAGIC: u32 = 0x52505043; // "CPPR"
 pub const BLCP_PARAM_MAX_SIZE: u32 = 512;
 pub const BLCP_PARAM_RETRY: u32 = 4;
 
 #[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct bl2_head {
    pub magic1: u64,
    pub magic2: u64,
    pub msid: u32,
    pub version: u32,
    pub reserved1: u64,
}
 
 /* this pub structure should be modified all of fsbl & MCU & osdrv side */
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct transfer_config_t {
    pub conf_magic: u32,
    pub conf_size: u32,  //conf_size exclude mcu_status & linux_status
    pub isp_buffer_addr: u32,
    pub isp_buffer_size: u32,
    pub encode_img_addr: u32,
    pub encode_img_size: u32,
    pub encode_buf_addr: u32,
    pub encode_buf_size: u32,
    pub dump_print_enable: u8,
    pub dump_print_size_idx: u8,
    pub image_type: u16,
    pub checksum: u16, // checksum exclude mcu_status & linux_status
    pub mcu_status: u8,
    pub linux_status: u8,
}
 
#[repr(C)]
pub enum _MUC_STATUS_E {
    MCU_STATUS_NONOS_INIT = 1,
    MCU_STATUS_NONOS_RUNNING,
    MCU_STATUS_NONOS_DONE,
    MCU_STATUS_RTOS_T1_INIT,  // before linux running
    MCU_STATUS_RTOS_T1_RUNNING,
    MCU_STATUS_RTOS_T2_INIT,  // after linux running
    MCU_STATUS_RTOS_T2_RUNNING,
    MCU_STATUS_LINUX_INIT,
    MCU_STATUS_LINUX_RUNNING,
}
 
#[repr(C)]
pub enum E_IMAGE_TYPE {
    E_FAST_JEPG = 1,
    E_FAST_H264,
    E_FAST_H265,
}
 

#[repr(C)]
pub enum DUMP_PRINT_SIZE_E {
    DUMP_PRINT_SZ_IDX_0K = 0,
    DUMP_PRINT_SZ_IDX_4K = 12, // 4096 = 1<<12
    DUMP_PRINT_SZ_IDX_8K,
    DUMP_PRINT_SZ_IDX_16K,
    DUMP_PRINT_SZ_IDX_32K,
    DUMP_PRINT_SZ_IDX_LIMIT,
}
 
 pub const BOOT_SRC_TAG: isize = 0xCE00;
 
 // NO ZERO in boot_src
#[repr(C)]
pub enum boot_src {
    // Read from flash
    BOOT_SRC_SPI_NAND = 0x0 | BOOT_SRC_TAG,
    BOOT_SRC_SPI_NOR = 0x2 | BOOT_SRC_TAG,
    BOOT_SRC_EMMC = 0x3 | BOOT_SRC_TAG,

    // Download
    BOOT_SRC_SD = 0xA0 | BOOT_SRC_TAG,
    BOOT_SRC_USB = 0xA3 | BOOT_SRC_TAG,
    BOOT_SRC_UART = 0xA5 | BOOT_SRC_TAG,
}
 
 pub const DOWNLOAD_BUTTON: u32 = 0x1;
 pub const DOWNLOAD_DISABLE: u32 = 0x2;
 
// #[repr(C, packed)]
// union sw_info {
//     pub value: u32,
//     pub struct {
//         u32 dis_dbg_inject : 1;
//         u32 usb_polling_time : 1;
//         u32 dis_uart_msg: 1;
//         u32 reserved : 2;
//         u32 usb_vid : 16;
//         u32 dis_usb_rxf : 1;
//         u32 sd_dl : 2;
//         u32 usd_dl : 2;
//         u32 uart_dl : 2;
//         u32 sd_polarity : 2;
//         u32 reset_type : 1;
//         u32 sw_info_enable : 1;
//     };
// }
 
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct _time_records {
    pub fsbl_start: u16,
    pub ddr_init_start: u16,
    pub ddr_init_end: u16,
    pub release_blcp_2nd: u16,
    pub load_loader_2nd_end: u16,
    pub fsbl_decomp_start: u16,
    pub fsbl_decomp_end: u16,
    pub fsbl_exit: u16,
    pub uboot_start: u16,
    pub bootcmd_start: u16,
    pub decompress_kernel_start: u16,
    pub kernel_start: u16,
    pub kernel_run_init_start: u16,
}
 
//  extern pub struct _time_records *time_records;
 
 /*
  * PINMUX
  */
 pub const PINMUX_SPI0: u32 = 11;
 pub const PINMUX_SDIO0: u32 = 22;
 pub const PINMUX_EMMC: u32 = 25;
 pub const PINMUX_SPI_NOR: u32 = 26;
 pub const PINMUX_SPI_NAND: u32 = 27;
 
 /*
  * SoC memory map
  */
 pub const SEC_SUBSYS_BASE: u32 = 0x02000000;
 pub const SEC_CRYPTODMA_BASE: u32 = (SEC_SUBSYS_BASE + 0x00060000);
 pub const SEC_FAB_FIREWALL: u32 = (SEC_SUBSYS_BASE + 0x00090000);
 pub const SEC_DDR_FIREWALL: u32 = (SEC_SUBSYS_BASE + 0x000A0000);
 pub const SEC_SYS_BASE: u32 = (SEC_SUBSYS_BASE + 0x000B0000);
 pub const SEC_EFUSE_BASE: u32 = (SEC_SUBSYS_BASE + 0x000C0000);
 
 pub const TOP_BASE: u32 = 0x03000000;
 pub const PINMUX_BASE: u32 = (TOP_BASE + 0x00001000);
 pub const CLKGEN_BASE: u32 = (TOP_BASE + 0x00002000);
 pub const RST_BASE: u32 = (TOP_BASE + 0x00003000);
 pub const WATCHDOG_BASE: u32 = (TOP_BASE + 0x00010000);
 pub const GPIO_BASE: u32 = (TOP_BASE + 0x00020000);
 pub const EFUSE_BASE: u32 = (TOP_BASE + 0x00050000);
 pub const PLL_G2_BASE: u32 = (TOP_BASE + 0x00002800);
 pub const PWM0_BASE: u32 = (TOP_BASE + 0x60000);
 
 pub const HSPERI_BASE: u32 = 0x04000000;
 pub const SPINAND_BASE: u32 = (HSPERI_BASE + 0x00060000);
 pub const UART0_BASE: u32 = (HSPERI_BASE + 0x00140000);
 pub const UART2_BASE: u32 = (HSPERI_BASE + 0x00160000);
 pub const USB_BASE: u32 = (HSPERI_BASE + 0x00340000);
 pub const EMMC_BASE: u32 = (HSPERI_BASE + 0x00300000);
 pub const SDIO_BASE: u32 = (HSPERI_BASE + 0x00310000);
 pub const SYSDMA_BASE: u32 = (HSPERI_BASE + 0x00330000);
 pub const SPIF_BASE: u32 = 0x10000000;
 pub const SPIF1_BASE: u32 = 0x05400000;
 
 pub const RTC_SYS_BASE: u32 = 0x05000000;
 pub const RTC_GPIO_BASE: u32 = (RTC_SYS_BASE + 0x00021000);
 
 pub const RTC_SRAM_BASE: u32 = (RTC_SYS_BASE + 0x00200000);
 pub const RTC_SRAM_SIZE: u32 = 0x6000; // 24KiB
 
 pub const AXI_SRAM_BASE: u32 = 0x0E000000;
 pub const AXI_SRAM_SIZE: u32 = 0x40;
 pub const AXI_SRAM_RTOS_OFS: u32 = 0x7C;
 pub const AXI_SRAM_RTOS_BASE: u32 = (AXI_SRAM_BASE + AXI_SRAM_RTOS_OFS);
 pub const CVI_RTOS_MAGIC_CODE: u32 = 0xABC0DEF;
 
 pub const MAILBOX_FIELD: u32 = 0x1900400;
 
 pub const C906_MAGIC_HEADER: u32 = 0xA55AC906; // master cpu is c906
 pub const CA53_MAGIC_HEADER: u32 = 0xA55ACA53; // master cpu is ca53
 
//  #ifdef __riscv
 pub const RTOS_MAGIC_HEADER: u32 = C906_MAGIC_HEADER;
//  #else
//  pub const RTOS_MAGIC_HEADER: u32 = CA53_MAGIC_HEADER;
//  #endif
 
 pub const ROM_SIZE: u32 = 0x18000; // 96KiB
 pub const TPU_SRAM_ORIGIN_BASE: u32 = 0x0C000000;
 pub const TPU_SRAM_SIZE: u32 = 0x40000; // 256KiB
 
//  #if ROM_LOCATION == ROM_LOCATION_HSPERI_ROM
//  #ifdef __riscv
     pub const ROM_BASE: u32 = 0x04418000; // no mirrored address for c906b
     pub const TPU_SRAM_BASE: u32 = TPU_SRAM_ORIGIN_BASE; // no mirrored address for c906b
     pub const SYSMAP_MIRROR_OFFSET: u32 = 0x20000000;
//  #else
//  pub const ROM_BASE: u32 = 0x40000000 // mirrored address;
//  pub const TPU_SRAM_BASE: u32 = 0x40100000 // mirrored address;
//  #endif
//  #elif ROM_LOCATION == ROM_LOCATION_SPINOR1
//      pub const ROM_BASE: u32 = (HSPERI_BASE + 0x00400000);
//      pub const TPU_SRAM_BASE: u32 = TPU_SRAM_ORIGIN_BASE;
//  #else
//  #errro "ROM_LOCATION"
//  #endif
 
 /*
  * AXI SRAM
  */
 pub const EFUSE_SW_INFO_ADDR: u32 = (AXI_SRAM_BASE);
 pub const EFUSE_SW_INFO_SIZE: u32 = 4;
 
 pub const BOOT_SOURCE_FLAG_ADDR: u32 = (EFUSE_SW_INFO_ADDR + EFUSE_SW_INFO_SIZE);
 pub const BOOT_SOURCE_FLAG_SIZE: u32 = 4;
 pub const MAGIC_NUM_USB_DL: u32 = 0x4D474E31; // MGN1
 pub const MAGIC_NUM_SD_DL: u32 = 0x4D474E32; // MGN2
 pub const MAGIC_NUM_UART_DL: u32 = 0x4D474E33; // MGN3
 
 pub const BOOT_LOG_LEN_ADDR: u32 = (BOOT_SOURCE_FLAG_ADDR + BOOT_SOURCE_FLAG_SIZE); // 0x0E000008
 pub const BOOT_LOG_LEN_SIZE: u32 = 4;
 
 pub const TIME_RECORDS_ADDR: u32 = (AXI_SRAM_BASE + 0x10); // 0x0E000010
 
 // only for debugging
 pub const ATF_DBG_REG: u32 = (BOOT_LOG_LEN_ADDR + BOOT_LOG_LEN_SIZE);
 pub const ATF_ERR_REG: u32 = (ATF_DBG_REG + 0x04);
 pub const ATF_ERR_INFO0: u32 = (ATF_DBG_REG + 0x08);
 pub const CP_STATE_REG: u32 = (ATF_DBG_REG + 0x0C);
 
//  pub const ATF_ERR: u32 = (((unsigned int __volatile__ *)ATF_ERR_REG)[0]);
 
 /* End of AXI SRAM */
 
 /*
  * Some data must be aligned on the biggest cache line size in the platform.
  * This is known only to the platform as it might have a combination of
  * integrated and external caches.
  */
 pub const CACHE_WRITEBACK_SHIFT: u32 = 6;
 pub const CACHE_WRITEBACK_GRANULE: u32 = (1 << CACHE_WRITEBACK_SHIFT);
 
 pub const PLAT_PHY_ADDR_SPACE_SIZE: u64 = (1 << 32);
 pub const PLAT_VIRT_ADDR_SPACE_SIZE: u64 = (1 << 32);
 pub const MAX_MMAP_REGIONS: u32 = 8;
 pub const MAX_XLAT_TABLES: u32 = 6; // varies when memory layout changes
 
 /*
  * UART definitions
  */
//  #if ROM_LOCATION == ROM_LOCATION_HSPERI_ROM
 pub const PLAT_BOOT_UART_BASE: u32 = UART0_BASE;
//  #elif ROM_LOCATION == ROM_LOCATION_SPINOR1
//  pub const PLAT_BOOT_UART_BASE: u32 = UART2_BASE;
//  #else
//  #errro "ROM_LOCATION"
//  #endif
 
 /*
  * TOP registers.
  */
 pub const REG_TOP_CHIPID: u32 = (TOP_BASE + 0x0);
 pub const REG_TOP_CONF_INFO: u32 = (TOP_BASE + 0x4);
 pub const REG_TOP_USB_PHY_CTRL: u32 = (TOP_BASE + 0x48);
 
 pub const BIT_C906L_BOOT_FROM_RTCSYS_EN: u32 = (1 << 6);
 
 pub const REG_GP_REG0: u32 = (TOP_BASE + 0x80);
 pub const REG_GP_REG1: u32 = (TOP_BASE + 0x84);
 pub const REG_GP_REG2: u32 = (TOP_BASE + 0x88); // Trig simulation bench to increse cntpct_el0
 pub const REG_GP_REG3: u32 = (TOP_BASE + 0x8C);
 
 pub const REG_USB_ECO_REG: u32 = (TOP_BASE + 0xB4);
 pub const REG_USB_ECO_RXF: u32 = 0x80;
 
 pub const REG_CLK_BYPASS_SEL_REG: u32 = (CLKGEN_BASE + 0x30);
 pub const REG_CLK_DIV0_CTL_CA53_REG: u32 = (CLKGEN_BASE + 0x40);
 pub const REG_CLK_DIV0_CTL_CPU_AXI0_REG: u32 = (CLKGEN_BASE + 0x48);
 pub const REG_CLK_DIV0_CTL_TPU_AXI_REG: u32 = (CLKGEN_BASE + 0x54);
 
 pub const REG_CLK_DIV_AXI4: u32 = (CLKGEN_BASE + 0xB8);
 
 pub const REG_PLL_G2_CTRL: u32 = (PLL_G2_BASE + 0x0);
 pub const REG_APLL0_CSR: u32 = (PLL_G2_BASE + 0x0C);
 pub const REG_DISPPLL_CSR: u32 = (PLL_G2_BASE + 0x10);
 pub const REG_CAM0PLL_CSR: u32 = (PLL_G2_BASE + 0x14);
 pub const REG_CAM1PLL_CSR: u32 = (PLL_G2_BASE + 0x18);
 pub const REG_PLL_G2_SSC_SYN_CTRL: u32 = (PLL_G2_BASE + 0x40);
 pub const REG_APLL_SSC_SYN_CTRL: u32 = (PLL_G2_BASE + 0x50);
 pub const REG_APLL_SSC_SYN_SET: u32 = (PLL_G2_BASE + 0x54);
 pub const REG_DISPPLL_SSC_SYN_CTRL: u32 = (PLL_G2_BASE + 0x60);
 pub const REG_DISPPLL_SSC_SYN_SET: u32 = (PLL_G2_BASE + 0x64);
 pub const REG_CAM0PLL_SSC_SYN_CTRL: u32 = (PLL_G2_BASE + 0x70);
 pub const REG_CAM0PLL_SSC_SYN_SET: u32 = (PLL_G2_BASE + 0x74);
 pub const REG_CAM1PLL_SSC_SYN_CTRL: u32 = (PLL_G2_BASE + 0x80);
 pub const REG_CAM1PLL_SSC_SYN_SET: u32 = (PLL_G2_BASE + 0x84);
 
 pub const SHIFT_TOP_USB_ID: u32 = 8;
 pub const SHIFT_TOP_USB_VBUS: u32 = 9;
 pub const BIT_TOP_USB_ID: u32 = (1 << SHIFT_TOP_USB_ID);
 pub const BIT_TOP_USB_VBUS: u32 = (1 << SHIFT_TOP_USB_VBUS);
 
 pub const REG_TOP_SD_PWRSW_CTRL: u32 = (TOP_BASE + 0x1F4);
 pub const REG_TOP_SD_CTRL_OPT: u32 = (TOP_BASE + 0x294);
 pub const BIT_IO_TRAP_SD0_PWR_DIN: u32 = (1 << 27);
 pub const BIT_SD0_PWR_EN_POLARITY: u32 = (1 << 16);
 pub const BIT_SD1_PWR_EN_POLARITY: u32 = (1 << 17);
 
 pub const PWM_HLPERIOD0: u32 = 0x0;
 pub const PWM_PERIOD0: u32 = 0x4;
 pub const PWM_HLPERIOD1: u32 = 0x8;
 pub const PWM_PERIOD1: u32 = 0xC;
 pub const PWM_HLPERIOD2: u32 = 0x10;
 pub const PWM_PERIOD2: u32 = 0x14;
 pub const PWM_HLPERIOD3: u32 = 0x18;
 pub const PWM_PERIOD3: u32 = 0x1C;
 pub const PWM_START: u32 = 0x44;
 pub const PWM_OE: u32 = 0xD0;
 
 /*
  * DEBUG register
  */
 pub const ATF_STATE_REG: u32 = REG_GP_REG1;
//  pub const ATF_STATE: u32 = (((unsigned int volatile *)ATF_STATE_REG)[0]);
 
 pub const ATF_WAIT_DEBUG_REG: u32 = REG_GP_REG0;
 pub const ATF_WAIT_DEBUG_MAGIC: u32 = 0x6526228C;
 pub const ATF_WAIT_DEBUG_TIMEOUT: u32 = 1000;
 
 /*
  * Firewall register
  */
 pub const FABFW_ROM_PSMSK: u32 = 0x5C;
 
 /*
  * Arch timer definitions
  */
 pub const SYS_COUNTER_FREQ_IN_SECOND: u32 = 25000000;
 
 /*
  * If enable, the global variable of emmc/sd clock could be changed by blp
  */
//  #define SUPPORT_SD_EMMC_CLOCK_ADJUSTMENT
 
 /*
  * UART buadrate and clock
  */
 pub const PLAT_CONSOLE_BAUDRATE: u32 = 115200;
 pub const PLAT_UART_CLK_IN_HZ: u32 = 25000000;
 
 /*
  * UART download
  */
 pub const UART_DL_MAGIC: u32 = 0x5552444c; // "URDL"
 pub const UART_DL_KERMIT_TIMEROUT: u32 = 10000; // ms
 pub const UART_DL_BAUDRATE: u32 = 1500000;
 
 /*
  * SD/EMMC definitions
  */
 pub const PLAT_SD_CLK: u32 = 25000000;
 pub const PLAT_EMMC_CLK: u32 = 25000000;
 
//  #define ENABLE_SDIO_IO_CELL_POWER
//  #define ENABLE_SDIO_SOURCE_SELECT_SETTING
 
//  pub const EMMC_BUS_WIDTH: u32 = EMMC_BUS_WIDTH_1;
 pub const DEFAULT_DIV_EMMC_INIT_CLOCK: u32 = 0x2;
 
 /*
  * USB definitions
  */
//  #define USB_PHY_DETECTION








pub unsafe fn init_pll_speed() {
    // OD clk setting
    let mut value;
    let mut byp0_value;

    let pll_syn_set = [
        614400000, // set apll synthesizer  98.304 M
        610080582, // set disp synthesizer  99 M
        610080582, // set cam0 synthesizer  99 M
        //586388132, // set cam1 synthesizer  103 M
        615164587, // set cam1 synthesizer  98.18181818 M
    ];
    const DISP_CLK_CSR: u32 = /*disppll_pre_div_sel*/0x000001 
                            | /*disppll_post_div_sel*/(0b0000001 << 8) 
                            | /*disppll_sel_mode*/(0b01 << 15) 
                            | /*disppll_div_sel*/(0b010101 << 17) 
                            | /*disppll_ictrl*/(0b000 << 24);
    let pll_csr = [
        0x00208201, // set apll *16/2 (786.432 MHz)
        0x00188101, // DISP_CLK_CSR, // set disp *12/1 (1188 MHz)
        // 0x00188101, // set cam0 *12/1 (1188 MHz)
        0x00308201, // set cam0 *24/2 (1188 MHz)
        //0x00148101, // set cam1 *10/1 (1030 MHz)
        0x00168101, // set cam1 *11/1 (1080 MHz)
    ];

    // NOTICE("PLLS/OD.\n");
    unsafe fn config_core_power(low_period: u32) {
        /*
         * low_period = 0x42; // 0.90V
         * low_period = 0x48; // 0.93V
         * low_period = 0x4F; // 0.96V
         * low_period = 0x58; // 1.00V
         * low_period = 0x5C; // 1.02V
         * low_period = 0x62; // 1.05V
         * low_period = 0x62; // 1.05V
         */
        mmio_write_32!(PWM0_BASE + PWM_HLPERIOD0, low_period);
        mmio_write_32!(PWM0_BASE + PWM_PERIOD0, 0x64);
        mmio_write_32!(PINMUX_BASE + 0xA4, 0x0); // set pinmux for pwm0
        mmio_write_32!(PWM0_BASE + PWM_START, 0x1); // enable bit0:pwm0
        mmio_write_32!(PWM0_BASE + PWM_OE, 0x1); // output enable bit0:pwm0
        crate::timer::mdelay(10);
    }

    // set vddc for OD clock
    config_core_power(0x58); //1.00V

    // store byp0 value
    byp0_value = mmio_read_32!(0x03002030);

    // switch clock to xtal
    mmio_write_32!(0x03002030, 0xffffffff);
    mmio_write_32!(0x03002034, 0x0000003f);

    //set mipipll = 900MHz
    mmio_write_32!(0x03002808, 0x05488101);

    // set synthersizer clock
    mmio_write_32!(REG_PLL_G2_SSC_SYN_CTRL, 0x3F); // enable synthesizer clock enable,
                                                   // [0]: 1: MIPIMPLL(900)/1=900MHz,
                                                   //      0: MIPIMPLL(900)/2=450MHz

    for i in 0..4 {
        mmio_write_32!(REG_APLL_SSC_SYN_SET + 0x10 * i, pll_syn_set[i as usize]); // set pll_syn_set

        value = mmio_read_32!(REG_APLL_SSC_SYN_CTRL + 0x10 * i);
        value |= 1; // [0]: sw update (w1t: write one toggle)
        value &= !(1 << 4); // [4]: bypass = 0
        mmio_write_32!(REG_APLL_SSC_SYN_CTRL + 0x10 * i, value);

        mmio_write_32!(REG_APLL0_CSR + 4 * i, pll_csr[i as usize]); // set pll_csr
    }

    value = mmio_read_32!(REG_PLL_G2_CTRL);
    value = value & (!0x00011111);
    mmio_write_32!(REG_PLL_G2_CTRL, value); //clear all pll PD

    // set mpll = 1050MHz
    mmio_write_32!(0x03002908, 0x05548101);

    // set clk_sel_23: [23] clk_sel for clk_c906_0 = 1 (DIV_IN0_SRC_MUX)
    // set clk_sel_24: [24] clk_sel for clk_c906_1 = 1 (DIV_IN0_SRC_MUX)
    mmio_write_32!(0x03002020, 0x01800000);

    // set div, src_mux of clk_c906_0: [20:16]div_factor=1, [9:8]clk_src = 3 (mpll), 1050/1 = 1050MHz
    mmio_write_32!(0x03002130, 0x00010309);

    // set div, src_mux of clk_c906_1: [20:16]div_factor=1, [9:8]clk_src = 1 (a0pll), 786.432/1 = 786.432MHz
    mmio_write_32!(0x03002138, 0x00010109);


    // set tpll = 1400MHz
    mmio_write_32!(0x0300290C, 0x07708101);

    mmio_write_32!(0x03002048, 0x00020109); //clk_cpu_axi0 = DISPPLL(1188) / 2
    mmio_write_32!(0x03002054, 0x00020009); //clk_tpu = TPLL(1400) / 2 = 700MHz
    mmio_write_32!(0x03002064, 0x00080009); //clk_emmc = FPLL(1500) / 8 = 187.5MHz
    mmio_write_32!(0x03002088, 0x00080009); //clk_spi_nand = FPLL(1500) / 8 = 187.5MHz
    mmio_write_32!(0x03002098, 0x00200009); //clk_sdma_aud0 = APLL(786.432) / 32 = 24.576MHz
    mmio_write_32!(0x03002120, 0x000F0009); //clk_pwm_src = FPLL(1500) / 15 = 100MHz
    mmio_write_32!(0x030020A8, 0x00010009); //clk_uart -> clk_cam0_200 = XTAL(25) / 1 = 25MHz
    mmio_write_32!(0x030020E4, 0x00030209); //clk_axi_video_codec = CAM1PLL(1080) / 3 = 360MHz
    mmio_write_32!(0x030020EC, 0x00020109); //clk_vc_src0 = MIPIPLL(900) / 2 = 450MHz
    mmio_write_32!(0x030020C8, 0x00030009); //clk_axi_vip = MIPIPLL(900) / 3 = 300MHz
    mmio_write_32!(0x030020D0, 0x00060309); //clk_src_vip_sys_0 = FPLL(1500) / 6 = 250MHz
    mmio_write_32!(0x030020D8, 0x000F0209); //clk_src_vip_sys_1 = DISPPLL(1188)/ 4 = 297MHz
    mmio_write_32!(0x03002110, 0x000F0209); //clk_src_vip_sys_2 = DISPPLL(1188) / 2 = 594MHz
                                            //mmio_write_32(0x03002140, 0x00020009); //clk_src_vip_sys_3 = MIPIPLL(900) / 2 = 450MHz
    mmio_write_32!(0x03002144, 0x00030309); //clk_src_vip_sys_4 = FPLL(1500) / 3 = 500MHz


    let div_clk_axi6 = (0x03002000 + 0x0bc) as *mut u32;
    div_clk_axi6.write_volatile((1 << 3) | 0x40000 | (1));

    // set hsperi clock to PLL (FPLL) div by 5  = 300MHz
    mmio_write_32!(0x030020B8, 0x00050009); //--> CLK_AXI4

    // set rtcsys clock to PLL (FPLL) div by 5  = 300MHz
    mmio_write_32!(0x0300212C, 0x00050009); // CLK_SRC_RTC_SYS_0

    // disable powerdown, mipimpll_d3_pd[2] = 0
    mmio_clrbits_32(0x030028A0, 0x4);

    // disable powerdown, cam0pll_d2_pd[1]/cam0pll_d3_pd[2] = 0
    mmio_clrbits_32(0x030028AC, 0x6);


    //wait for pll stable
    crate::timer::udelay(200);

    // switch clock to PLL from xtal except clk_axi4 & clk_spi_nand
    byp0_value &= (
        1 << 8 | //clk_spi_nand
        0 // 1 << 19 // clk_axi4
    );
    mmio_write_32!(0x03002030, byp0_value); // REG_CLK_BYPASS_SEL0_REG
    mmio_write_32!(0x03002034, 0x0); // REG_CLK_BYPASS_SEL1_REG

}

unsafe fn mmio_clrbits_32(addr: u32, clear: u32) {
    mmio_write_32!(addr, mmio_read_32!(addr) & !clear);
}

unsafe fn mmio_setbits_32(addr: u32, set: u32) {
    mmio_write_32!(addr, mmio_read_32!(addr) | set);
}

pub unsafe fn reset_c906l(){
	mmio_clrbits_32(0x3003024, 1 << 6);
}

pub unsafe fn start_c906l(){
	mmio_setbits_32(0x3003024, 1 << 6);
}

pub unsafe fn reset_c906l_to_addr(reset_address: usize){
	mmio_clrbits_32(0x3003024, 1 << 6);

	mmio_setbits_32(SEC_SYS_BASE + 0x04, 1 << 13);
	mmio_write_32!(SEC_SYS_BASE + 0x20, reset_address as u32);
	mmio_write_32!(SEC_SYS_BASE + 0x24, (reset_address >> 32) as u32);

	mmio_setbits_32(0x3003024, 1 << 6);
}