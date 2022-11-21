use super::Jump;
use crate::containers::{Condition, Signed, Value};
use crate::operations::Operation;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct RelativeJump {
    pub offset: Signed<Value<u8>>,
    pub condition: Condition,
}

impl RelativeJump {
    pub fn create<O>(offset: O, condition: Condition) -> Operation
    where
        O: Into<Signed<Value<u8>>>,
    {
        Operation::Jump(Jump::Relative(Self {
            offset: offset.into(),
            condition,
        }))
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
