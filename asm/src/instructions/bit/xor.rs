use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::bit as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("XOR {target}")]
pub struct BitwiseXor {
    pub target: BitwiseXorTarget,
}

impl BitwiseXor {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<BitwiseXorTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<BitwiseXor> for InstructionKind {
    fn from(value: BitwiseXor) -> Self {
        Self::Bit(super::Bit::Xor(value))
    }
}

impl Parse for BitwiseXor {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use BitwiseXorTarget::*;

        let target: op::BitwiseXorTarget = match self.target {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::BitwiseXor::create(target))
    }
}

impl const SetRegister for BitwiseXor {
    fn register(builder: &mut Builder) {
        use Register::*;

        // XOR r8
        builder.base(0xA8, Self::new(B), 1, 1);
        builder.base(0xA9, Self::new(C), 1, 1);
        builder.base(0xAA, Self::new(D), 1, 1);
        builder.base(0xAB, Self::new(E), 1, 1);
        builder.base(0xAC, Self::new(H), 1, 1);
        builder.base(0xAD, Self::new(L), 1, 1);
        builder.base(0xAF, Self::new(A), 1, 1);

        // Others
        builder.base(0xAE, Self::new(Pointer(Pair::HL)), 1, 2);
        builder.base(0xEE, Self::new(Data::new()), 2, 2);
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseXorTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

enum_from_helper!(
    const Register => BitwiseXorTarget::Register,
    const Pointer<Pair> => BitwiseXorTarget::PairPointer,
    const Data<u8> => BitwiseXorTarget::Data,
);
