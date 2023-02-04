use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Read {
    fn read_byte(&self, offset: u16) -> Result<u8>;

    fn read_word(&self, offset: u16) -> Result<u16> {
        let low = self.read_byte(offset)?;
        let high = self.read_byte(offset + 1)?;

        Ok(u16::from_be_bytes([high, low]))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not read byte at {0:#02X}")]
    OutOfBounds(u16),
}

impl Read for [u8] {
    fn read_byte(&self, offset: u16) -> Result<u8> {
        self.get(offset as usize)
            .copied()
            .ok_or(Error::OutOfBounds(offset))
    }
}

impl Read for Vec<u8> {
    fn read_byte(&self, offset: u16) -> Result<u8> {
        self.get(offset as usize)
            .copied()
            .ok_or(Error::OutOfBounds(offset))
    }
}
