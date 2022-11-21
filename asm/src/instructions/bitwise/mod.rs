use super::{Instruction, SetRegister};
use crate::operations::{bitwise as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use and::*;
pub use complement::*;
pub use or::*;
pub use reset::*;
pub use set::*;
pub use shift_left::*;
pub use shift_right::*;
pub use swap::*;
pub use test::*;
pub use xor::*;

mod and;
mod complement;
mod or;
mod reset;
mod set;
mod shift_left;
mod shift_right;
mod swap;
mod test;
mod xor;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Bitwise {
    #[display("SCF")]
    SetCarryFlag,

    Complement(BitwiseComplement),
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

impl Parse for Bitwise {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::SetCarryFlag => Operation::Bitwise(op::Bitwise::SetCarryFlag),
            Self::Complement(inner),
            Self::Set(inner),
            Self::Reset(inner),
            Self::Test(inner),
            Self::And(inner),
            Self::Xor(inner),
            Self::Or(inner),
            Self::ShiftLeft(inner),
            Self::ShiftRight(inner),
            Self::Swap(inner),
        )
    }
}

impl const SetRegister for Bitwise {
    fn register(builder: &mut Builder) {
        builder.base(0x37, Self::SetCarryFlag);

        register_helper!(
            builder,
            BitwiseComplement,
            BitwiseSet,
            BitwiseReset,
            BitwiseTest,
            BitwiseAnd,
            BitwiseXor,
            BitwiseOr,
            ShiftLeft,
            ShiftRight,
            Swap,
        );
    }
}

impl const From<Bitwise> for Instruction {
    fn from(value: Bitwise) -> Self {
        Instruction::Bitwise(value)
    }
}
