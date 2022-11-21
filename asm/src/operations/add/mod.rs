use parse_display::Display;

pub use pair::*;
pub use register::*;

mod pair;
mod register;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Add {
    Pair(PairAdd),
    Register(RegisterAdd),
}
