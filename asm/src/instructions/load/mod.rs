use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::{parse_helper, read, sets};
use parse_display::Display;

pub use data_pointer::*;
pub use pair::*;
pub use pair_pointer::*;
pub use register::*;

mod data_pointer;
mod pair;
mod pair_pointer;
mod register;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Load {
    Pair(PairLoad),
    PairPointer(PairPointerLoad),
    Register(RegisterLoad),
    DataPointer(DataPointerLoad),
}

impl Parse for Load {
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Pair(inner),
            Self::PairPointer(inner),
            Self::Register(inner),
            Self::DataPointer(inner),
        )
    }
}

impl const SetRegister for Load {
    fn register(builder: &mut sets::Builder) {
        PairLoad::register(builder);
        PairPointerLoad::register(builder);
        RegisterLoad::register(builder);
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum Action {
    #[display("")]
    None,

    #[display("+")]
    Increment,

    #[display("-")]
    Decrement,
}
