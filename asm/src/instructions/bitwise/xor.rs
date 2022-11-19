use super::Bitwise;
use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::bitwise as op;
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

impl const From<BitwiseXor> for Instruction {
    fn from(value: BitwiseXor) -> Self {
        Self::Bitwise(Bitwise::Xor(value))
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
        builder.base(0xA8, Self::new(B));
        builder.base(0xA9, Self::new(C));
        builder.base(0xAA, Self::new(D));
        builder.base(0xAB, Self::new(E));
        builder.base(0xAC, Self::new(H));
        builder.base(0xAD, Self::new(L));
        builder.base(0xAF, Self::new(A));

        // Others
        builder.base(0xAE, Self::new(Pointer(Pair::HL)));
        builder.base(0xEE, Self::new(Data::new()));
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
