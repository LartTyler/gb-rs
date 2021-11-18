use crate::cartridge::Cartridge;
use crate::constants::*;
use gb_rs_core::bytes::{bytes_to_word, word_to_bytes};
use gb_rs_core::config::CONFIGURATION;
use gb_rs_core::DeviceMode;

pub mod cartridge;
pub mod constants;

pub struct Memory {
    cartridge: Cartridge,
    vram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    io: Vec<u8>,
    hram: Vec<u8>,
    interrupt_enable: u8,
    vram_bank: usize,
    wram_bank: usize,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        let cartridge = Cartridge::new(rom).unwrap(); // TODO Replace this with real error handling
        let mode = if let Some(mode) = CONFIGURATION.mode.clone() {
            mode
        } else {
            cartridge.get_device_mode().into()
        };

        let (vram, wram) = match mode {
            DeviceMode::Color => (
                vec![0; VRAM_SIZE * 2],
                vec![0; RAM0_SIZE + RAM_BANK_SIZE * 7],
            ),
            DeviceMode::Classic => (vec![0; VRAM_SIZE], vec![0; RAM0_SIZE + RAM_BANK_SIZE]),
        };

        Self {
            cartridge,
            vram,
            wram,
            oam: vec![0; OAM_SIZE],
            io: vec![0; IO_SIZE],
            hram: vec![0; HRAM_SIZE],
            interrupt_enable: 0,
            vram_bank: 0,
            wram_bank: 1,
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                self.cartridge.rom_read(address)
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);

                self.vram[address]
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => self.cartridge.ram_read(address),
            RAM0_START..=RAM0_END => self.wram[address - RAM0_START],
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);

                self.wram[address]
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);

                self.wram[address]
            }
            OAM_START..=OAM_END => self.oam[address - OAM_START],
            IO_START..=IO_END => self.io[address - IO_START],
            HRAM_START..=HRAM_END => self.hram[address - HRAM_START],
            0xFFFF => self.interrupt_enable,
            _ => unreachable!(),
        }
    }

    pub fn read_word(&self, address: usize) -> u16 {
        bytes_to_word(self.read_byte(address + 1), self.read_byte(address))
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                self.cartridge.rom_write(address, value);
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);

                self.vram[address] = value;
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => self.cartridge.ram_write(address, value),
            RAM0_START..=RAM0_END => self.wram[address - ROM0_START] = value,
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);

                self.wram[address] = value;
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);

                self.wram[address] = value;
            }
            OAM_START..=OAM_END => self.oam[address - OAM_START] = value,
            IO_START..=IO_END => self.io[address - IO_START] = value,
            HRAM_START..=HRAM_END => self.hram[address - HRAM_START] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => unreachable!(),
        }
    }

    pub fn write_word(&mut self, address: usize, value: u16) {
        let (low, high) = word_to_bytes(value);

        self.write_byte(address + 1, low);
        self.write_byte(address, high);
    }
}
