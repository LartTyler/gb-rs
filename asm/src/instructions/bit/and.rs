use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::bit as op;
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

impl const From<BitwiseAnd> for InstructionKind {
    fn from(value: BitwiseAnd) -> Self {
        Self::Bit(super::Bit::And(value))
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
        builder.base(0xA0, Self::new(B), 1, 1);
        builder.base(0xA1, Self::new(C), 1, 1);
        builder.base(0xA2, Self::new(D), 1, 1);
        builder.base(0xA3, Self::new(E), 1, 1);
        builder.base(0xA4, Self::new(H), 1, 1);
        builder.base(0xA5, Self::new(L), 1, 1);
        builder.base(0xA7, Self::new(A), 1, 1);

        // Others
        builder.base(0xA6, Self::new(Pointer(Pair::HL)), 1, 2);
        builder.base(0xE6, Self::new(Data::new()), 2, 2);
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
