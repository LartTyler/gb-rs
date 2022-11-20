use super::Jump;
use crate::containers::{Condition, Data, Flag, Pair, Pointer};
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::jump as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct AbsoluteJump {
    pub target: AbsoluteJumpTarget,
    pub condition: Condition,
}

impl AbsoluteJump {
    pub const fn new<T>(target: T, condition: Condition) -> Self
    where
        T: ~const Into<AbsoluteJumpTarget>,
    {
        Self {
            target: target.into(),
            condition,
        }
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

impl const From<AbsoluteJump> for Instruction {
    fn from(value: AbsoluteJump) -> Self {
        Self::Jump(Jump::Absolute(value))
    }
}

impl const SetRegister for AbsoluteJump {
    fn register(builder: &mut Builder) {
        use Condition::*;
        use Flag::*;

        builder.base(0xC2, Self::new(Pointer(Data::new()), Unset(Zero)));
        builder.base(0xC3, Self::new(Pointer(Data::new()), Always));
        builder.base(0xCA, Self::new(Pointer(Data::new()), Set(Zero)));
        builder.base(0xD2, Self::new(Pointer(Data::new()), Unset(Carry)));
        builder.base(0xDA, Self::new(Pointer(Data::new()), Set(Carry)));
        builder.base(0xE9, Self::new(Pointer(Pair::HL), Always));
    }
}

impl Parse for AbsoluteJump {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use AbsoluteJumpTarget::*;

        let target: op::AbsoluteJumpTarget = match self.target {
            DataPointer(p) => p.parse(data, offset)?.into(),
            PairPointer(p) => p.into(),
        };

        Ok(op::AbsoluteJump::create(target, self.condition))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum AbsoluteJumpTarget {
    DataPointer(Pointer<Data<u16>>),
    PairPointer(Pointer<Pair>),
}

enum_from_helper!(
    const Pointer<Data<u16>> => AbsoluteJumpTarget::DataPointer,
    const Pointer<Pair> => AbsoluteJumpTarget::PairPointer
);
