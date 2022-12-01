use super::Bit;
use crate::containers::{Flag, Pair, Pointer, Register, Value};
use crate::operations::OperationKind;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BitwiseComplement {
    pub target: BitwiseComplementTarget,
    pub short: bool,
}

impl BitwiseComplement {
    pub fn create<T>(target: T, short: bool) -> OperationKind
    where
        T: Into<BitwiseComplementTarget>,
    {
        OperationKind::Bit(Bit::Complement(Self {
            target: target.into(),
            short,
        }))
    }
}

impl Display for BitwiseComplement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BitwiseComplementTarget::*;

        if self.short {
            match self.target {
                Register(_) => write!(f, "CPL"),
                Flag(_) => write!(f, "CCF"),
                _ => panic!("cannot represent {:?} as short operation", self),
            }
        } else {
            write!(f, "CP {}", self.target)
        }
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum BitwiseComplementTarget {
    Register(Register),
    Flag(Flag),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

impl From<Register> for BitwiseComplementTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Flag> for BitwiseComplementTarget {
    fn from(value: Flag) -> Self {
        Self::Flag(value)
    }
}

impl From<Pointer<Pair>> for BitwiseComplementTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}

impl From<Value<u8>> for BitwiseComplementTarget {
    fn from(value: Value<u8>) -> Self {
        Self::Data(value)
    }
}
