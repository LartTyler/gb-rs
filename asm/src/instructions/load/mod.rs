use super::SetRegister;
use crate::operations::Operation;
use crate::{parse, parse_helper, read, sets};
use parse_display::Display;

pub use pair::*;
pub use pair_pointer::*;
pub use register::*;

mod pair;
mod pair_pointer;
mod register;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Load {
    Pair(PairLoad),
    PairPointer(PairPointerLoad),
    Register(RegisterLoad),
}

impl parse::Parse for Load {
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        parse_helper!(
            self,
            data[offset],
            Self::Pair(inner),
            Self::PairPointer(inner),
            Self::Register(inner),
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
