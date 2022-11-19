use super::Subroutine;
use crate::containers::{Condition, Pointer, Value};
use crate::operations::Operation;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Call {
    pub target: Pointer<Value<u16>>,
    pub condition: Condition,
}

impl Call {
    pub fn create(target: Pointer<Value<u16>>, condition: Condition) -> Operation {
        Operation::Subroutine(Subroutine::Call(Self { target, condition }))
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
