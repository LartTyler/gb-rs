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
#[display("OR {target}")]
pub struct BitwiseOr {
    pub target: BitwiseOrTarget,
}

impl BitwiseOr {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<BitwiseOrTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<BitwiseOr> for Instruction {
    fn from(value: BitwiseOr) -> Self {
        Self::Bitwise(Bitwise::Or(value))
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

impl const SetRegister for BitwiseOr {
    fn register(builder: &mut Builder) {
        use Register::*;

        // OR r8
        builder.base(0xB0, Self::new(B));
        builder.base(0xB1, Self::new(C));
        builder.base(0xB2, Self::new(D));
        builder.base(0xB3, Self::new(E));
        builder.base(0xB4, Self::new(H));
        builder.base(0xB5, Self::new(L));
        builder.base(0xB7, Self::new(A));

        // Others
        builder.base(0xB6, Self::new(Pointer(Pair::HL)));
        builder.base(0xF6, Self::new(Data::new()));
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
    const Register => BitwiseOrTarget::Register,
    const Pointer<Pair> => BitwiseOrTarget::PairPointer,
    const Data<u8> => BitwiseOrTarget::Data,
);
