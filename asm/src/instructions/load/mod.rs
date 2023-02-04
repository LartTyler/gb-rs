use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::{parse_helper, read, register_helper, sets};
use parse_display::Display;

pub use data_pointer::*;
pub use pair::*;
pub use pair_pointer::*;
pub use register::*;
pub use register_pointer::*;

mod data_pointer;
mod pair;
mod pair_pointer;
mod register;
mod register_pointer;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Load {
    Pair(PairLoad),
    PairPointer(PairPointerLoad),
    Register(RegisterLoad),
    RegisterPointer(RegisterPointerLoad),
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
            Self::RegisterPointer(inner),
        )
    }
}

impl const SetRegister for Load {
    fn register(builder: &mut sets::Builder) {
        register_helper!(
            builder,
            PairLoad,
            PairPointerLoad,
            RegisterLoad,
            DataPointerLoad,
            RegisterPointerLoad,
        );
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

impl Action {
    pub fn apply(&self, value: &mut u16) {
        match self {
            Self::Increment => *value = value.wrapping_add(1),
            Self::Decrement => *value = value.wrapping_sub(1),
            Self::None => (),
        };
    }
}
