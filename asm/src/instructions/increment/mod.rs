use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
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
pub enum Increment {
    Pair(PairIncrement),
    Register(RegisterIncrement),
    PairPointer(PairPointerIncrement),
}

impl Parse for Increment {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Pair(inner),
            Self::Register(inner),
            Self::PairPointer(inner)
        )
    }
}

impl SetRegister for Increment {
    fn register(builder: &mut Builder) {
        register_helper!(
            builder,
            PairIncrement,
            RegisterIncrement,
            PairPointerIncrement
        );
    }
}
