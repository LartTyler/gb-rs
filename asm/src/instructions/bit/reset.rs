use crate::containers::{BitPosition, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bit as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("RES {bit}, {target}")]
pub struct BitwiseReset {
    pub target: BitwiseResetTarget,
    pub bit: BitPosition,
}

impl BitwiseReset {
    pub const fn new<T>(target: T, bit: BitPosition) -> Self
    where
        T: ~const Into<BitwiseResetTarget>,
    {
        Self {
            target: target.into(),
            bit,
        }
    }
}

impl const From<BitwiseReset> for Instruction {
    fn from(value: BitwiseReset) -> Self {
        Self::Bit(super::Bit::Reset(value))
    }
}

impl From<BitwiseReset> for Operation {
    fn from(value: BitwiseReset) -> Self {
        Self::Bit(op::Bit::Reset(value))
    }
}

impl Parse for BitwiseReset {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for BitwiseReset {
    fn register(builder: &mut Builder) {
        use Register::*;

        // RES 0, r8
        builder.extended(0x80, Self::new(A, BitPosition::ZERO));
        builder.extended(0x81, Self::new(B, BitPosition::ZERO));
        builder.extended(0x82, Self::new(C, BitPosition::ZERO));
        builder.extended(0x83, Self::new(D, BitPosition::ZERO));
        builder.extended(0x84, Self::new(E, BitPosition::ZERO));
        builder.extended(0x85, Self::new(H, BitPosition::ZERO));
        builder.extended(0x87, Self::new(L, BitPosition::ZERO));

        // RES 0, (HL)
        builder.extended(0x86, Self::new(Pointer(Pair::HL), BitPosition::ZERO));

        // RES 1, r8
        builder.extended(0x88, Self::new(A, BitPosition::ONE));
        builder.extended(0x89, Self::new(B, BitPosition::ONE));
        builder.extended(0x8A, Self::new(C, BitPosition::ONE));
        builder.extended(0x8B, Self::new(D, BitPosition::ONE));
        builder.extended(0x8C, Self::new(E, BitPosition::ONE));
        builder.extended(0x8D, Self::new(H, BitPosition::ONE));
        builder.extended(0x8F, Self::new(L, BitPosition::ONE));

        // RES 1, (HL)
        builder.extended(0x8E, Self::new(Pointer(Pair::HL), BitPosition::ONE));

        // RES 2, r8
        builder.extended(0x90, Self::new(A, BitPosition::TWO));
        builder.extended(0x91, Self::new(B, BitPosition::TWO));
        builder.extended(0x92, Self::new(C, BitPosition::TWO));
        builder.extended(0x93, Self::new(D, BitPosition::TWO));
        builder.extended(0x94, Self::new(E, BitPosition::TWO));
        builder.extended(0x95, Self::new(H, BitPosition::TWO));
        builder.extended(0x97, Self::new(L, BitPosition::TWO));

        // RES 2, (HL)
        builder.extended(0x96, Self::new(Pointer(Pair::HL), BitPosition::TWO));

        // RES 3, r8
        builder.extended(0x98, Self::new(A, BitPosition::THREE));
        builder.extended(0x99, Self::new(B, BitPosition::THREE));
        builder.extended(0x9A, Self::new(C, BitPosition::THREE));
        builder.extended(0x9B, Self::new(D, BitPosition::THREE));
        builder.extended(0x9C, Self::new(E, BitPosition::THREE));
        builder.extended(0x9D, Self::new(H, BitPosition::THREE));
        builder.extended(0x9F, Self::new(L, BitPosition::THREE));

        // RES 3, (HL)
        builder.extended(0x9E, Self::new(Pointer(Pair::HL), BitPosition::THREE));

        // RES 4, r8
        builder.extended(0xA0, Self::new(A, BitPosition::FOUR));
        builder.extended(0xA1, Self::new(B, BitPosition::FOUR));
        builder.extended(0xA2, Self::new(C, BitPosition::FOUR));
        builder.extended(0xA3, Self::new(D, BitPosition::FOUR));
        builder.extended(0xA4, Self::new(E, BitPosition::FOUR));
        builder.extended(0xA5, Self::new(H, BitPosition::FOUR));
        builder.extended(0xA7, Self::new(L, BitPosition::FOUR));

        // RES 4, (HL)
        builder.extended(0xA6, Self::new(Pointer(Pair::HL), BitPosition::FOUR));

        // RES 5, r8
        builder.extended(0xA8, Self::new(A, BitPosition::FIVE));
        builder.extended(0xA9, Self::new(B, BitPosition::FIVE));
        builder.extended(0xAA, Self::new(C, BitPosition::FIVE));
        builder.extended(0xAB, Self::new(D, BitPosition::FIVE));
        builder.extended(0xAC, Self::new(E, BitPosition::FIVE));
        builder.extended(0xAD, Self::new(H, BitPosition::FIVE));
        builder.extended(0xAF, Self::new(L, BitPosition::FIVE));

        // RES 5, (HL)
        builder.extended(0xAE, Self::new(Pointer(Pair::HL), BitPosition::FIVE));

        // RES 6, r8
        builder.extended(0xB0, Self::new(A, BitPosition::SIX));
        builder.extended(0xB1, Self::new(B, BitPosition::SIX));
        builder.extended(0xB2, Self::new(C, BitPosition::SIX));
        builder.extended(0xB3, Self::new(D, BitPosition::SIX));
        builder.extended(0xB4, Self::new(E, BitPosition::SIX));
        builder.extended(0xB5, Self::new(H, BitPosition::SIX));
        builder.extended(0xB7, Self::new(L, BitPosition::SIX));

        // RES 6, (HL)
        builder.extended(0xB6, Self::new(Pointer(Pair::HL), BitPosition::SIX));

        // RES 7, r8
        builder.extended(0xB8, Self::new(A, BitPosition::SEVEN));
        builder.extended(0xB9, Self::new(B, BitPosition::SEVEN));
        builder.extended(0xBA, Self::new(C, BitPosition::SEVEN));
        builder.extended(0xBB, Self::new(D, BitPosition::SEVEN));
        builder.extended(0xBC, Self::new(E, BitPosition::SEVEN));
        builder.extended(0xBD, Self::new(H, BitPosition::SEVEN));
        builder.extended(0xBF, Self::new(L, BitPosition::SEVEN));

        // RES 7, (HL)
        builder.extended(0xBE, Self::new(Pointer(Pair::HL), BitPosition::SEVEN));
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseResetTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for BitwiseResetTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for BitwiseResetTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
