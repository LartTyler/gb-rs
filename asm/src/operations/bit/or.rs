use super::Bit;
use crate::containers::{Pair, Pointer, Register, Value};
use crate::enum_from_helper;
use crate::operations::OperationKind;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("OR {target}")]
pub struct BitwiseOr {
    pub target: BitwiseOrTarget,
}

impl BitwiseOr {
    pub fn create<T>(target: T) -> OperationKind
    where
        T: Into<BitwiseOrTarget>,
    {
        OperationKind::Bit(Bit::Or(Self {
            target: target.into(),
        }))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseOrTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Value<u8>),
}

enum_from_helper!(
    const Register => BitwiseOrTarget::Register,
    const Pointer<Pair> => BitwiseOrTarget::PairPointer,
    const Value<u8> => BitwiseOrTarget::Data,
);
