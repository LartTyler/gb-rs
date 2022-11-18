use super::RotateLeft;
use crate::containers::{Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::{read::Read, sets::Builder};
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct CarryingRotateLeft {
    pub target: CarryingRotateLeftTarget,
    pub extended: bool,
}

impl CarryingRotateLeft {
    pub const fn new<T>(target: T, extended: bool) -> Self
    where
        T: ~const Into<CarryingRotateLeftTarget>,
    {
        Self {
            target: target.into(),
            extended,
        }
    }
}

impl Display for CarryingRotateLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = if self.extended { " " } else { "" };
        write!(f, "RL{spacing}{}", self.target)
    }
}

impl Parse for CarryingRotateLeft {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for CarryingRotateLeft {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RLA
        builder.base(0x17, Self::new(A, false));

        // PREFIX RL r8
        builder.extended(0x10, Self::new(B, true));
        builder.extended(0x11, Self::new(C, true));
        builder.extended(0x12, Self::new(D, true));
        builder.extended(0x13, Self::new(E, true));
        builder.extended(0x14, Self::new(H, true));
        builder.extended(0x15, Self::new(L, true));
        builder.extended(0x17, Self::new(A, true));

        // PREFIX RL (HL)
        builder.extended(0x16, Self::new(Pointer(Pair::HL), true));
    }
}

impl const From<CarryingRotateLeft> for Instruction {
    fn from(value: CarryingRotateLeft) -> Self {
        Self::RotateLeft(RotateLeft::Carrying(value))
    }
}

impl From<CarryingRotateLeft> for Operation {
    fn from(value: CarryingRotateLeft) -> Self {
        Self::RotateLeft(RotateLeft::Carrying(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CarryingRotateLeftTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for CarryingRotateLeftTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for CarryingRotateLeftTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
