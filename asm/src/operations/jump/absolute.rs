use super::Jump;
use crate::containers::{Condition, Pair, Pointer, Value};
use crate::enum_from_helper;
use crate::operations::OperationKind;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct AbsoluteJump {
    pub target: AbsoluteJumpTarget,
    pub condition: Condition,
}

impl AbsoluteJump {
    pub fn create(target: AbsoluteJumpTarget, condition: Condition) -> OperationKind {
        OperationKind::Jump(Jump::Absolute(Self { target, condition }))
    }
}

impl Display for AbsoluteJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.condition != Condition::Always {
            write!(f, "JP {}, {}", self.condition, self.target)
        } else {
            write!(f, "JP {}", self.target)
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum AbsoluteJumpTarget {
    DataPointer(Pointer<Value<u16>>),
    PairPointer(Pointer<Pair>),
}

enum_from_helper!(
    Pointer<Value<u16>> => AbsoluteJumpTarget::DataPointer,
    Pointer<Pair> => AbsoluteJumpTarget::PairPointer,
);
