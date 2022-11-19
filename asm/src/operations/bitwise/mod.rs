use crate::instructions::bitwise::{BitwiseReset, BitwiseSet, BitwiseTest};
use parse_display::Display;

pub use and::*;
pub use complement::*;
pub use xor::*;

mod and;
mod complement;
mod xor;

#[derive(Debug, Clone, Display)]
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
}
