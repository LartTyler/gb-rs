use super::Increment;
use crate::containers::Register;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("INC {target}")]
pub struct RegisterIncrement {
    pub target: Register,
}

impl RegisterIncrement {
    pub const fn new(target: Register) -> Self {
        Self { target }
    }
}

impl Parse for RegisterIncrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for RegisterIncrement {
    fn register(builder: &mut Builder) {
        use Register::*;

        builder.base(0x04, Self::new(B));
        builder.base(0x0C, Self::new(C));
        builder.base(0x14, Self::new(D));
        builder.base(0x1C, Self::new(E));
        builder.base(0x24, Self::new(H));
        builder.base(0x2C, Self::new(L));
        builder.base(0x3C, Self::new(A));
    }
}

impl const From<RegisterIncrement> for Instruction {
    fn from(value: RegisterIncrement) -> Self {
        Self::Increment(Increment::Register(value))
    }
}

impl From<RegisterIncrement> for Operation {
    fn from(value: RegisterIncrement) -> Self {
        Self::Increment(Increment::Register(value))
    }
}
