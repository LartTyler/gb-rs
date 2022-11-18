use super::Load;
use crate::containers::{Register, Value};
use crate::instructions::load as instr;
use crate::operations::Operation;
use parse_display::Display;

#[derive(Debug, Clone, Display)]
#[display("LD {target}, {source}")]
pub struct RegisterLoad {
    pub target: Register,
    pub source: RegisterLoadSource,
}

impl RegisterLoad {
    pub fn create<S>(target: Register, source: S) -> Operation
    where
        S: Into<RegisterLoadSource>,
    {
        Operation::Load(Load::Register(Self {
            target,
            source: source.into(),
        }))
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum RegisterLoadSource {
    Data(Value<u8>),
    PairPointer(instr::PairPointerRegisterLoadSource),
    Register(Register),
}

impl From<Value<u8>> for RegisterLoadSource {
    fn from(value: Value<u8>) -> Self {
        Self::Data(value)
    }
}

impl From<instr::PairPointerRegisterLoadSource> for RegisterLoadSource {
    fn from(value: instr::PairPointerRegisterLoadSource) -> Self {
        Self::PairPointer(value)
    }
}

impl From<Register> for RegisterLoadSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}
