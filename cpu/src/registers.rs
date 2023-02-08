use gb_rs_asm::containers::{Flag, Pair, Register};
use gb_rs_core::{
    bytes::{bytes_to_word, word_to_bytes},
    MathResult,
};

/// Used to mask the flags register, since only the upper 4 bits of the register are used.
const FLAGS_MASK: u8 = 0xF0;

#[derive(Default, Debug)]
pub struct FlagsRegister(u8);

impl FlagsRegister {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has(&self, flag: Flag) -> bool {
        self.0 & (flag as u8) > 0
    }

    pub fn set(&mut self, flag: Flag) {
        self.0 |= flag as u8;
    }

    pub fn set_if(&mut self, flag: Flag, condition: bool) {
        if condition {
            self.set(flag);
        } else {
            self.unset(flag);
        }
    }

    pub fn unset(&mut self, flag: Flag) {
        self.0 &= !(flag as u8) & FLAGS_MASK;
    }

    pub fn toggle(&mut self, flag: Flag) {
        self.set_if(flag, !self.has(flag));
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn replace(&mut self, flags: u8) {
        self.0 = flags & FLAGS_MASK;
    }

    pub fn update_from_math_result<T>(
        &mut self,
        MathResult {
            half_carry, carry, ..
        }: &MathResult<T>,
    ) {
        if *half_carry {
            self.set(Flag::HalfCarry);
        } else {
            self.unset(Flag::HalfCarry);
        }

        if *carry {
            self.set(Flag::Carry);
        } else {
            self.unset(Flag::Carry);
        }
    }
}

#[derive(Debug, Default)]
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
    pub flags: FlagsRegister,
    pub stop_flag: bool,
}

impl Registers {
    pub fn get_byte(&self, reg: Register) -> u8 {
        use Register::*;

        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
        }
    }

    pub fn set_byte(&mut self, reg: Register, value: u8) {
        use Register::*;

        let reg = match reg {
            A => &mut self.a,
            B => &mut self.b,
            C => &mut self.c,
            D => &mut self.d,
            E => &mut self.e,
            H => &mut self.h,
            L => &mut self.l,
        };

        *reg = value;
    }

    pub fn get_pair(&self, reg: Pair) -> u16 {
        use Pair::*;

        match reg {
            BC => bytes_to_word(self.b, self.c),
            DE => bytes_to_word(self.d, self.e),
            HL => bytes_to_word(self.h, self.l),
            SP => self.stack_pointer,
        }
    }

    pub fn set_pair(&mut self, reg: Pair, value: u16) {
        let [high, low] = word_to_bytes(value);

        use Pair::*;

        match reg {
            BC => {
                self.b = high;
                self.c = low;
            }
            DE => {
                self.d = high;
                self.e = low;
            }
            HL => {
                self.h = high;
                self.l = low;
            }
            SP => self.stack_pointer = value,
        }
    }

    pub fn update_pc<U>(&mut self, update: U)
    where
        U: Into<UpdateKind>,
    {
        update.into().apply(&mut self.program_counter);
    }
}

pub enum UpdateKind {
    Forward(u8),
    Offset(i8),
    Set(u16),
}

impl UpdateKind {
    pub fn apply(self, pc: &mut u16) {
        match self {
            Self::Forward(n) => *pc = pc.wrapping_add(n.into()),
            Self::Offset(n) => *pc = pc.wrapping_add_signed(n.into()),
            Self::Set(n) => *pc = n,
        }
    }
}

impl From<u8> for UpdateKind {
    fn from(value: u8) -> Self {
        Self::Forward(value)
    }
}

impl From<i8> for UpdateKind {
    fn from(value: i8) -> Self {
        Self::Offset(value)
    }
}

impl From<u16> for UpdateKind {
    fn from(value: u16) -> Self {
        Self::Set(value)
    }
}
