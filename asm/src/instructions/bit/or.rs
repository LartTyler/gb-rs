use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::bit as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("OR {target}")]
pub struct BitwiseOr {
    pub target: BitwiseOrTarget,
}

impl BitwiseOr {
    pub fn new<T>(target: T) -> Self
    where
        T: Into<BitwiseOrTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl From<BitwiseOr> for InstructionKind {
    fn from(value: BitwiseOr) -> Self {
        Self::Bit(super::Bit::Or(value))
    }
}

impl Parse for BitwiseOr {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use BitwiseOrTarget::*;

        let target: op::BitwiseOrTarget = match self.target {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::BitwiseOr::create(target))
    }
}

impl SetRegister for BitwiseOr {
    fn register(builder: &mut Builder) {
        use Register::*;

        // OR r8
        builder.base(0xB0, Self::new(B), 1, 1);
        builder.base(0xB1, Self::new(C), 1, 1);
        builder.base(0xB2, Self::new(D), 1, 1);
        builder.base(0xB3, Self::new(E), 1, 1);
        builder.base(0xB4, Self::new(H), 1, 1);
        builder.base(0xB5, Self::new(L), 1, 1);
        builder.base(0xB7, Self::new(A), 1, 1);

        // Others
        builder.base(0xB6, Self::new(Pointer(Pair::HL)), 1, 2);
        builder.base(0xF6, Self::new(Data::new()), 2, 2);
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseOrTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

enum_from_helper!(
    Register => BitwiseOrTarget::Register,
    Pointer<Pair> => BitwiseOrTarget::PairPointer,
    Data<u8> => BitwiseOrTarget::Data,
);
