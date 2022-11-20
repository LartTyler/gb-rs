use super::Operation;
use crate::instructions::load::RegisterPointerLoad;
use parse_display::Display;

pub use data_pointer::*;
pub use pair::*;
pub use pair_pointer::*;
pub use register::*;

mod data_pointer;
mod pair;
mod pair_pointer;
mod register;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Load {
    Pair(PairLoad),
    PairPointer(PairPointerLoad),
    Register(RegisterLoad),
    RegisterPointer(RegisterPointerLoad),
    DataPointer(DataPointerLoad),
}

impl From<RegisterPointerLoad> for Operation {
    fn from(value: RegisterPointerLoad) -> Self {
        Self::Load(Load::RegisterPointer(value))
    }
}
