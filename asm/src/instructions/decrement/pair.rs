use super::Decrement;
use crate::containers::Pair;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("DEC {target}")]
pub struct PairDecrement {
    pub target: Pair,
}

impl PairDecrement {
    pub const fn new(target: Pair) -> Self {
        Self { target }
    }
}

impl Parse for PairDecrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for PairDecrement {
    fn register(builder: &mut crate::sets::Builder) {
        use Pair::*;

        builder.base(0x0B, Self::new(BC));
        builder.base(0x1B, Self::new(DE));
        builder.base(0x2B, Self::new(HL));
        builder.base(0x3B, Self::new(SP));
    }
}

impl const From<PairDecrement> for Instruction {
    fn from(value: PairDecrement) -> Self {
        Self::Decrement(Decrement::Pair(value))
    }
}

impl From<PairDecrement> for Operation {
    fn from(value: PairDecrement) -> Self {
        Self::Decrement(Decrement::Pair(value))
    }
}
