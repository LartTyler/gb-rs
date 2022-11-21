use super::Operation;
use crate::instructions::subroutine::Return;
use parse_display::Display;

pub use call::*;

mod call;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Subroutine {
    Return(Return),
    Call(Call),
}

impl From<Return> for Operation {
    fn from(value: Return) -> Self {
        Self::Subroutine(Subroutine::Return(value))
    }
}
