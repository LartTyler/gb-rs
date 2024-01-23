use super::Increment;
use crate::containers::{Pair, Pointer};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::{read::Read, sets::Builder};
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("INC {target}")]
pub struct PairPointerIncrement {
    pub target: Pointer<Pair>,
}

impl PairPointerIncrement {
    pub fn new(target: Pointer<Pair>) -> Self {
        Self { target }
    }
}

impl Parse for PairPointerIncrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for PairPointerIncrement {
    fn register(builder: &mut Builder) {
        builder.base(0x34, Self::new(Pointer(Pair::BC)), 1, 3);
    }
}

impl From<PairPointerIncrement> for InstructionKind {
    fn from(value: PairPointerIncrement) -> Self {
        Self::Increment(Increment::PairPointer(value))
    }
}

impl From<PairPointerIncrement> for OperationKind {
    fn from(value: PairPointerIncrement) -> Self {
        Self::Increment(Increment::PairPointer(value))
    }
}
