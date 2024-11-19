#![no_std]
#![no_main]

use cc23x0r5_pac as cc23x0r5;
use cortex_m::interrupt::{self};
use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};


const FAPI_STATUS_SUCCESS: u32 = 0x00000000; // Function completed successfully

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0x0,
    flash_size: 0x5800,
    page_size: 0x800,
    empty_value: 0xFF,
    sectors: [{
        size: 0x800,
        address: 0x0,
    }]
});

use core::ffi::c_void;

// SHA256 types
pub type SHA256SW_Handle = *mut c_void;
pub type SHA2SW_HashType = u32;

/// HAPI table containing ROM function pointers
#[repr(C)]
pub struct HardApi {
    pub enter_standby: unsafe extern "C" fn(*const u32),
    pub crc32: unsafe extern "C" fn(*const u8, u32) -> u32,
    pub apply_copy_list: unsafe extern "C" fn(*const u32),
    pub flash_sector_erase: unsafe extern "C" fn(u32, u32) -> u32,
    pub flash_bank_erase: unsafe extern "C" fn(u32) -> u32,
    pub flash_program: unsafe extern "C" fn(u32, *const u8, u32, u32) -> u32,
    pub count_bits: unsafe extern "C" fn(u32) -> u32,
    pub secded_encode: unsafe extern "C" fn(*mut u8, *const u64, u32),
    pub secded_decode: unsafe extern "C" fn(*mut u64, *const u8, u32) -> i32,
    pub enter_application: unsafe extern "C" fn(),
    pub sha256_sw_hash_data: unsafe extern "C" fn(SHA256SW_Handle, SHA2SW_HashType, *const c_void, usize, *mut u32) -> i16,
    pub sha256_sw_start: unsafe extern "C" fn(SHA256SW_Handle, SHA2SW_HashType) -> i16,
    pub sha256_sw_add_data: unsafe extern "C" fn(SHA256SW_Handle, *const c_void, usize) -> i16,
    pub sha256_sw_finalize: unsafe extern "C" fn(SHA256SW_Handle, *mut u32) -> i16,
    pub reset_device: unsafe extern "C" fn(),
    pub sha256_sw_process_block: unsafe extern "C" fn(*mut u32, *mut u32),
    pub sha256_sw_k256: *const [u32; 64],
    pub sha256_sw_initial_digest256: *const [u32; 8],
    pub wait_us: unsafe extern "C" fn(u32),
    pub clz: unsafe extern "C" fn(u32) -> u32,
}

pub const FLASH_API_KEY: u32 = 0xB7E3A08F;
const HAPI_TABLE_BASE_ADDR: usize = 0x0F00004C;

#[inline(always)]
pub fn get_hapi() -> &'static HardApi {
    unsafe { &*(HAPI_TABLE_BASE_ADDR as *const HardApi) }
}

// Safe wrapper functions
pub fn enter_standby(copy_list: Option<&[u32]>) {
    unsafe {
        (get_hapi().enter_standby)(copy_list.map_or(core::ptr::null(), |x| x.as_ptr()));
    }
}

pub fn crc32(data: &[u8]) -> u32 {
    unsafe {
        (get_hapi().crc32)(data.as_ptr(), data.len() as u32)
    }
}

pub fn flash_sector_erase(sector_address: u32) -> u32 {
    unsafe {
        ((get_hapi().flash_sector_erase))(FLASH_API_KEY, sector_address)
    }
}

pub fn flash_bank_erase() -> u32 {
    unsafe {
        //(get_hapi().flash_bank_erase)(FLASH_API_KEY)
        0
    }
}

pub fn flash_program(data: &[u8], address: u32) -> u32 {
    unsafe {
        (get_hapi().flash_program)(FLASH_API_KEY, data.as_ptr(), address, data.len().try_into().unwrap())
    }
}

pub fn wait_us(microseconds: u32) {
    unsafe {
        (get_hapi().wait_us)(microseconds);
    }
}


fn decode_flash_status(status: u32) -> Result<(), ErrorCode> {
    match status {
        FAPI_STATUS_SUCCESS => Ok(()),
        _ => Err(ErrorCode::new(status).unwrap()),
    }
}

static mut CCH_CTRL:u32 = 0;

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {

        rtt_init_print!();
        rprintln!("Init");
        unsafe {
            let p = cc23x0r5::Peripherals::steal();
            // Store current configuration
            CCH_CTRL = p.vims.cchctrl().read().bits();

            // Clear instruction cache
            p.vims.cchctrl().write(|w| {
                w.bits(0)
            });
        }

        interrupt::disable();

        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");

        let status = flash_bank_erase();

        decode_flash_status(status)
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);

        let status = flash_sector_erase(addr);

        decode_flash_status(status)
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Progam sector addr:{}", addr);
        let status = flash_program(data, addr);

        decode_flash_status(status)
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        unsafe {
            let p = cc23x0r5::Peripherals::steal();
            // Restore configuration
            p.vims.cchctrl().write(|w| w.bits(CCH_CTRL));

            interrupt::enable();

        };
    }
}
