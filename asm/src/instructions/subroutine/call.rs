use super::Subroutine;
use crate::containers::{Condition, Data, Flag, Pointer};
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::subroutine as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Call {
    pub target: CallTarget,
    pub condition: Condition,
}

impl Call {
    pub const fn new<T>(target: T, condition: Condition) -> Self
    where
        T: ~const Into<CallTarget>,
    {
        Self {
            target: target.into(),
            condition,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let CallTarget::Vector(p) = self.target {
            write!(f, "RST {p}")
        } else if self.condition != Condition::Always {
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

        // CALL instructions
        builder.base(0xC4, Self::new(Pointer(Data::new()), Unset(Zero)));
        builder.base(0xCC, Self::new(Pointer(Data::new()), Set(Zero)));
        builder.base(0xCD, Self::new(Pointer(Data::new()), Always));
        builder.base(0xD4, Self::new(Pointer(Data::new()), Unset(Carry)));
        builder.base(0xDC, Self::new(Pointer(Data::new()), Set(Carry)));

        // RST instructions
        builder.base(0xC7, Self::new(Pointer(0x00), Always));
        builder.base(0xCF, Self::new(Pointer(0x08), Always));
        builder.base(0xD7, Self::new(Pointer(0x10), Always));
        builder.base(0xDF, Self::new(Pointer(0x18), Always));
        builder.base(0xE7, Self::new(Pointer(0x20), Always));
        builder.base(0xEF, Self::new(Pointer(0x28), Always));
        builder.base(0xF7, Self::new(Pointer(0x30), Always));
        builder.base(0xFF, Self::new(Pointer(0x38), Always));
    }
}

impl Parse for Call {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use CallTarget::*;

        let target: op::CallTarget = match self.target {
            DataPointer(p) => p.parse(data, offset)?.into(),
            Vector(p) => p.into(),
        };

        Ok(op::Call::create(target, self.condition))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CallTarget {
    DataPointer(Pointer<Data<u16>>),
    Vector(Pointer<u16>),
}

enum_from_helper!(
    const Pointer<Data<u16>> => CallTarget::DataPointer,
    const Pointer<u16> => CallTarget::Vector,
);
