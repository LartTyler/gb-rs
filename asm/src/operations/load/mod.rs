use parse_display::Display;

pub use pair::*;

mod pair;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Load {
    Pair(PairLoad),
}
