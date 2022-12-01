use crate::containers::{BitPosition, Pair, Pointer, Register};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::{bit as op, OperationKind};
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
        builder.extended(0xC0, Self::new(A, BitPosition::ZERO), 2, 2);
        builder.extended(0xC1, Self::new(B, BitPosition::ZERO), 2, 2);
        builder.extended(0xC2, Self::new(C, BitPosition::ZERO), 2, 2);
        builder.extended(0xC3, Self::new(D, BitPosition::ZERO), 2, 2);
        builder.extended(0xC4, Self::new(E, BitPosition::ZERO), 2, 2);
        builder.extended(0xC5, Self::new(H, BitPosition::ZERO), 2, 2);
        builder.extended(0xC7, Self::new(L, BitPosition::ZERO), 2, 2);

        // SET 0, (HL)
        builder.extended(0xC6, Self::new(Pointer(Pair::HL), BitPosition::ZERO), 2, 4);

        // SET 1, r8
        builder.extended(0xC8, Self::new(A, BitPosition::ONE), 2, 2);
        builder.extended(0xC9, Self::new(B, BitPosition::ONE), 2, 2);
        builder.extended(0xCA, Self::new(C, BitPosition::ONE), 2, 2);
        builder.extended(0xCB, Self::new(D, BitPosition::ONE), 2, 2);
        builder.extended(0xCC, Self::new(E, BitPosition::ONE), 2, 2);
        builder.extended(0xCD, Self::new(H, BitPosition::ONE), 2, 2);
        builder.extended(0xCF, Self::new(L, BitPosition::ONE), 2, 2);

        // SET 1, (HL)
        builder.extended(0xCE, Self::new(Pointer(Pair::HL), BitPosition::ONE), 2, 4);

        // SET 2, r8
        builder.extended(0xD0, Self::new(A, BitPosition::TWO), 2, 2);
        builder.extended(0xD1, Self::new(B, BitPosition::TWO), 2, 2);
        builder.extended(0xD2, Self::new(C, BitPosition::TWO), 2, 2);
        builder.extended(0xD3, Self::new(D, BitPosition::TWO), 2, 2);
        builder.extended(0xD4, Self::new(E, BitPosition::TWO), 2, 2);
        builder.extended(0xD5, Self::new(H, BitPosition::TWO), 2, 2);
        builder.extended(0xD7, Self::new(L, BitPosition::TWO), 2, 2);

        // SET 2, (HL)
        builder.extended(0xD6, Self::new(Pointer(Pair::HL), BitPosition::TWO), 2, 4);

        // SET 3, r8
        builder.extended(0xD8, Self::new(A, BitPosition::THREE), 2, 2);
        builder.extended(0xD9, Self::new(B, BitPosition::THREE), 2, 2);
        builder.extended(0xDA, Self::new(C, BitPosition::THREE), 2, 2);
        builder.extended(0xDB, Self::new(D, BitPosition::THREE), 2, 2);
        builder.extended(0xDC, Self::new(E, BitPosition::THREE), 2, 2);
        builder.extended(0xDD, Self::new(H, BitPosition::THREE), 2, 2);
        builder.extended(0xDF, Self::new(L, BitPosition::THREE), 2, 2);

        // SET 3, (HL)
        builder.extended(0xDE, Self::new(Pointer(Pair::HL), BitPosition::THREE), 2, 4);

        // SET 4, r8
        builder.extended(0xE0, Self::new(A, BitPosition::FOUR), 2, 2);
        builder.extended(0xE1, Self::new(B, BitPosition::FOUR), 2, 2);
        builder.extended(0xE2, Self::new(C, BitPosition::FOUR), 2, 2);
        builder.extended(0xE3, Self::new(D, BitPosition::FOUR), 2, 2);
        builder.extended(0xE4, Self::new(E, BitPosition::FOUR), 2, 2);
        builder.extended(0xE5, Self::new(H, BitPosition::FOUR), 2, 2);
        builder.extended(0xE7, Self::new(L, BitPosition::FOUR), 2, 2);

        // SET 4, (HL)
        builder.extended(0xE6, Self::new(Pointer(Pair::HL), BitPosition::FOUR), 2, 4);

        // SET 5, r8
        builder.extended(0xE8, Self::new(A, BitPosition::FIVE), 2, 2);
        builder.extended(0xE9, Self::new(B, BitPosition::FIVE), 2, 2);
        builder.extended(0xEA, Self::new(C, BitPosition::FIVE), 2, 2);
        builder.extended(0xEB, Self::new(D, BitPosition::FIVE), 2, 2);
        builder.extended(0xEC, Self::new(E, BitPosition::FIVE), 2, 2);
        builder.extended(0xED, Self::new(H, BitPosition::FIVE), 2, 2);
        builder.extended(0xEF, Self::new(L, BitPosition::FIVE), 2, 2);

        // SET 5, (HL)
        builder.extended(0xEE, Self::new(Pointer(Pair::HL), BitPosition::FIVE), 2, 4);

        // SET 6, r8
        builder.extended(0xF0, Self::new(A, BitPosition::SIX), 2, 2);
        builder.extended(0xF1, Self::new(B, BitPosition::SIX), 2, 2);
        builder.extended(0xF2, Self::new(C, BitPosition::SIX), 2, 2);
        builder.extended(0xF3, Self::new(D, BitPosition::SIX), 2, 2);
        builder.extended(0xF4, Self::new(E, BitPosition::SIX), 2, 2);
        builder.extended(0xF5, Self::new(H, BitPosition::SIX), 2, 2);
        builder.extended(0xF7, Self::new(L, BitPosition::SIX), 2, 2);

        // SET 6, (HL)
        builder.extended(0xF6, Self::new(Pointer(Pair::HL), BitPosition::SIX), 2, 4);

        // SET 7, r8
        builder.extended(0xF8, Self::new(A, BitPosition::SEVEN), 2, 2);
        builder.extended(0xF9, Self::new(B, BitPosition::SEVEN), 2, 2);
        builder.extended(0xFA, Self::new(C, BitPosition::SEVEN), 2, 2);
        builder.extended(0xFB, Self::new(D, BitPosition::SEVEN), 2, 2);
        builder.extended(0xFC, Self::new(E, BitPosition::SEVEN), 2, 2);
        builder.extended(0xFD, Self::new(H, BitPosition::SEVEN), 2, 2);
        builder.extended(0xFF, Self::new(L, BitPosition::SEVEN), 2, 2);

        // SET 7, (HL)
        builder.extended(0xFE, Self::new(Pointer(Pair::HL), BitPosition::SEVEN), 2, 4);
    }
}

impl const From<BitwiseSet> for InstructionKind {
    fn from(value: BitwiseSet) -> Self {
        Self::Bit(super::Bit::Set(value))
    }
}

impl From<BitwiseSet> for OperationKind {
    fn from(value: BitwiseSet) -> Self {
        OperationKind::Bit(op::Bit::Set(value))
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
