use super::OperationKind;
use crate::{
    containers::{Pair, Pointer, Register, Value},
    enum_from_helper,
};
use parse_display::Display;

#[derive(Debug, Clone, Display)]
#[display("CP {target}")]
pub struct Compare {
    pub target: CompareTarget,
}

impl Compare {
    pub fn create<T>(target: T) -> OperationKind
    where
        T: Into<CompareTarget>,
    {
        OperationKind::Compare(Self {
            target: target.into(),
        })
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum CompareTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

enum_from_helper!(
    Register => CompareTarget::Register,
    Pointer<Pair> => CompareTarget::PairPointer,
    Value<u8> => CompareTarget::Data,
);
