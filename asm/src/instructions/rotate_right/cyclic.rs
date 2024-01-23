use super::RotateRight;
use crate::containers::{Pair, Pointer, Register};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct CyclicRotateRight {
    pub target: CyclicRotateRightTarget,
    pub extended: bool,
}

impl CyclicRotateRight {
    pub fn new<T>(target: T, extended: bool) -> Self
    where
        T: Into<CyclicRotateRightTarget>,
    {
        Self {
            target: target.into(),
            extended,
        }
    }
}

impl Parse for CyclicRotateRight {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for CyclicRotateRight {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RRCA
        builder.base(0x0F, Self::new(A, false), 1, 1);

        // PREFIX RRC r8
        builder.extended(0x08, Self::new(B, true), 2, 2);
        builder.extended(0x09, Self::new(C, true), 2, 2);
        builder.extended(0x0A, Self::new(D, true), 2, 2);
        builder.extended(0x0B, Self::new(E, true), 2, 2);
        builder.extended(0x0C, Self::new(H, true), 2, 2);
        builder.extended(0x0D, Self::new(L, true), 2, 2);
        builder.extended(0x0F, Self::new(A, true), 2, 2);

        // PREFIX RRC (HL)
        builder.extended(0x0E, Self::new(Pointer(Pair::HL), true), 2, 4);
    }
}

impl Display for CyclicRotateRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = if self.extended { " " } else { "" };
        write!(f, "RRC{spacing}{}", self.target)
    }
}

impl From<CyclicRotateRight> for InstructionKind {
    fn from(value: CyclicRotateRight) -> Self {
        Self::RotateRight(RotateRight::Cyclic(value))
    }
}

impl From<CyclicRotateRight> for OperationKind {
    fn from(value: CyclicRotateRight) -> Self {
        Self::RotateRight(RotateRight::Cyclic(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CyclicRotateRightTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl From<Register> for CyclicRotateRightTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Pointer<Pair>> for CyclicRotateRightTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
