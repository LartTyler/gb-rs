use super::RotateRight;
use crate::containers::{Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct CarryingRotateRight {
    pub target: CarryingRotateRightTarget,
    pub extended: bool,
}

impl CarryingRotateRight {
    pub const fn new<T>(target: T, extended: bool) -> Self
    where
        T: ~const Into<CarryingRotateRightTarget>,
    {
        Self {
            target: target.into(),
            extended,
        }
    }
}

impl Parse for CarryingRotateRight {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for CarryingRotateRight {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RRA
        builder.base(0x1F, Self::new(A, false));

        // PREFIX RR r8
        builder.extended(0x18, Self::new(B, true));
        builder.extended(0x19, Self::new(C, true));
        builder.extended(0x1A, Self::new(D, true));
        builder.extended(0x1B, Self::new(E, true));
        builder.extended(0x1C, Self::new(H, true));
        builder.extended(0x1D, Self::new(L, true));
        builder.extended(0x1F, Self::new(A, true));

        // PREFIX RR (HL)
        builder.extended(0x1E, Self::new(Pointer(Pair::HL), true));
    }
}

impl Display for CarryingRotateRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = if self.extended { " " } else { "" };
        write!(f, "RR{spacing}{}", self.target)
    }
}

impl const From<CarryingRotateRight> for Instruction {
    fn from(value: CarryingRotateRight) -> Self {
        Self::RotateRight(RotateRight::Carrying(value))
    }
}

impl From<CarryingRotateRight> for Operation {
    fn from(value: CarryingRotateRight) -> Self {
        Self::RotateRight(RotateRight::Carrying(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CarryingRotateRightTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for CarryingRotateRightTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for CarryingRotateRightTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
