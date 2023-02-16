use gb_rs_cpu::Cpu;
use gb_rs_memory::{Memory, MemoryError};
use std::{fs::File, io::Read, path::Path};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("memory error: {0}")]
    Memory(#[from] MemoryError),

    #[error("file size too big")]
    FileTooBig,
}

pub struct Hardware {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl Hardware {
    pub fn new(rom: Vec<u8>) -> Result<Self> {
        let memory = Memory::new(rom)?;

        Ok(Self {
            cpu: Cpu::new(memory.cartridge.device_mode.into()),
            memory,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;

        let len = file.metadata()?.len();
        let len: usize = len.try_into().map_err(|_| Error::FileTooBig)?;

        let mut data: Vec<u8> = Vec::with_capacity(len);
        file.read_to_end(&mut data)?;

        Self::new(data)
    }
}
