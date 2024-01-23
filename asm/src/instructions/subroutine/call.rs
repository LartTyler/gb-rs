use super::Subroutine;
use crate::containers::{Condition, Data, Flag, Pointer};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
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
    pub fn new<T>(target: T, condition: Condition) -> Self
    where
        T: Into<CallTarget>,
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

impl From<Call> for InstructionKind {
    fn from(value: Call) -> Self {
        Self::Subroutine(Subroutine::Call(value))
    }
}

impl SetRegister for Call {
    fn register(builder: &mut Builder) {
        use Condition::*;
        use Flag::*;

        // CALL instructions
        builder.base(
            0xC4,
            Self::new(Pointer(Data::new()), Unset(Zero)),
            3,
            (3, 6),
        );

        builder.base(0xCC, Self::new(Pointer(Data::new()), Set(Zero)), 3, (3, 6));
        builder.base(0xCD, Self::new(Pointer(Data::new()), Always), 3, 6);

        builder.base(
            0xD4,
            Self::new(Pointer(Data::new()), Unset(Carry)),
            3,
            (3, 6),
        );

        builder.base(0xDC, Self::new(Pointer(Data::new()), Set(Carry)), 3, (3, 6));

        // RST instructions
        builder.base(0xC7, Self::new(Pointer(0x00), Always), 1, 4);
        builder.base(0xCF, Self::new(Pointer(0x08), Always), 1, 4);
        builder.base(0xD7, Self::new(Pointer(0x10), Always), 1, 4);
        builder.base(0xDF, Self::new(Pointer(0x18), Always), 1, 4);
        builder.base(0xE7, Self::new(Pointer(0x20), Always), 1, 4);
        builder.base(0xEF, Self::new(Pointer(0x28), Always), 1, 4);
        builder.base(0xF7, Self::new(Pointer(0x30), Always), 1, 4);
        builder.base(0xFF, Self::new(Pointer(0x38), Always), 1, 4);
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
    Pointer<Data<u16>> => CallTarget::DataPointer,
    Pointer<u16> => CallTarget::Vector,
);
