use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Read {
    fn read_byte(&self, offset: u16) -> Result<u8>;

    fn read_word(&self, offset: u16) -> Result<u16> {
        let high = self.read_byte(offset)? as u16;
        let low = self.read_byte(offset + 1)? as u16;

        Ok((high << 8) | low)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not read byte at {0:#02X}")]
    OutOfBounds(u16),
}
