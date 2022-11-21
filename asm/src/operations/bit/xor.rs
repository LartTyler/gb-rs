use super::Bit;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::enum_from_helper;
use crate::operations::Operation;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("XOR {target}")]
pub struct BitwiseXor {
    pub target: BitwiseXorTarget,
}

impl BitwiseXor {
    pub fn create<T>(target: T) -> Operation
    where
        T: Into<BitwiseXorTarget>,
    {
        Operation::Bit(Bit::Xor(Self {
            target: target.into(),
        }))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseXorTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

enum_from_helper!(
    const Register => BitwiseXorTarget::Register,
    const Pointer<Pair> => BitwiseXorTarget::PairPointer,
    const Value<u8> => BitwiseXorTarget::Data,
);
