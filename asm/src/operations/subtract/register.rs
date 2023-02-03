use super::Subtract;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::enum_from_helper;
use crate::operations::OperationKind;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct RegisterSubtract {
    pub source: RegisterSubtractSource,
    pub with_carry: bool,
}

impl RegisterSubtract {
    pub fn create<S>(source: S, with_carry: bool) -> OperationKind
    where
        S: Into<RegisterSubtractSource>,
    {
        OperationKind::Subtract(Subtract::Register(Self {
            source: source.into(),
            with_carry,
        }))
    }
}

impl Display for RegisterSubtract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.with_carry { "SBC" } else { "SUB" };
        write!(f, "{mnemonic} A, {}", self.source)
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum RegisterSubtractSource {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

enum_from_helper!(
    const Register => RegisterSubtractSource::Register,
    const Pointer<Pair> => RegisterSubtractSource::PairPointer,
    const Value<u8> => RegisterSubtractSource::Data,
);
