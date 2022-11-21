use crate::containers::{Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bit as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("SWAP {target}")]
pub struct Swap {
    pub target: SwapTarget,
}

impl Swap {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<SwapTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<Swap> for Instruction {
    fn from(value: Swap) -> Self {
        Self::Bit(super::Bit::Swap(value))
    }
}

impl From<Swap> for Operation {
    fn from(value: Swap) -> Self {
        Self::Bit(op::Bit::Swap(value))
    }
}

impl const SetRegister for Swap {
    fn register(builder: &mut Builder) {
        use Register::*;

        builder.extended(0x30, Self::new(B));
        builder.extended(0x31, Self::new(C));
        builder.extended(0x32, Self::new(D));
        builder.extended(0x33, Self::new(E));
        builder.extended(0x34, Self::new(H));
        builder.extended(0x35, Self::new(L));
        builder.extended(0x36, Self::new(Pointer(Pair::HL)));
        builder.extended(0x37, Self::new(A));
    }
}

impl Parse for Swap {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum SwapTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

enum_from_helper!(
    const Register => SwapTarget::Register,
    const Pointer<Pair> => SwapTarget::PairPointer,
);
