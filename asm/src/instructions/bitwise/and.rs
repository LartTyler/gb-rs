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
#[display("AND {target}")]
pub struct BitwiseAnd {
    pub target: BitwiseAndTarget,
}

impl BitwiseAnd {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<BitwiseAndTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<BitwiseAnd> for Instruction {
    fn from(value: BitwiseAnd) -> Self {
        Self::Bitwise(Bitwise::And(value))
    }
}

impl Parse for BitwiseAnd {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use BitwiseAndTarget::*;

        let target: op::BitwiseAndTarget = match self.target {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::BitwiseAnd::create(target))
    }
}

impl const SetRegister for BitwiseAnd {
    fn register(builder: &mut Builder) {
        use Register::*;

        // AND r8
        builder.base(0xA0, Self::new(B));
        builder.base(0xA1, Self::new(C));
        builder.base(0xA2, Self::new(D));
        builder.base(0xA3, Self::new(E));
        builder.base(0xA4, Self::new(H));
        builder.base(0xA5, Self::new(L));
        builder.base(0xA7, Self::new(A));

        // Others
        builder.base(0xA6, Self::new(Pointer(Pair::HL)));
        builder.base(0xE6, Self::new(Data::new()));
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseAndTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

enum_from_helper!(
    const Register => BitwiseAndTarget::Register,
    const Pointer<Pair> => BitwiseAndTarget::PairPointer,
    const Data<u8> => BitwiseAndTarget::Data,
);
