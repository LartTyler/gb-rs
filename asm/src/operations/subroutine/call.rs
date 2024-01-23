use super::Subroutine;
use crate::containers::{Condition, Pointer, Value};
use crate::enum_from_helper;
use crate::operations::OperationKind;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Call {
    pub target: CallTarget,
    pub condition: Condition,
}

impl Call {
    pub fn create(target: CallTarget, condition: Condition) -> OperationKind {
        OperationKind::Subroutine(Subroutine::Call(Self { target, condition }))
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

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CallTarget {
    DataPointer(Pointer<Value<u16>>),
    Vector(Pointer<u16>),
}

enum_from_helper!(
    Pointer<Value<u16>> => CallTarget::DataPointer,
    Pointer<u16> => CallTarget::Vector,
);
