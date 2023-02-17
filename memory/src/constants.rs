pub const ROM0_START: usize = 0x0000;
pub const ROM0_END: usize = 0x3FFF;

pub const ROM_BANK_START: usize = 0x4000;
pub const ROM_BANK_END: usize = 0x7FFF;
pub const ROM_BANK_SIZE: usize = ROM_BANK_END - ROM_BANK_START + 1;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

pub const EXTERNAL_RAM_START: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_START + 1;

pub const RAM0_START: usize = 0xC000;
pub const RAM0_END: usize = 0xCFFF;
pub const RAM0_SIZE: usize = RAM0_END - RAM0_START + 1;

pub const RAM_BANK_START: usize = 0xD000;
pub const RAM_BANK_END: usize = 0xDFFF;
pub const RAM_BANK_SIZE: usize = RAM_BANK_END - RAM_BANK_START + 1;

pub const ECHO_START: usize = 0xE000;
pub const ECHO_END: usize = 0xFDFF;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub const INTERRUPT_FLAGS: usize = 0xFF0F;

pub const IO_START: usize = 0xFF00;
pub const IO_END: usize = 0xFF7F;
pub const IO_SIZE: usize = IO_END - IO_START + 1;

pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFFE;
pub const HRAM_SIZE: usize = HRAM_END - HRAM_START + 1;

pub const INTERRUPT_ENABLE: usize = 0xFFFF;
