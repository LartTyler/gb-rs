use crate::containers::Register;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

use super::Decrement;

#[derive(Debug, Clone, Copy, Display)]
#[display("DEC {target}")]
pub struct RegisterDecrement {
    pub target: Register,
}

impl RegisterDecrement {
    pub const fn new(target: Register) -> Self {
        Self { target }
    }
}

impl Parse for RegisterDecrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for RegisterDecrement {
    fn register(builder: &mut Builder) {
        use Register::*;

        builder.base(0x05, Self::new(B));
        builder.base(0x0D, Self::new(C));
        builder.base(0x15, Self::new(D));
        builder.base(0x1D, Self::new(E));
        builder.base(0x25, Self::new(H));
        builder.base(0x2D, Self::new(L));
        builder.base(0x3D, Self::new(A));
    }
}

impl const From<RegisterDecrement> for Instruction {
    fn from(value: RegisterDecrement) -> Self {
        Self::Decrement(Decrement::Register(value))
    }
}

impl From<RegisterDecrement> for Operation {
    fn from(value: RegisterDecrement) -> Self {
        Self::Decrement(Decrement::Register(value))
    }
}
