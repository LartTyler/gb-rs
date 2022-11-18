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
    DataPointer(DataPointerLoad),
}
