use crate::containers::Register;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
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

        builder.base(0x05, Self::new(B), 1, 1);
        builder.base(0x0D, Self::new(C), 1, 1);
        builder.base(0x15, Self::new(D), 1, 1);
        builder.base(0x1D, Self::new(E), 1, 1);
        builder.base(0x25, Self::new(H), 1, 1);
        builder.base(0x2D, Self::new(L), 1, 1);
        builder.base(0x3D, Self::new(A), 1, 1);
    }
}

impl const From<RegisterDecrement> for InstructionKind {
    fn from(value: RegisterDecrement) -> Self {
        Self::Decrement(Decrement::Register(value))
    }
}

impl From<RegisterDecrement> for OperationKind {
    fn from(value: RegisterDecrement) -> Self {
        Self::Decrement(Decrement::Register(value))
    }
}
