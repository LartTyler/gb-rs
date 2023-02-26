use gb_rs_cpu::Cpu;
use gb_rs_memory::Memory;
use std::{fs::File, io::Read, path::Path};

pub struct Hardware {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl Hardware {
    pub fn from_file(cart_file: &Path) -> Result<Self, Error> {
        let mut file = File::open(cart_file)?;
        let len = file.metadata()?.len();
        let len: usize = len.try_into().map_err(|_| Error::FileTooBig)?;

        let mut rom: Vec<u8> = Vec::with_capacity(len);
        file.read_to_end(&mut rom)?;

        let memory = Memory::new(rom)?;

        Ok(Self {
            cpu: Cpu::new(memory.cartridge.device_mode.into()),
            memory,
        })
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("memory error: {0}")]
    Memory(#[from] gb_rs_memory::MemoryError),

    #[error("cart file size too big")]
    FileTooBig,
}
