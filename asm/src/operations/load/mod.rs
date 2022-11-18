pub use pair::*;

mod pair;

pub enum Load {
    Pair(PairLoad),
}
