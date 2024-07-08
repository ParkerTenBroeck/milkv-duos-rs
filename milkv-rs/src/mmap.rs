#![allow(unused_parens)]
use crate::{platform::{ROM_BASE, ROM_SIZE, TPU_SRAM_BASE, TPU_SRAM_SIZE}, rom_api::p_rom_api_get_number_of_retries};

// #if ROM_LOCATION == ROM_LOCATION_HSPERI_ROM
pub const BL1_ROM_BASE: u32 = ROM_BASE;
// #elif ROM_LOCATION == ROM_LOCATION_SPINOR1
// pub const BL1_ROM_BASE: u32 = SPIF1_BASE;
// #else
// #errro "ROM_LOCATION"
// #endif

pub const BL1_ROM_SIZE: u32 = ROM_SIZE;

pub const BL1_RO_BASE: u32 = BL1_ROM_BASE;
pub const BL1_RO_LIMIT: u32 = (BL1_ROM_BASE + BL1_ROM_SIZE);

pub const BL1_VERSION_INFO_SIZE: u32 = 64; // 32bytes date string + 32bytes of SHA256
pub const BL1_DIGEST_ADDR: u32 = (BL1_ROM_BASE + BL1_ROM_SIZE - 32); // Size of SHA256

pub const BL1_ROM_MASK_SIZE: u32 = 8192;
pub const RISCV_BL1_ROM_PSMSK_VALUE: u32 = (1 << 23);
pub const CA53_BL1_ROM_PSMSK_VALUE: u32 = (1 << 11);

pub const ROM_API_ALIGN: u32 = 32;


pub const BL_RAM_BASE: u32 = TPU_SRAM_BASE;
pub const BL_RAM_SIZE: u32 = TPU_SRAM_SIZE;

/*
 * IO buffer specific defines.
 * block IO buffer's start address and size must be block size aligned
 */
pub const BL2_BASE: u32 = (BL_RAM_BASE);
pub const BL2_SIZE: u32 = (0x37000);
pub const BL2_ENTRY_OFFSET: u32 = 32;

pub const BOOT_LOG_BUF_BASE: u32 = (BL2_BASE + BL2_SIZE);
pub const BOOT_LOG_BUF_SIZE: u32 = 0x2000;

pub const PARAM1_BASE: u32 = (BOOT_LOG_BUF_BASE + BOOT_LOG_BUF_SIZE);
pub const PARAM1_SIZE: u32 = 0x1000; // same as typeof(struct fip_param1).
pub const PARAM1_SIZE_WO_SIG: u32 = 0x800;

pub const CV_IO_BUF_BASE: u32 = (PARAM1_BASE + PARAM1_SIZE);
pub const CV_IO_BUF_SIZE: u32 = 0x2000;

pub const BL1_RW_BASE: u32 = (CV_IO_BUF_BASE + CV_IO_BUF_SIZE);
pub const BL1_RW_SIZE: u32 = (0x4000);

/*
 * FIP binary defines.
 */
pub const FIP_PARAM1_MAGIC1: u64 = 0x000A31304c425643; // "CVBL01\n\0"
pub const FIP_PARAM2_MAGIC1: u64 = 0x000A3230444c5643; // "CVLD02\n\0"

pub const FLASH_NUMBER_OF_RETRIES: u32 = 8;
pub const FIP_RETRY_OFFSET: u32 = (256 * 1024);
pub fn fip_max_size() -> u32 {
    unsafe{
        p_rom_api_get_number_of_retries() as u32 * FIP_RETRY_OFFSET
    }
}

pub const PLATFORM_STACK_SIZE: u32 = 0x2000;

/*
 * DRAM map
 */

// pub const DRAM_BASE: u32 = CVIMMAP_DRAM_BASE;
// pub const DRAM_SIZE: u32 = CVIMMAP_DRAM_SIZE;

// pub const MONITOR_RUNADDR: u32 = CVIMMAP_MONITOR_ADDR;
// pub const LICENSE_FILE_ADDR: u32 = (DRAM_BASE + 0x20020); // in ATF

// pub const OPENSBI_FDT_ADDR: u32 = CVIMMAP_OPENSBI_FDT_ADDR;

// pub const DECOMP_ALLOC_SIZE: u32 = (1 << 20);
// pub const DECOMP_BUF_SIZE: u32 = (CVIMMAP_FSBL_UNZIP_SIZE - DECOMP_ALLOC_SIZE);

// pub const DECOMP_ALLOC_ADDR: u32 = CVIMMAP_FSBL_UNZIP_ADDR;
// pub const DECOMP_BUF_ADDR: u32 = (DECOMP_ALLOC_ADDR + DECOMP_ALLOC_SIZE);

// // #if DECOMP_BUF_SIZE <= 0
// // #error "FSBL_UNZIP_SIZE is not enough"
// // #endif

// pub const DECOMP_DST_SIZE: u32 = (16 << 20);

// pub const BLCP_2ND_RUNADDR: u32 = CVIMMAP_FSBL_C906L_START_ADDR;