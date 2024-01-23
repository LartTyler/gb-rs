use super::Bit;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::enum_from_helper;
use crate::operations::OperationKind;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("XOR {target}")]
pub struct BitwiseXor {
    pub target: BitwiseXorTarget,
}

impl BitwiseXor {
    pub fn create<T>(target: T) -> OperationKind
    where
        T: Into<BitwiseXorTarget>,
    {
        OperationKind::Bit(Bit::Xor(Self {
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
    Register => BitwiseXorTarget::Register,
    Pointer<Pair> => BitwiseXorTarget::PairPointer,
    Value<u8> => BitwiseXorTarget::Data,
);
