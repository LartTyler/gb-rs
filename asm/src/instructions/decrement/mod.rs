use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use pair::*;
pub use pair_pointer::*;
pub use register::*;

mod pair;
mod pair_pointer;
mod register;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Decrement {
    Register(RegisterDecrement),
    Pair(PairDecrement),
    PairPointer(PairPointerDecrement),
}

impl Parse for Decrement {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Register(inner),
            Self::Pair(inner),
            Self::PairPointer(inner)
        )
    }
}

impl const SetRegister for Decrement {
    fn register(builder: &mut crate::sets::Builder) {
        register_helper!(
            builder,
            RegisterDecrement,
            PairDecrement,
            PairPointerDecrement
        );
    }
}
