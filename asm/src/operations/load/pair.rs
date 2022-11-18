use super::Load;
use crate::containers::{Pair, Value};
use crate::operations::Operation;

pub struct PairLoad {
    pub target: Pair,
    pub source: PairLoadSource,
}

impl PairLoad {
    pub fn create<S>(target: Pair, source: S) -> Operation
    where
        S: Into<PairLoadSource>,
    {
        Operation::Load(Load::Pair(Self {
            target,
            source: source.into(),
        }))
    }
}

pub enum PairLoadSource {
    Data(Value<u16>),
}

impl<T> From<T> for PairLoadSource
where
    T: Into<Value<u16>>,
{
    fn from(value: T) -> Self {
        PairLoadSource::Data(value.into())
    }
}
