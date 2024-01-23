use super::Jump;
use crate::containers::{Condition, Data, Flag, Signed};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::jump as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct RelativeJump {
    pub offset: Signed<Data<u8>>,
    pub condition: Condition,
}

impl RelativeJump {
    pub fn new(offset: Signed<Data<u8>>, condition: Condition) -> Self {
        Self { offset, condition }
    }
}

impl Parse for RelativeJump {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        let jump_offset = self.offset.parse(data, offset)?;
        Ok(op::RelativeJump::create(jump_offset, self.condition))
    }
}

impl SetRegister for RelativeJump {
    fn register(builder: &mut Builder) {
        use Condition::*;
        use Flag::{Carry, Zero};

        // JR cc, s8
        builder.base(0x20, Self::new(Signed::new(), Unset(Zero)), 2, (2, 3));
        builder.base(0x28, Self::new(Signed::new(), Set(Zero)), 2, (2, 3));
        builder.base(0x30, Self::new(Signed::new(), Unset(Carry)), 2, (2, 3));
        builder.base(0x38, Self::new(Signed::new(), Set(Carry)), 2, (2, 3));

        // Others
        builder.base(0x18, Self::new(Signed::new(), Always), 2, 3);
    }
}

impl From<RelativeJump> for InstructionKind {
    fn from(value: RelativeJump) -> Self {
        Self::Jump(Jump::Relative(value))
    }
}

impl Display for RelativeJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if self.condition != Condition::Always {
            ", "
        } else {
            ""
        };

        write!(f, "JR {}{sep}{}", self.condition, self.offset)
    }
}
