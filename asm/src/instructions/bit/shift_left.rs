use crate::containers::{Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::bit as op;
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("SLA {target}")]
pub struct ShiftLeft {
    pub target: ShiftLeftTarget,
}

impl ShiftLeft {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<ShiftLeftTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<ShiftLeft> for InstructionKind {
    fn from(value: ShiftLeft) -> Self {
        Self::Bit(super::Bit::ShiftLeft(value))
    }
}

impl From<ShiftLeft> for OperationKind {
    fn from(value: ShiftLeft) -> Self {
        Self::Bit(op::Bit::ShiftLeft(value))
    }
}

impl const SetRegister for ShiftLeft {
    fn register(builder: &mut Builder) {
        use Register::*;

        // SLA r8
        builder.extended(0x20, Self::new(B), 2, 2);
        builder.extended(0x21, Self::new(C), 2, 2);
        builder.extended(0x22, Self::new(D), 2, 2);
        builder.extended(0x23, Self::new(E), 2, 2);
        builder.extended(0x24, Self::new(H), 2, 2);
        builder.extended(0x25, Self::new(L), 2, 2);
        builder.extended(0x27, Self::new(A), 2, 2);

        // Others
        builder.extended(0x26, Self::new(Pointer(Pair::HL)), 2, 4);
    }
}

impl Parse for ShiftLeft {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum ShiftLeftTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

enum_from_helper!(
    const Register => ShiftLeftTarget::Register,
    const Pointer<Pair> => ShiftLeftTarget::PairPointer,
);
