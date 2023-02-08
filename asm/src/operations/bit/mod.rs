use crate::instructions::bit::{
    BitwiseReset, BitwiseSet, BitwiseTest, Complement, ShiftLeft, ShiftRight, Swap,
};
use parse_display::Display;

pub use and::*;
pub use or::*;
pub use xor::*;

use super::OperationKind;

mod and;
mod or;
mod xor;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Bit {
    #[display("SCF")]
    SetCarryFlag,

    Complement(Complement),
    Set(BitwiseSet),
    Reset(BitwiseReset),
    Test(BitwiseTest),
    And(BitwiseAnd),
    Xor(BitwiseXor),
    Or(BitwiseOr),
    ShiftLeft(ShiftLeft),
    ShiftRight(ShiftRight),
    Swap(Swap),
}

impl From<Complement> for OperationKind {
    fn from(value: Complement) -> Self {
        Self::Bit(Bit::Complement(value))
    }
}
