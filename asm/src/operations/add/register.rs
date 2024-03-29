use super::Add;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::operations::OperationKind;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct RegisterAdd {
    pub source: RegisterAddSource,
    pub with_carry: bool,
}

impl RegisterAdd {
    pub fn create<S>(source: S, with_carry: bool) -> OperationKind
    where
        S: Into<RegisterAddSource>,
    {
        OperationKind::Add(Add::Register(Self {
            source: source.into(),
            with_carry,
        }))
    }
}

impl Display for RegisterAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.with_carry { 'C' } else { 'D' };
        write!(f, "AD{c} A, {}", self.source)
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum RegisterAddSource {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

impl From<Register> for RegisterAddSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Pointer<Pair>> for RegisterAddSource {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}

impl From<Value<u8>> for RegisterAddSource {
    fn from(value: Value<u8>) -> Self {
        Self::Data(value)
    }
}
