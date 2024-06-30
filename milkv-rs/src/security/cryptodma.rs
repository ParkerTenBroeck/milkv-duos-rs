// Register offset
pub const CRYPTODMA_DMA_CTRL: usize = 0x0;
pub const CRYPTODMA_INT_MASK: usize = 0x4;
pub const CRYPTODMA_DES_BASE_L: usize = 0x8;
pub const CRYPTODMA_DES_BASE_H: usize = 0xC;
pub const CRYPTODMA_WR_INT: usize = 0x10;
pub const CRYPTODMA_DES_KEY: usize = 0x100;
pub const CRYPTODMA_DES_IV: usize = 0x180;
pub const CRYPTODMA_SHA_PAR: usize = 0x1C0;

// Descriptor
pub const CRYPTODMA_CTRL: usize = 0x00;
pub const CRYPTODMA_CIPHER: usize = 0x01;
pub const CRYPTODMA_SRC_ADDR_L: usize = 0x04;
pub const CRYPTODMA_SRC_ADDR_H: usize = 0x05;
pub const CRYPTODMA_DST_ADDR_L: usize = 0x06;
pub const CRYPTODMA_DST_ADDR_H: usize = 0x07;
pub const CRYPTODMA_DATA_AMOUNT_L: usize = 0x08;
pub const CRYPTODMA_DATA_AMOUNT_H: usize = 0x09;
pub const CRYPTODMA_KEY: usize = 0x0A;
pub const CRYPTODMA_IV: usize = 0x12;

pub const DES_USE_BYPASS: usize = 1 << 8;
pub const DES_USE_AES: usize = 1 << 9;
pub const DES_USE_DES: usize = 1 << 10;
pub const DES_USE_SHA: usize = 1 << 12;
pub const DES_USE_DESCRIPTOR_KEY: usize = 1 << 19;
pub const DES_USE_DESCRIPTOR_IV: usize = 1 << 23;

// Cipher control for AES
pub const DECRYPT_ENABLE: usize = 0x0;
pub const CBC_ENABLE: usize = 0x1;
pub const AES_KEY_MODE: usize = 0x4;

// Cipher control for SHA
pub const SHA_MODE_SHA256: usize = 0x1 << 1;
pub const SHA_LOAD_PARAM: usize = 0x1;

// DMA control
pub const DMA_ENABLE: usize = 1;
pub const DMA_DESCRIPTOR_MODE: usize = 1;
pub const DMA_READ_MAX_BURST: usize = 16;
pub const DMA_WRITE_MAX_BURST: usize = 16;