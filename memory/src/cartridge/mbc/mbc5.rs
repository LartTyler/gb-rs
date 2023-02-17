use super::ControllerType;
use crate::cartridge::constants::RAM_SIZE;
use crate::cartridge::{get_ram_size, MemoryBankController};
use crate::constants::{EXTERNAL_RAM_SIZE, EXTERNAL_RAM_START, ROM_BANK_SIZE};

pub struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u16,
    ram_bank: u8,
    ram_enabled: bool,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = get_ram_size(rom[RAM_SIZE]).expect("Unsupported RAM size");
        let ram = vec![0; ram_size];

        // TODO MBC5s also included rumble pack support, but I'm not sure if that's really relevant
        // for the purposes of emulation. Since the write is to a RAM bank register, I might need
        // to at least account for writes to that register so it doesn't affect the current RAM
        // bank.

        Self {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
        }
    }

    fn map_rom_address(&self, address: usize) -> usize {
        self.rom_bank as usize * ROM_BANK_SIZE + address
    }

    fn map_ram_address(&self, address: usize) -> usize {
        self.ram_bank as usize * EXTERNAL_RAM_SIZE + (address - EXTERNAL_RAM_START)
    }
}

impl MemoryBankController for Mbc5 {
    fn get_controller_type(&self) -> ControllerType {
        ControllerType::Mbc5
    }

    fn rom_read(&self, address: usize) -> u8 {
        *self.rom.get(self.map_rom_address(address)).unwrap_or(&0xFF)
    }

    fn rom_write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0A != 0,
            0x2000..=0x2FFF => self.rom_bank = (self.rom_bank & 0x100) | value as u16,
            0x3000..=0x3FFF => self.rom_bank = (self.rom_bank & 0xFF) | (value & 1) as u16,
            0x4000..=0x5FFF => self.ram_bank = value & 0x0F,
            _ => (),
        }
    }

    fn ram_read(&self, address: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        *self.ram.get(self.map_ram_address(address)).unwrap_or(&0xFF)
    }

    fn ram_write(&mut self, address: usize, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let address = self.map_ram_address(address);
        let slot = self.ram.get_mut(address);

        if let Some(slot) = slot {
            *slot = value;
        }
    }
}
