use super::Decrement;
use crate::containers::{Pair, Pointer};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
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
    pub fn new(target: Pointer<Pair>) -> Self {
        Self { target }
    }
}

impl Parse for PairPointerDecrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for PairPointerDecrement {
    fn register(builder: &mut Builder) {
        builder.base(0x35, Self::new(Pointer(Pair::HL)), 1, 3);
    }
}

impl From<PairPointerDecrement> for InstructionKind {
    fn from(value: PairPointerDecrement) -> Self {
        Self::Decrement(Decrement::PairPointer(value))
    }
}

impl From<PairPointerDecrement> for OperationKind {
    fn from(value: PairPointerDecrement) -> Self {
        Self::Decrement(Decrement::PairPointer(value))
    }
}
