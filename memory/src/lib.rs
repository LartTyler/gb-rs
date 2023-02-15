use crate::cartridge::{Cartridge, CartridgeError};
use crate::constants::*;
use gb_rs_asm::read::Read;
use gb_rs_core::bytes::{bytes_to_word, word_to_bytes};
use gb_rs_core::DeviceMode;

pub mod cartridge;
pub mod constants;

pub struct Memory {
    pub cartridge: Cartridge,
    vram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    io: Vec<u8>,
    hram: Vec<u8>,
    interrupt_flags: u8,
    interrupt_enable: u8,
    vram_bank: usize,
    wram_bank: usize,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> MemoryResult {
        let cartridge = Cartridge::new(rom)?;
        let mode = cartridge.get_device_mode().into();

        let (vram, wram) = match mode {
            DeviceMode::Color => (
                vec![0; VRAM_SIZE * 2],
                vec![0; RAM0_SIZE + RAM_BANK_SIZE * 7],
            ),
            DeviceMode::Classic => (vec![0; VRAM_SIZE], vec![0; RAM0_SIZE + RAM_BANK_SIZE]),
        };

        Ok(Self {
            cartridge,
            vram,
            wram,
            oam: vec![0; OAM_SIZE],
            io: vec![0; IO_SIZE],
            hram: vec![0; HRAM_SIZE],
            interrupt_flags: 0,
            interrupt_enable: 0,
            vram_bank: 0,
            wram_bank: 1,
        })
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                return self.cartridge.rom_read(address)
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);
                self.vram.get(address)
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => return self.cartridge.ram_read(address),
            RAM0_START..=RAM0_END => self.wram.get(address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);
                self.wram.get(address)
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);
                self.wram.get(address)
            }
            OAM_START..=OAM_END => self.oam.get(address - OAM_START),
            INTERRUPT_FLAGS => Some(&self.interrupt_flags),
            IO_START..=IO_END => self.io.get(address - IO_START),
            HRAM_START..=HRAM_END => self.hram.get(address - HRAM_START),
            INTERRUPT_ENABLE => Some(&self.interrupt_enable),
            _ => unreachable!(),
        };

        *slot.unwrap_or(&0xFF)
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);

        bytes_to_word(high, low)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                self.cartridge.rom_write(address, value);

                return;
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);
                self.vram.get_mut(address)
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.cartridge.ram_write(address, value);

                return;
            }
            RAM0_START..=RAM0_END => self.wram.get_mut(address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);
                self.wram.get_mut(address)
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);
                self.wram.get_mut(address)
            }
            OAM_START..=OAM_END => self.oam.get_mut(address - OAM_START),
            INTERRUPT_FLAGS => Some(&mut self.interrupt_flags),
            IO_START..=IO_END => self.io.get_mut(address - IO_START),
            HRAM_START..=HRAM_END => self.hram.get_mut(address - HRAM_START),
            INTERRUPT_ENABLE => Some(&mut self.interrupt_enable),
            _ => unreachable!(),
        };

        if let Some(slot) = slot {
            *slot = value;
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [high, low] = word_to_bytes(value);

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }
}

impl Read for Memory {
    fn read_byte(&self, offset: u16) -> gb_rs_asm::read::Result<u8> {
        Ok(Memory::read_byte(self, offset))
    }
}

#[derive(Debug)]
pub enum MemoryError {
    CartridgeError(CartridgeError),
}

pub type MemoryResult = Result<Memory, MemoryError>;

impl From<CartridgeError> for MemoryError {
    fn from(e: CartridgeError) -> Self {
        MemoryError::CartridgeError(e)
    }
}
