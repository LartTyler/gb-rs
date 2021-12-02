use gb_rs_core::bytes::{bytes_to_word, word_to_bytes};

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub flags: u8,
    pub stop_flag: bool,
}

#[derive(Copy, Clone)]
pub enum Flag {
    Zero = 0x80,
    Subtract = 0x40,
    HalfCarry = 0x20,
    Carry = 0x10,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ByteRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum PairRegister {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Registers {
    pub fn get_byte(&self, reg: ByteRegister) -> u8 {
        match reg {
            ByteRegister::A => self.a,
            ByteRegister::B => self.b,
            ByteRegister::C => self.c,
            ByteRegister::D => self.d,
            ByteRegister::E => self.e,
            ByteRegister::H => self.h,
            ByteRegister::L => self.l,
        }
    }

    pub fn set_byte(&mut self, reg: ByteRegister, value: u8) {
        let reg = match reg {
            ByteRegister::A => &mut self.a,
            ByteRegister::B => &mut self.b,
            ByteRegister::C => &mut self.c,
            ByteRegister::D => &mut self.d,
            ByteRegister::E => &mut self.e,
            ByteRegister::H => &mut self.h,
            ByteRegister::L => &mut self.l,
        };

        *reg = value;
    }

    pub fn get_pair(&self, reg: PairRegister) -> u16 {
        match reg {
            PairRegister::AF => bytes_to_word(self.flags, self.a),
            PairRegister::BC => bytes_to_word(self.c, self.b),
            PairRegister::DE => bytes_to_word(self.e, self.d),
            PairRegister::HL => bytes_to_word(self.l, self.h),
            PairRegister::SP => self.stack_pointer,
        }
    }

    pub fn set_pair(&mut self, reg: PairRegister, value: u16) {
        let (low, high) = word_to_bytes(value);
        match reg {
            PairRegister::AF => {
                self.flags = low;
                self.a = high;
            }
            PairRegister::BC => {
                self.c = low;
                self.b = high;
            }
            PairRegister::DE => {
                self.e = low;
                self.d = high;
            }
            PairRegister::HL => {
                self.l = low;
                self.h = high;
            }
            PairRegister::SP => self.stack_pointer = value,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.flags & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }

    pub fn is_flag_set(&self, flag: Flag) -> bool {
        self.flags & (flag as u8) > 0
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.set_flag(flag, false);
    }

    pub fn clear_flags(&mut self) {
        self.flags = 0;
    }
}
