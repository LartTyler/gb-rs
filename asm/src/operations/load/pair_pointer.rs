use super::Load;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::instructions::load::Action;
use crate::operations::Operation;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PairPointerLoad {
    pub target: Pointer<Pair>,
    pub source: PairPointerLoadSource,
    pub action: Action,
}

impl PairPointerLoad {
    pub fn create<S>(target: Pointer<Pair>, source: S, action: Action) -> Operation
    where
        S: Into<PairPointerLoadSource>,
    {
        Operation::Load(Load::PairPointer(Self {
            target,
            source: source.into(),
            action,
        }))
    }
}

impl Display for PairPointerLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LD ({}{}), {}", self.target, self.action, self.source)
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum PairPointerLoadSource {
    Register(Register),
    Data(Value<u8>),
}

impl From<Register> for PairPointerLoadSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Value<u8>> for PairPointerLoadSource {
    fn from(value: Value<u8>) -> Self {
        Self::Data(value)
    }
}
