use crate::containers::{Data, Flag, Pair, Pointer, Register};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::bit as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct BitwiseComplement {
    pub target: BitwiseComplementTarget,
    pub short: bool,
}

impl BitwiseComplement {
    pub const fn new<T>(target: T, short: bool) -> Self
    where
        T: ~const Into<BitwiseComplementTarget>,
    {
        Self {
            target: target.into(),
            short,
        }
    }
}

impl Parse for BitwiseComplement {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use BitwiseComplementTarget::*;

        let target: op::BitwiseComplementTarget = match self.target {
            Register(r) => r.into(),
            Flag(f) => f.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::BitwiseComplement::create(target, self.short))
    }
}

impl const SetRegister for BitwiseComplement {
    fn register(builder: &mut Builder) {
        use Flag::*;
        use Register::*;

        // CP r8
        builder.base(0xB8, Self::new(B, false), 1, 1);
        builder.base(0xB9, Self::new(C, false), 1, 1);
        builder.base(0xBA, Self::new(D, false), 1, 1);
        builder.base(0xBB, Self::new(E, false), 1, 1);
        builder.base(0xBC, Self::new(H, false), 1, 1);
        builder.base(0xBD, Self::new(L, false), 1, 1);
        builder.base(0xBF, Self::new(A, false), 1, 1);

        // Others
        builder.base(0x2F, Self::new(A, true), 1, 1);
        builder.base(0x3F, Self::new(Carry, true), 1, 1);
        builder.base(0xBE, Self::new(Pointer(Pair::HL), false), 1, 2);
        builder.base(0xFE, Self::new(Data::new(), false), 2, 2);
    }
}

impl Display for BitwiseComplement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BitwiseComplementTarget::*;

        if self.short {
            match self.target {
                Register(_) => write!(f, "CPL"),
                Flag(_) => write!(f, "CCF"),
                _ => panic!("cannot represent {:?} as short instruction", self),
            }
        } else {
            write!(f, "CP {}", self.target)
        }
    }
}

impl const From<BitwiseComplement> for InstructionKind {
    fn from(value: BitwiseComplement) -> Self {
        Self::Bit(super::Bit::Complement(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseComplementTarget {
    Register(Register),
    Flag(Flag),
    Data(Data<u8>),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for BitwiseComplementTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Flag> for BitwiseComplementTarget {
    fn from(value: Flag) -> Self {
        Self::Flag(value)
    }
}

impl const From<Data<u8>> for BitwiseComplementTarget {
    fn from(value: Data<u8>) -> Self {
        Self::Data(value)
    }
}

impl const From<Pointer<Pair>> for BitwiseComplementTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
