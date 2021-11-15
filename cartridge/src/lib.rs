use crate::mbc::MemoryBankController;
use gb_rs_core::DeviceMode;
use gb_rs_mmu::constants::EXTERNAL_RAM_SIZE;

pub mod constants;
pub mod mbc;

pub struct Cartridge {
    title: String,
    manufacturer_code: Option<String>,
    device_mode: DeviceMode,
    licensee_id: u16,
    sgb_support: bool,
    version: u8,
    controller: Box<dyn MemoryBankController>,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

pub fn get_ram_size(value: u8) -> Result<usize, u8> {
    Ok(match value {
        0 => 0,
        1 => 2048,              // 2KB, however Pan Docs lists this as never actually used
        2 => EXTERNAL_RAM_SIZE, // 8KB (1 bank)
        3 => 4 * EXTERNAL_RAM_SIZE, // 32KB (4 banks)
        4 => 16 * EXTERNAL_RAM_SIZE, // 128KB (16 banks)
        5 => 8 * EXTERNAL_RAM_SIZE, // 64KB (8 banks); TCAGBD: Used by "Pokemon Crystal (J)"
        x => return Err(x),
    })
}
