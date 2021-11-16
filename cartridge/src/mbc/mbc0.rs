use crate::mbc::MemoryBankController;

pub struct Mbc0 {
    rom: Vec<u8>,
}

impl Mbc0 {
    pub fn new(rom: Vec<u8>) -> Self {
        Mbc0 { rom }
    }
}

impl MemoryBankController for Mbc0 {
    fn rom_read(&self, address: usize) -> u8 {
        *self.rom.get(address).unwrap_or(&0xFF)
    }

    fn rom_write(&mut self, _address: usize, _value: u8) {}

    fn ram_read(&self, _address: usize) -> u8 {
        0xFF
    }

    fn ram_write(&mut self, _address: usize, _value: u8) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write() {
        let mut controller = Mbc0::new(vec![1, 2, 3]);

        assert_eq!(controller.rom_read(0), 1);
        assert_eq!(controller.rom_read(1), 2);

        controller.rom_write(0, 10);
        assert_eq!(controller.rom_read(0), 1);

        assert_eq!(controller.ram_read(0), 0xFF);
        assert_eq!(controller.ram_read(1), 0xFF);

        controller.ram_write(0, 10);
        assert_eq!(controller.ram_read(0), 0xFF);
    }
}
