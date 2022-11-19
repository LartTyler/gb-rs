use super::Bitwise;
use crate::containers::{Bit, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bitwise as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("SET {bit}, {target}")]
pub struct BitwiseSet {
    pub target: BitwiseSetTarget,
    pub bit: Bit,
}

impl BitwiseSet {
    pub const fn new<T>(target: T, bit: Bit) -> Self
    where
        T: ~const Into<BitwiseSetTarget>,
    {
        Self {
            target: target.into(),
            bit,
        }
    }
}

impl Parse for BitwiseSet {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for BitwiseSet {
    fn register(builder: &mut Builder) {
        use Register::*;

        // SET 0, r8
        builder.extended(0xC0, Self::new(A, Bit::ZERO));
        builder.extended(0xC1, Self::new(B, Bit::ZERO));
        builder.extended(0xC2, Self::new(C, Bit::ZERO));
        builder.extended(0xC3, Self::new(D, Bit::ZERO));
        builder.extended(0xC4, Self::new(E, Bit::ZERO));
        builder.extended(0xC5, Self::new(H, Bit::ZERO));
        builder.extended(0xC7, Self::new(L, Bit::ZERO));

        // SET 0, (HL)
        builder.extended(0xC6, Self::new(Pointer(Pair::HL), Bit::ZERO));

        // SET 1, r8
        builder.extended(0xC8, Self::new(A, Bit::ONE));
        builder.extended(0xC9, Self::new(B, Bit::ONE));
        builder.extended(0xCA, Self::new(C, Bit::ONE));
        builder.extended(0xCB, Self::new(D, Bit::ONE));
        builder.extended(0xCC, Self::new(E, Bit::ONE));
        builder.extended(0xCD, Self::new(H, Bit::ONE));
        builder.extended(0xCF, Self::new(L, Bit::ONE));

        // SET 1, (HL)
        builder.extended(0xCE, Self::new(Pointer(Pair::HL), Bit::ONE));

        // SET 2, r8
        builder.extended(0xD0, Self::new(A, Bit::TWO));
        builder.extended(0xD1, Self::new(B, Bit::TWO));
        builder.extended(0xD2, Self::new(C, Bit::TWO));
        builder.extended(0xD3, Self::new(D, Bit::TWO));
        builder.extended(0xD4, Self::new(E, Bit::TWO));
        builder.extended(0xD5, Self::new(H, Bit::TWO));
        builder.extended(0xD7, Self::new(L, Bit::TWO));

        // SET 2, (HL)
        builder.extended(0xD6, Self::new(Pointer(Pair::HL), Bit::TWO));

        // SET 3, r8
        builder.extended(0xD8, Self::new(A, Bit::THREE));
        builder.extended(0xD9, Self::new(B, Bit::THREE));
        builder.extended(0xDA, Self::new(C, Bit::THREE));
        builder.extended(0xDB, Self::new(D, Bit::THREE));
        builder.extended(0xDC, Self::new(E, Bit::THREE));
        builder.extended(0xDD, Self::new(H, Bit::THREE));
        builder.extended(0xDF, Self::new(L, Bit::THREE));

        // SET 3, (HL)
        builder.extended(0xDE, Self::new(Pointer(Pair::HL), Bit::THREE));

        // SET 4, r8
        builder.extended(0xE0, Self::new(A, Bit::FOUR));
        builder.extended(0xE1, Self::new(B, Bit::FOUR));
        builder.extended(0xE2, Self::new(C, Bit::FOUR));
        builder.extended(0xE3, Self::new(D, Bit::FOUR));
        builder.extended(0xE4, Self::new(E, Bit::FOUR));
        builder.extended(0xE5, Self::new(H, Bit::FOUR));
        builder.extended(0xE7, Self::new(L, Bit::FOUR));

        // SET 4, (HL)
        builder.extended(0xE6, Self::new(Pointer(Pair::HL), Bit::FOUR));

        // SET 5, r8
        builder.extended(0xE8, Self::new(A, Bit::FIVE));
        builder.extended(0xE9, Self::new(B, Bit::FIVE));
        builder.extended(0xEA, Self::new(C, Bit::FIVE));
        builder.extended(0xEB, Self::new(D, Bit::FIVE));
        builder.extended(0xEC, Self::new(E, Bit::FIVE));
        builder.extended(0xED, Self::new(H, Bit::FIVE));
        builder.extended(0xEF, Self::new(L, Bit::FIVE));

        // SET 5, (HL)
        builder.extended(0xEE, Self::new(Pointer(Pair::HL), Bit::FIVE));

        // SET 6, r8
        builder.extended(0xF0, Self::new(A, Bit::SIX));
        builder.extended(0xF1, Self::new(B, Bit::SIX));
        builder.extended(0xF2, Self::new(C, Bit::SIX));
        builder.extended(0xF3, Self::new(D, Bit::SIX));
        builder.extended(0xF4, Self::new(E, Bit::SIX));
        builder.extended(0xF5, Self::new(H, Bit::SIX));
        builder.extended(0xF7, Self::new(L, Bit::SIX));

        // SET 6, (HL)
        builder.extended(0xF6, Self::new(Pointer(Pair::HL), Bit::SIX));

        // SET 7, r8
        builder.extended(0xF8, Self::new(A, Bit::SEVEN));
        builder.extended(0xF9, Self::new(B, Bit::SEVEN));
        builder.extended(0xFA, Self::new(C, Bit::SEVEN));
        builder.extended(0xFB, Self::new(D, Bit::SEVEN));
        builder.extended(0xFC, Self::new(E, Bit::SEVEN));
        builder.extended(0xFD, Self::new(H, Bit::SEVEN));
        builder.extended(0xFF, Self::new(L, Bit::SEVEN));

        // SET 7, (HL)
        builder.extended(0xFE, Self::new(Pointer(Pair::HL), Bit::SEVEN));
    }
}

impl const From<BitwiseSet> for Instruction {
    fn from(value: BitwiseSet) -> Self {
        Self::Bitwise(Bitwise::Set(value))
    }
}

impl From<BitwiseSet> for Operation {
    fn from(value: BitwiseSet) -> Self {
        Operation::Bitwise(op::Bitwise::Set(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseSetTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for BitwiseSetTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for BitwiseSetTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
