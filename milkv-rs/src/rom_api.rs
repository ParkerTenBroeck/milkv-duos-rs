#[repr(C)]
#[allow(non_camel_case_types)]
pub enum rsa_size {
    RSA_2048_BITS = 0,
    RSA_4096_BITS = 1,
}

pub const BOOT_SRC_TAG: isize =  0xCE00;


use core::ffi::*;

use crate::platform::boot_src;

extern "C"{

    // called from BL2
    #[link_name = "p_rom_api_load_image"]
    pub fn p_rom_api_load_image(buf: *mut c_void, offset: u32, image_size: c_size_t, retry_num: c_int) -> c_int;
    #[link_name = "p_rom_api_image_crc"]
    pub fn p_rom_api_image_crc(buf: *const i8, len: c_int) -> u32;
    #[link_name = "p_rom_api_flash_init"]
    pub fn p_rom_api_flash_init() -> c_int;

    #[link_name = "p_rom_api_get_boot_src"]
    pub fn p_rom_api_get_boot_src() -> boot_src;
    #[link_name = "p_rom_api_set_boot_src"]
    pub fn p_rom_api_set_boot_src(src: boot_src);
    #[link_name = "p_rom_api_get_number_of_retries"]
    pub fn p_rom_api_get_number_of_retries() -> c_int;

    #[link_name = "p_rom_api_verify_rsa"]
    pub fn p_rom_api_verify_rsa(message: *mut c_void, n: c_size_t, sig: *mut c_void, rsa_size: rsa_size) -> c_int;
    #[link_name = "p_rom_api_cryptodma_aes_decrypt"]
    pub fn p_rom_api_cryptodma_aes_decrypt(plain: *mut c_void, encrypted: *const c_void, len: u64, key: *mut u8, iv: *mut u8) -> c_int;

}