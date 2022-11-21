use super::Bit;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::enum_from_helper;
use crate::operations::Operation;
use parse_display::Display;

#[derive(Debug, Clone, Display)]
#[display("AND {target}")]
pub struct BitwiseAnd {
    pub target: BitwiseAndTarget,
}

impl BitwiseAnd {
    pub fn create<T>(target: T) -> Operation
    where
        T: Into<BitwiseAndTarget>,
    {
        Operation::Bit(Bit::And(Self {
            target: target.into(),
        }))
    }
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum BitwiseAndTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

enum_from_helper!(
    const Register => BitwiseAndTarget::Register,
    const Pointer<Pair> => BitwiseAndTarget::PairPointer,
    const Value<u8> => BitwiseAndTarget::Data,
);
