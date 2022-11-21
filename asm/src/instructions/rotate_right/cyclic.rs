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
pub struct CyclicRotateRight {
    pub target: CyclicRotateRightTarget,
    pub extended: bool,
}

impl CyclicRotateRight {
    pub const fn new<T>(target: T, extended: bool) -> Self
    where
        T: ~const Into<CyclicRotateRightTarget>,
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

impl const SetRegister for CyclicRotateRight {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RRCA
        builder.base(0x0F, Self::new(A, false));

        // PREFIX RRC r8
        builder.extended(0x08, Self::new(B, true));
        builder.extended(0x09, Self::new(C, true));
        builder.extended(0x0A, Self::new(D, true));
        builder.extended(0x0B, Self::new(E, true));
        builder.extended(0x0C, Self::new(H, true));
        builder.extended(0x0D, Self::new(L, true));
        builder.extended(0x0F, Self::new(A, true));

        // PREFIX RRC (HL)
        builder.extended(0x0E, Self::new(Pointer(Pair::HL), true));
    }
}

impl Display for CyclicRotateRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = if self.extended { " " } else { "" };
        write!(f, "RRC{spacing}{}", self.target)
    }
}

impl const From<CyclicRotateRight> for Instruction {
    fn from(value: CyclicRotateRight) -> Self {
        Self::RotateRight(RotateRight::Cyclic(value))
    }
}

impl From<CyclicRotateRight> for Operation {
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

impl const From<Register> for CyclicRotateRightTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for CyclicRotateRightTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
