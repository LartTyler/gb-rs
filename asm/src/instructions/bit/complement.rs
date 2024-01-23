use crate::instructions::{InstructionKind, SetRegister};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("C{target}")]
pub struct Complement {
    pub target: ComplementTarget,
}

impl Complement {
    pub fn new(target: ComplementTarget) -> Self {
        Self { target }
    }
}

impl From<Complement> for InstructionKind {
    fn from(value: Complement) -> Self {
        Self::Bit(super::Bit::Complement(value))
    }
}

impl SetRegister for Complement {
    fn register(builder: &mut Builder) {
        use ComplementTarget::*;

        // CPL
        builder.base(0x2F, Self::new(Accumulator), 1, 1);

        // CCF
        builder.base(0x3F, Self::new(Carry), 1, 1);
    }
}

impl Parse for Complement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ComplementTarget {
    #[display("PL")]
    Accumulator,

    #[display("CF")]
    Carry,
}
