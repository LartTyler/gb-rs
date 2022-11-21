use super::Increment;
use crate::containers::Pair;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("INC {target}")]
pub struct PairIncrement {
    pub target: Pair,
}

impl PairIncrement {
    pub const fn new(target: Pair) -> Self {
        Self { target }
    }
}

impl Parse for PairIncrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for PairIncrement {
    fn register(builder: &mut crate::sets::Builder) {
        use Pair::*;

        builder.base(0x03, Self::new(BC));
        builder.base(0x13, Self::new(DE));
        builder.base(0x23, Self::new(HL));
        builder.base(0x33, Self::new(SP));
    }
}

impl const From<PairIncrement> for Instruction {
    fn from(value: PairIncrement) -> Self {
        Self::Increment(Increment::Pair(value))
    }
}

impl From<PairIncrement> for Operation {
    fn from(value: PairIncrement) -> Self {
        Self::Increment(Increment::Pair(value))
    }
}
