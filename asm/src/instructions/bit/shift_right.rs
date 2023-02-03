use crate::containers::{Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::{bit as op, OperationKind};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct ShiftRight {
    pub target: ShiftRightTarget,
    pub kind: ShiftRightKind,
}

impl ShiftRight {
    pub const fn new<T>(target: T, kind: ShiftRightKind) -> Self
    where
        T: ~const Into<ShiftRightTarget>,
    {
        Self {
            target: target.into(),
            kind,
        }
    }
}

impl Display for ShiftRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SR{} {}", self.kind, self.target)
    }
}

impl const From<ShiftRight> for InstructionKind {
    fn from(value: ShiftRight) -> Self {
        Self::Bit(super::Bit::ShiftRight(value))
    }
}

impl From<ShiftRight> for OperationKind {
    fn from(value: ShiftRight) -> Self {
        Self::Bit(op::Bit::ShiftRight(value))
    }
}

impl const SetRegister for ShiftRight {
    fn register(builder: &mut Builder) {
        use Register::*;
        use ShiftRightKind::*;

        // SRA instructions
        builder.extended(0x28, Self::new(B, Arithmetic), 2, 2);
        builder.extended(0x29, Self::new(C, Arithmetic), 2, 2);
        builder.extended(0x2A, Self::new(D, Arithmetic), 2, 2);
        builder.extended(0x2B, Self::new(E, Arithmetic), 2, 2);
        builder.extended(0x2C, Self::new(H, Arithmetic), 2, 2);
        builder.extended(0x2D, Self::new(L, Arithmetic), 2, 2);
        builder.extended(0x2E, Self::new(Pointer(Pair::HL), Arithmetic), 2, 4);
        builder.extended(0x2F, Self::new(A, Arithmetic), 2, 2);

        // SRL instructions
        builder.extended(0x38, Self::new(B, Logical), 2, 2);
        builder.extended(0x39, Self::new(C, Logical), 2, 2);
        builder.extended(0x3A, Self::new(D, Logical), 2, 2);
        builder.extended(0x3B, Self::new(E, Logical), 2, 2);
        builder.extended(0x3C, Self::new(H, Logical), 2, 2);
        builder.extended(0x3D, Self::new(L, Logical), 2, 2);
        builder.extended(0x3E, Self::new(Pointer(Pair::HL), Logical), 2, 4);
        builder.extended(0x3F, Self::new(A, Logical), 2, 2);
    }
}

impl Parse for ShiftRight {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum ShiftRightTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

enum_from_helper!(
    const Register => ShiftRightTarget::Register,
    const Pointer<Pair> => ShiftRightTarget::PairPointer,
);

#[derive(Debug, Clone, Copy, Display)]
pub enum ShiftRightKind {
    #[display("A")]
    Arithmetic,

    #[display("L")]
    Logical,
}
