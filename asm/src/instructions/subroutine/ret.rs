use super::Subroutine;
use crate::containers::{Condition, Flag};
use crate::instructions::{Instruction, SetRegister};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Return {
    pub condition: Condition,
    pub enable_interrupt: bool,
}

impl Return {
    pub const fn new(condition: Condition, enable_interrupt: bool) -> Self {
        Self {
            condition,
            enable_interrupt,
        }
    }
}

impl const From<Return> for Instruction {
    fn from(value: Return) -> Self {
        Self::Subroutine(Subroutine::Return(value))
    }
}

impl const SetRegister for Return {
    fn register(builder: &mut Builder) {
        use Condition::*;
        use Flag::*;

        builder.base(0xC0, Self::new(Unset(Zero), false));
        builder.base(0xC8, Self::new(Set(Zero), false));
        builder.base(0xC9, Self::new(Always, false));
        builder.base(0xD0, Self::new(Unset(Carry), false));
        builder.base(0xD8, Self::new(Set(Carry), false));
        builder.base(0xD9, Self::new(Always, true));
    }
}

impl Parse for Return {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.condition != Condition::Always {
            write!(f, "RET {}", self.condition)
        } else {
            let interrupt = if self.enable_interrupt { "I" } else { "" };
            write!(f, "RET{interrupt}")
        }
    }
}
