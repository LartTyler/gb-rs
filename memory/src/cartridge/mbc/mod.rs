use crate::cartridge::constants::CONTROLLER_TYPE;
use crate::constants::{EXTERNAL_RAM_SIZE, EXTERNAL_RAM_START, ROM_BANK_SIZE};

pub mod mbc0;
pub mod mbc1;
pub mod mbc3;
pub mod mbc5;

/// Unifying trait for all Memory Bank Controller (MBC) implementations.
///
/// Please refer to each MBC's implementation for more information on how the controller
/// interprets reads from and writes to various memory segments.
pub trait MemoryBankController {
    /// Reads a value from ROM.
    ///
    /// Reads outside the supported range should return `0xFF`.
    fn rom_read(&self, address: usize) -> u8;

    /// Writes a value to the MBC registers mapped to ROM memory segments (if supported by the MBC
    /// type).
    ///
    /// Each implementation will handle this drastically differently. Please refer to each
    /// controller's documentation for a more in-depth explanation.
    fn rom_write(&mut self, address: usize, value: u8);

    /// Reads a value from the external RAM stored on the cart.
    ///
    /// In (almost?) all cases, RAM must be enabled before it can be read. Reads to disabled RAM,
    /// or RAM addresses out of range, should return `0xFF`.
    fn ram_read(&self, address: usize) -> u8;

    /// Writes a value to the external RAM stored on the cart.
    ///
    /// In (almost?) all cases, RAM must be enabled before it can be written. Writes to disabled
    /// RAM may be ignored, or may be interpreted in MBC-specific ways.
    fn ram_write(&mut self, address: usize, value: u8);
}

pub enum ControllerType {
    Mbc0,
    Mbc1,
    Mbc3,
    Mbc5,
}

impl ControllerType {
    /// Creates a new controller of the selected type.
    pub fn create(&self, rom: Vec<u8>) -> Box<dyn MemoryBankController> {
        match self {
            ControllerType::Mbc0 => Box::new(mbc0::Mbc0::new(rom)),
            ControllerType::Mbc1 => Box::new(mbc1::Mbc1::new(rom)),
            ControllerType::Mbc3 => Box::new(mbc3::Mbc3::new(rom)),
            ControllerType::Mbc5 => Box::new(mbc5::Mbc5::new(rom)),
        }
    }

    /// Attempts to create a new controller using the [`CONTROLLER_TYPE`] flag in the ROM's header.
    pub fn create_for_rom(rom: Vec<u8>) -> CreateResult {
        match rom[CONTROLLER_TYPE] {
            0x00 => Ok(ControllerType::Mbc0.create(rom)),
            0x01..=0x03 => Ok(ControllerType::Mbc1.create(rom)),
            0x0F..=0x13 => Ok(ControllerType::Mbc3.create(rom)),
            0x19..=0x1E => Ok(ControllerType::Mbc5.create(rom)),
            x => Err(CreateError::UnsupportedControllerType(x)),
        }
    }
}

pub type CreateResult = Result<Box<dyn MemoryBankController>, CreateError>;

#[derive(Debug)]
pub enum CreateError {
    /// Indicates the value in the ROM's [`CONTROLLER_TYPE`] header could not be mapped to a
    /// supported controller type.
    UnsupportedControllerType(u8),
    UnsupportedRamSize(u8),
}

/// Maps a ROM memory address to an absolute banked address.
///
/// This function assumes banks are stored in a single `Vec`, _including_ ROM0.
pub fn map_rom_address(bank: u8, address: usize) -> usize {
    bank as usize * ROM_BANK_SIZE + address
}

/// Maps a RAM memory address to an absolute banked address.
///
/// This function assumes all banks are stored in a single `Vec`.
pub fn map_ram_address(bank: u8, address: usize) -> usize {
    bank as usize * EXTERNAL_RAM_SIZE + (address - EXTERNAL_RAM_START)
}
