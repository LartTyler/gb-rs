use crate::cartridge::mbc::MemoryBankController;
use crate::constants::EXTERNAL_RAM_SIZE;
use gb_rs_common::bytes::bytes_to_word;
use gb_rs_common::DeviceMode;

pub mod constants;
pub mod mbc;

#[derive(Debug, Copy, Clone)]
pub enum SupportedDeviceMode {
    Color,
    Classic,
    Any,
}

impl From<SupportedDeviceMode> for DeviceMode {
    fn from(value: SupportedDeviceMode) -> Self {
        use SupportedDeviceMode::*;

        match value {
            Classic => Self::Classic,
            Color | Any => Self::Color,
        }
    }
}

pub struct Cartridge {
    pub title: String,
    pub device_mode: SupportedDeviceMode,
    pub licensee_id: u16,
    pub sgb_support: bool,
    pub version: u8,
    pub controller: Box<dyn MemoryBankController>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> CartridgeResult {
        Ok(Self {
            title: get_title(&rom),
            device_mode: get_device_mode(&rom),
            licensee_id: get_licensee_id(&rom),
            sgb_support: get_sgb_support(&rom),
            version: get_version(&rom),
            controller: mbc::ControllerType::create_for_rom(rom)?,
        })
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_device_mode(&self) -> SupportedDeviceMode {
        self.device_mode
    }

    pub fn get_licensee_id(&self) -> u16 {
        self.licensee_id
    }

    pub fn get_sgb_support(&self) -> bool {
        self.sgb_support
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn rom_read(&self, address: usize) -> u8 {
        self.controller.rom_read(address)
    }

    pub fn rom_write(&mut self, address: usize, value: u8) {
        self.controller.rom_write(address, value);
    }

    pub fn ram_read(&self, address: usize) -> u8 {
        self.controller.ram_read(address)
    }

    pub fn ram_write(&mut self, address: usize, value: u8) {
        self.controller.ram_write(address, value);
    }
}

pub type CartridgeResult = Result<Cartridge, CartridgeError>;

#[derive(Debug, thiserror::Error)]
pub enum CartridgeError {
    #[error("controller error: {0}")]
    ControllerError(#[from] mbc::CreateError),
}

/// Returns the maximum possible title length, based on the value of the
/// [`constants::GBC_SUPPORT_TYPE`].
///
/// On older ROMs (specifically, ROMs created for hardware older than the Gameboy Color) could
/// include titles up to 16 ASCII characters long. For the Gameboy Color and on, the title length
/// was reduced to 11 characters to make room for two additional header fields: manufacturer
/// code and [support type].
///
/// [support type]: constants::GBC_SUPPORT_TYPE
pub fn get_title_max_len(rom: &[u8]) -> usize {
    if rom[constants::GBC_SUPPORT_TYPE] & 0x80 != 0 {
        11
    } else {
        16
    }
}

/// Retrieves the ROM's title.
///
/// A title begins at [`constants::TITLE_START`], and may be up to either 11 or 16 ASII characters
/// long, depending on the return value of [`get_title_max_len()`].
///
/// Titles are padded with null bytes if the text is fewer than the maximum length. The return
/// value of this function _does not_ include any null padding.
pub fn get_title(rom: &[u8]) -> String {
    let len = get_title_max_len(rom);
    let mut title = String::with_capacity(len);

    for i in 0..len {
        match rom[constants::TITLE_START + i] {
            0 => break,
            c => title.push(c as char),
        }
    }

    title
}

/// Retrieves which hardware the ROM supports.
///
/// The support mode is determined by examing the byte at [`constants::GBC_SUPPORT_TYPE`]:
/// - A value of `0x80` means that the ROM supports both Gameboy Classic hardware _and_ Gameboy
/// Color hardware.
/// - A value of `0xC0` means that the ROM _only_ supports Gameboy Color hardware.
/// - Any other value means that the ROM was _intended_ to only support Classic hardware.
///
/// Emphasis is placed on "intended" because the Gameboy Color is both capable of and happy to run
/// Gameboy Classic games without issue. There are some differences in how the contents of the ROM
/// are interpreted (e.g. title length, color palettes, etc), but that appears to be about it.
pub fn get_device_mode(rom: &[u8]) -> SupportedDeviceMode {
    match rom[constants::GBC_SUPPORT_TYPE] {
        0x80 => SupportedDeviceMode::Any,
        0xC0 => SupportedDeviceMode::Color,
        _ => SupportedDeviceMode::Classic,
    }
}

/// Retrieves the licensee ID.
///
/// The Licensee ID is used to determine which company created the ROM. Translating licensee IDs to
/// company names is (at the moment) beyond the scope of this project, but a table of ID to name
/// mappings can be found
/// [here](https://gbdev.io/pandocs/The_Cartridge_Header.html#0144-0145---new-licensee-code) and
/// [here](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt).
pub fn get_licensee_id(rom: &[u8]) -> u16 {
    match rom[constants::OLD_LICENSEE] {
        0x33 => bytes_to_word(
            rom[constants::NEW_LICENSEE_HIGH],
            rom[constants::NEW_LICENSEE_LOW],
        ),
        x => x as u16,
    }
}

/// Retrieves the status of Super Gameboy feature support.
pub fn get_sgb_support(rom: &[u8]) -> bool {
    rom[constants::SGB_SUPPORT_FLAG] == 0x03
}

/// Retrieves the ROM's version.
///
/// For most games, this is usually `0x00`.
pub fn get_version(rom: &[u8]) -> u8 {
    rom[constants::VERSION]
}

/// Retrieves the size of the cartridge RAM.
///
/// A map of RAM sizes can be found
/// [here](https://gbdev.io/pandocs/The_Cartridge_Header.html#0149---ram-size).
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
