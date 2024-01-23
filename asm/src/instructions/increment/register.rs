use super::Increment;
use crate::containers::Register;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
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
    pub fn new(target: Register) -> Self {
        Self { target }
    }
}

impl Parse for RegisterIncrement {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for RegisterIncrement {
    fn register(builder: &mut Builder) {
        use Register::*;

        builder.base(0x04, Self::new(B), 1, 1);
        builder.base(0x0C, Self::new(C), 1, 1);
        builder.base(0x14, Self::new(D), 1, 1);
        builder.base(0x1C, Self::new(E), 1, 1);
        builder.base(0x24, Self::new(H), 1, 1);
        builder.base(0x2C, Self::new(L), 1, 1);
        builder.base(0x3C, Self::new(A), 1, 1);
    }
}

impl From<RegisterIncrement> for InstructionKind {
    fn from(value: RegisterIncrement) -> Self {
        Self::Increment(Increment::Register(value))
    }
}

impl From<RegisterIncrement> for OperationKind {
    fn from(value: RegisterIncrement) -> Self {
        Self::Increment(Increment::Register(value))
    }
}
