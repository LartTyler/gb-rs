use super::Add;
use crate::containers::{Pair, Signed, Value};
use crate::operations::Operation;
use parse_display::Display;

#[derive(Debug, Clone, Display)]
#[display("ADD {target}, {source}")]
pub struct PairAdd {
    pub target: Pair,
    pub source: PairAddSource,
}

impl PairAdd {
    pub fn create<S>(target: Pair, source: S) -> Operation
    where
        S: Into<PairAddSource>,
    {
        Operation::Add(Add::Pair(Self {
            target,
            source: source.into(),
        }))
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum PairAddSource {
    Pair(Pair),
    SignedData(Signed<Value<u8>>),
}

impl From<Pair> for PairAddSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl From<Signed<Value<u8>>> for PairAddSource {
    fn from(value: Signed<Value<u8>>) -> Self {
        Self::SignedData(value)
    }
}
