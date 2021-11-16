use crate::constants::RAM_SIZE;
use crate::mbc::{map_ram_address, map_rom_address};
use crate::{get_ram_size, MemoryBankController};
use gb_rs_mmu::constants::{ROM0_END, ROM0_START, ROM_BANK_END, ROM_BANK_START};

pub mod rtc;

pub enum RamMode {
    Normal(u8),
    Rtc(u8),
}

pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_rtc_enabled: bool,
    ram_mode: RamMode,
    rtc: rtc::Rtc,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = get_ram_size(rom[RAM_SIZE]).expect("Unsupported RAM size");
        let ram = vec![0; ram_size];

        Self {
            rom,
            ram,
            rom_bank: 1,
            ram_rtc_enabled: false,
            ram_mode: RamMode::Normal(0),
            rtc: rtc::Rtc::new(),
        }
    }
}

impl MemoryBankController for Mbc3 {
    fn rom_read(&self, address: usize) -> u8 {
        match address {
            ROM0_START..=ROM0_END => *self.rom.get(address).unwrap_or(&0xFF),
            ROM_BANK_START..=ROM_BANK_END => *self
                .rom
                .get(map_rom_address(self.rom_bank, address))
                .unwrap_or(&0xFF),
            _ => panic!("ROM read out of range for MBC3: {:#X}", address),
        }
    }

    fn rom_write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_rtc_enabled = value & 0x0A != 0,
            0x2000..=0x3FFF => self.rom_bank = core::cmp::max(1, value & 0x7F),
            0x4000..=0x5FFF => {
                if value <= 3 {
                    self.ram_mode = RamMode::Normal(value);
                } else {
                    self.ram_mode = RamMode::Rtc(value - 8);
                }
            }
            0x6000..=0x7FFF => self.rtc.latch_write(value),
            _ => (),
        };
    }

    fn ram_read(&self, address: usize) -> u8 {
        if !self.ram_rtc_enabled {
            return 0xFF;
        }

        match self.ram_mode {
            RamMode::Normal(bank) => *self
                .ram
                .get(map_ram_address(bank, address))
                .unwrap_or(&0xFF),
            RamMode::Rtc(register) => self.rtc.register_read(register),
        }
    }

    fn ram_write(&mut self, address: usize, value: u8) {
        if !self.ram_rtc_enabled {
            return;
        }

        match self.ram_mode {
            RamMode::Normal(bank) => {
                let slot = self.ram.get_mut(map_ram_address(bank, address));

                if let Some(slot) = slot {
                    *slot = value;
                }
            }
            RamMode::Rtc(register) => self.rtc.register_write(register, value),
        };
    }
}
