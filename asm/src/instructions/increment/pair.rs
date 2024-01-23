use super::Increment;
use crate::containers::Pair;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("INC {target}")]
pub struct PairIncrement {
    pub target: Pair,
}

impl PairIncrement {
    pub fn new(target: Pair) -> Self {
        Self { target }
    }
}

impl Parse for PairIncrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for PairIncrement {
    fn register(builder: &mut crate::sets::Builder) {
        use Pair::*;

        builder.base(0x03, Self::new(BC), 1, 2);
        builder.base(0x13, Self::new(DE), 1, 2);
        builder.base(0x23, Self::new(HL), 1, 2);
        builder.base(0x33, Self::new(SP), 1, 2);
    }
}

impl From<PairIncrement> for InstructionKind {
    fn from(value: PairIncrement) -> Self {
        Self::Increment(Increment::Pair(value))
    }
}

impl From<PairIncrement> for OperationKind {
    fn from(value: PairIncrement) -> Self {
        Self::Increment(Increment::Pair(value))
    }
}
