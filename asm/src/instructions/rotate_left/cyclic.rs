use super::RotateLeft;
use crate::containers::{Pair, Pointer, Register};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::{read::Read, sets::Builder};
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct CyclicRotateLeft {
    pub target: CyclicRotateLeftTarget,
    pub extended: bool,
}

impl CyclicRotateLeft {
    pub fn new<T>(target: T, extended: bool) -> Self
    where
        T: Into<CyclicRotateLeftTarget>,
    {
        Self {
            target: target.into(),
            extended,
        }
    }
}

impl Display for CyclicRotateLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = if self.extended { " " } else { "" };
        write!(f, "RLC{spacing}{}", self.target)
    }
}

impl Parse for CyclicRotateLeft {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl SetRegister for CyclicRotateLeft {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RLCA
        builder.base(0x07, Self::new(A, false), 1, 1);

        // PREFIX RLC r8
        builder.extended(0x00, Self::new(B, true), 2, 2);
        builder.extended(0x01, Self::new(C, true), 2, 2);
        builder.extended(0x02, Self::new(D, true), 2, 2);
        builder.extended(0x03, Self::new(E, true), 2, 2);
        builder.extended(0x04, Self::new(H, true), 2, 2);
        builder.extended(0x05, Self::new(L, true), 2, 2);
        builder.extended(0x07, Self::new(A, true), 2, 2);

        // PREFIX RLC (HL)
        builder.extended(0x06, Self::new(Pointer(Pair::HL), true), 2, 4);
    }
}

impl From<CyclicRotateLeft> for InstructionKind {
    fn from(value: CyclicRotateLeft) -> Self {
        Self::RotateLeft(RotateLeft::Cylic(value))
    }
}

impl From<CyclicRotateLeft> for OperationKind {
    fn from(value: CyclicRotateLeft) -> Self {
        Self::RotateLeft(RotateLeft::Cylic(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CyclicRotateLeftTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl From<Register> for CyclicRotateLeftTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Pointer<Pair>> for CyclicRotateLeftTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
