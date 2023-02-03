use super::Load;
use crate::containers::{Pair, Signed, Value};
use crate::operations::OperationKind;
use parse_display::Display;

#[derive(Debug, Clone, Display)]
#[display("LD {target}, {source}")]
pub struct PairLoad {
    pub target: Pair,
    pub source: PairLoadSource,
}

impl PairLoad {
    pub fn create<S>(target: Pair, source: S) -> OperationKind
    where
        S: Into<PairLoadSource>,
    {
        OperationKind::Load(Load::Pair(Self {
            target,
            source: source.into(),
        }))
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum PairLoadSource {
    Data(Value<u16>),
    Pair(Pair),

    #[display("SP + {0}")]
    SignedData(Signed<Value<u8>>),
}

impl From<Value<u16>> for PairLoadSource {
    fn from(value: Value<u16>) -> Self {
        Self::Data(value)
    }
}

impl From<Pair> for PairLoadSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl From<Signed<Value<u8>>> for PairLoadSource {
    fn from(value: Signed<Value<u8>>) -> Self {
        Self::SignedData(value)
    }
}
