use crate::constants::RAM_SIZE;
use crate::mbc::{map_ram_address, map_rom_address};
use crate::{get_ram_size, MemoryBankController};
use gb_rs_mmu::constants::{
    EXTERNAL_RAM_SIZE, EXTERNAL_RAM_START, ROM0_END, ROM0_START, ROM_BANK_END, ROM_BANK_SIZE,
    ROM_BANK_START,
};

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode_advanced: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = get_ram_size(rom[RAM_SIZE]).expect("Unsupported RAM size");
        let ram = vec![0; ram_size];

        Self {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode_advanced: false,
        }
    }
}

impl MemoryBankController for Mbc1 {
    fn rom_read(&self, address: usize) -> u8 {
        match address {
            ROM0_START..=ROM0_END => *self.rom.get(address).unwrap_or(&0xFF),
            ROM_BANK_START..=ROM_BANK_END => *self
                .ram
                .get(map_rom_address(self.rom_bank, address))
                .unwrap_or(&0xFF),
            _ => panic!("ROM read out of range for MBC1: {:#X}", address),
        }
    }

    fn rom_write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0A == 0x0A,
            0x2000..=0x3FFF => {
                let lower = core::cmp::max(1, value & 0x1F);

                self.rom_bank = (self.rom_bank & 0x60) | lower;
            }
            0x4000..=0x5FFF => {
                let bits = value & 0x03;

                if self.banking_mode_advanced {
                    // "Large RAM" carts allow RAM bank swapping in advanced mode
                    if self.ram.len() >= EXTERNAL_RAM_SIZE {
                        self.ram_bank = bits;
                    } else {
                        // "Large ROM" carts allow special case ROM banking in advanced mode that I
                        // really do not understand at the moment...
                        unimplemented!("Support for 'large ROM' carts is not yet complete");
                    }
                } else {
                    self.rom_bank = self.rom_bank & 0x1F | (bits << 5);
                }
            }
            _ => panic!("ROM register write out of range for MBC1: {:#X}", address),
        };
    }

    fn ram_read(&self, address: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        *self
            .ram
            .get(map_ram_address(self.ram_bank, address))
            .unwrap_or(&0xFF)
    }

    fn ram_write(&mut self, address: usize, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let address = map_ram_address(self.ram_bank, address);
        let slot = self.ram.get_mut(address);

        if let Some(slot) = slot {
            *slot = value;
        }
    }
}
