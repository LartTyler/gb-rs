use crate::containers::{BitPosition, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bit as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("SET {bit}, {target}")]
pub struct BitwiseSet {
    pub target: BitwiseSetTarget,
    pub bit: BitPosition,
}

impl BitwiseSet {
    pub const fn new<T>(target: T, bit: BitPosition) -> Self
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
        builder.extended(0xC0, Self::new(A, BitPosition::ZERO));
        builder.extended(0xC1, Self::new(B, BitPosition::ZERO));
        builder.extended(0xC2, Self::new(C, BitPosition::ZERO));
        builder.extended(0xC3, Self::new(D, BitPosition::ZERO));
        builder.extended(0xC4, Self::new(E, BitPosition::ZERO));
        builder.extended(0xC5, Self::new(H, BitPosition::ZERO));
        builder.extended(0xC7, Self::new(L, BitPosition::ZERO));

        // SET 0, (HL)
        builder.extended(0xC6, Self::new(Pointer(Pair::HL), BitPosition::ZERO));

        // SET 1, r8
        builder.extended(0xC8, Self::new(A, BitPosition::ONE));
        builder.extended(0xC9, Self::new(B, BitPosition::ONE));
        builder.extended(0xCA, Self::new(C, BitPosition::ONE));
        builder.extended(0xCB, Self::new(D, BitPosition::ONE));
        builder.extended(0xCC, Self::new(E, BitPosition::ONE));
        builder.extended(0xCD, Self::new(H, BitPosition::ONE));
        builder.extended(0xCF, Self::new(L, BitPosition::ONE));

        // SET 1, (HL)
        builder.extended(0xCE, Self::new(Pointer(Pair::HL), BitPosition::ONE));

        // SET 2, r8
        builder.extended(0xD0, Self::new(A, BitPosition::TWO));
        builder.extended(0xD1, Self::new(B, BitPosition::TWO));
        builder.extended(0xD2, Self::new(C, BitPosition::TWO));
        builder.extended(0xD3, Self::new(D, BitPosition::TWO));
        builder.extended(0xD4, Self::new(E, BitPosition::TWO));
        builder.extended(0xD5, Self::new(H, BitPosition::TWO));
        builder.extended(0xD7, Self::new(L, BitPosition::TWO));

        // SET 2, (HL)
        builder.extended(0xD6, Self::new(Pointer(Pair::HL), BitPosition::TWO));

        // SET 3, r8
        builder.extended(0xD8, Self::new(A, BitPosition::THREE));
        builder.extended(0xD9, Self::new(B, BitPosition::THREE));
        builder.extended(0xDA, Self::new(C, BitPosition::THREE));
        builder.extended(0xDB, Self::new(D, BitPosition::THREE));
        builder.extended(0xDC, Self::new(E, BitPosition::THREE));
        builder.extended(0xDD, Self::new(H, BitPosition::THREE));
        builder.extended(0xDF, Self::new(L, BitPosition::THREE));

        // SET 3, (HL)
        builder.extended(0xDE, Self::new(Pointer(Pair::HL), BitPosition::THREE));

        // SET 4, r8
        builder.extended(0xE0, Self::new(A, BitPosition::FOUR));
        builder.extended(0xE1, Self::new(B, BitPosition::FOUR));
        builder.extended(0xE2, Self::new(C, BitPosition::FOUR));
        builder.extended(0xE3, Self::new(D, BitPosition::FOUR));
        builder.extended(0xE4, Self::new(E, BitPosition::FOUR));
        builder.extended(0xE5, Self::new(H, BitPosition::FOUR));
        builder.extended(0xE7, Self::new(L, BitPosition::FOUR));

        // SET 4, (HL)
        builder.extended(0xE6, Self::new(Pointer(Pair::HL), BitPosition::FOUR));

        // SET 5, r8
        builder.extended(0xE8, Self::new(A, BitPosition::FIVE));
        builder.extended(0xE9, Self::new(B, BitPosition::FIVE));
        builder.extended(0xEA, Self::new(C, BitPosition::FIVE));
        builder.extended(0xEB, Self::new(D, BitPosition::FIVE));
        builder.extended(0xEC, Self::new(E, BitPosition::FIVE));
        builder.extended(0xED, Self::new(H, BitPosition::FIVE));
        builder.extended(0xEF, Self::new(L, BitPosition::FIVE));

        // SET 5, (HL)
        builder.extended(0xEE, Self::new(Pointer(Pair::HL), BitPosition::FIVE));

        // SET 6, r8
        builder.extended(0xF0, Self::new(A, BitPosition::SIX));
        builder.extended(0xF1, Self::new(B, BitPosition::SIX));
        builder.extended(0xF2, Self::new(C, BitPosition::SIX));
        builder.extended(0xF3, Self::new(D, BitPosition::SIX));
        builder.extended(0xF4, Self::new(E, BitPosition::SIX));
        builder.extended(0xF5, Self::new(H, BitPosition::SIX));
        builder.extended(0xF7, Self::new(L, BitPosition::SIX));

        // SET 6, (HL)
        builder.extended(0xF6, Self::new(Pointer(Pair::HL), BitPosition::SIX));

        // SET 7, r8
        builder.extended(0xF8, Self::new(A, BitPosition::SEVEN));
        builder.extended(0xF9, Self::new(B, BitPosition::SEVEN));
        builder.extended(0xFA, Self::new(C, BitPosition::SEVEN));
        builder.extended(0xFB, Self::new(D, BitPosition::SEVEN));
        builder.extended(0xFC, Self::new(E, BitPosition::SEVEN));
        builder.extended(0xFD, Self::new(H, BitPosition::SEVEN));
        builder.extended(0xFF, Self::new(L, BitPosition::SEVEN));

        // SET 7, (HL)
        builder.extended(0xFE, Self::new(Pointer(Pair::HL), BitPosition::SEVEN));
    }
}

impl const From<BitwiseSet> for Instruction {
    fn from(value: BitwiseSet) -> Self {
        Self::Bit(super::Bit::Set(value))
    }
}

impl From<BitwiseSet> for Operation {
    fn from(value: BitwiseSet) -> Self {
        Operation::Bit(op::Bit::Set(value))
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
