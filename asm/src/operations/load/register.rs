use super::Load;
use crate::containers::{Pointer, Register, Value};
use crate::enum_from_helper;
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
    DataPointer(Pointer<Value<u16>>),
    HighDataPointer(Pointer<Value<u8>>),
    RegisterPointer(Pointer<Register>),
}

enum_from_helper!(
    const Value<u8> => RegisterLoadSource::Data,
    const instr::PairPointerRegisterLoadSource => RegisterLoadSource::PairPointer,
    const Register => RegisterLoadSource::Register,
    const Pointer<Value<u16>> => RegisterLoadSource::DataPointer,
    const Pointer<Value<u8>> => RegisterLoadSource::HighDataPointer,
    const Pointer<Register> => RegisterLoadSource::RegisterPointer,
);
