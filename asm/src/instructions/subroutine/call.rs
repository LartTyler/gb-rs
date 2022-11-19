use super::Subroutine;
use crate::containers::{Condition, Data, Flag, Pointer};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::subroutine as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Call {
    pub target: Pointer<Data<u16>>,
    pub condition: Condition,
}

impl Call {
    pub const fn new(condition: Condition) -> Self {
        Self {
            target: Pointer(Data::new()),
            condition,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.condition != Condition::Always {
            write!(f, "CALL {}, {}", self.condition, self.target)
        } else {
            write!(f, "CALL {}", self.target)
        }
    }
}

impl const From<Call> for Instruction {
    fn from(value: Call) -> Self {
        Self::Subroutine(Subroutine::Call(value))
    }
}

impl const SetRegister for Call {
    fn register(builder: &mut Builder) {
        use Condition::*;
        use Flag::*;

        builder.base(0xC4, Self::new(Unset(Zero)));
        builder.base(0xCC, Self::new(Set(Zero)));
        builder.base(0xCD, Self::new(Always));
        builder.base(0xD4, Self::new(Unset(Carry)));
        builder.base(0xDC, Self::new(Set(Carry)));
    }
}

impl Parse for Call {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        Ok(op::Call::create(
            self.target.parse(data, offset)?,
            self.condition,
        ))
    }
}
