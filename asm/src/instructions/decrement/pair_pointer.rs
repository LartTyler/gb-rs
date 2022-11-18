use super::Decrement;
use crate::containers::{Pair, Pointer};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("DEC {target}")]
pub struct PairPointerDecrement {
    pub target: Pointer<Pair>,
}

impl PairPointerDecrement {
    pub const fn new(target: Pointer<Pair>) -> Self {
        Self { target }
    }
}

impl Parse for PairPointerDecrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for PairPointerDecrement {
    fn register(builder: &mut Builder) {
        builder.base(0x35, Self::new(Pointer(Pair::HL)));
    }
}

impl const From<PairPointerDecrement> for Instruction {
    fn from(value: PairPointerDecrement) -> Self {
        Self::Decrement(Decrement::PairPointer(value))
    }
}

impl From<PairPointerDecrement> for Operation {
    fn from(value: PairPointerDecrement) -> Self {
        Self::Decrement(Decrement::PairPointer(value))
    }
}
