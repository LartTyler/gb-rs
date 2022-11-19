use super::Bitwise;
use crate::containers::{Bit, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bitwise as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("RES {bit}, {target}")]
pub struct BitwiseReset {
    pub target: BitwiseResetTarget,
    pub bit: Bit,
}

impl BitwiseReset {
    pub const fn new<T>(target: T, bit: Bit) -> Self
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
        Self::Bitwise(Bitwise::Reset(value))
    }
}

impl From<BitwiseReset> for Operation {
    fn from(value: BitwiseReset) -> Self {
        Self::Bitwise(op::Bitwise::Reset(value))
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
        builder.extended(0x80, Self::new(A, Bit::ZERO));
        builder.extended(0x81, Self::new(B, Bit::ZERO));
        builder.extended(0x82, Self::new(C, Bit::ZERO));
        builder.extended(0x83, Self::new(D, Bit::ZERO));
        builder.extended(0x84, Self::new(E, Bit::ZERO));
        builder.extended(0x85, Self::new(H, Bit::ZERO));
        builder.extended(0x87, Self::new(L, Bit::ZERO));

        // RES 0, (HL)
        builder.extended(0x86, Self::new(Pointer(Pair::HL), Bit::ZERO));

        // RES 1, r8
        builder.extended(0x88, Self::new(A, Bit::ONE));
        builder.extended(0x89, Self::new(B, Bit::ONE));
        builder.extended(0x8A, Self::new(C, Bit::ONE));
        builder.extended(0x8B, Self::new(D, Bit::ONE));
        builder.extended(0x8C, Self::new(E, Bit::ONE));
        builder.extended(0x8D, Self::new(H, Bit::ONE));
        builder.extended(0x8F, Self::new(L, Bit::ONE));

        // RES 1, (HL)
        builder.extended(0x8E, Self::new(Pointer(Pair::HL), Bit::ONE));

        // RES 2, r8
        builder.extended(0x90, Self::new(A, Bit::TWO));
        builder.extended(0x91, Self::new(B, Bit::TWO));
        builder.extended(0x92, Self::new(C, Bit::TWO));
        builder.extended(0x93, Self::new(D, Bit::TWO));
        builder.extended(0x94, Self::new(E, Bit::TWO));
        builder.extended(0x95, Self::new(H, Bit::TWO));
        builder.extended(0x97, Self::new(L, Bit::TWO));

        // RES 2, (HL)
        builder.extended(0x96, Self::new(Pointer(Pair::HL), Bit::TWO));

        // RES 3, r8
        builder.extended(0x98, Self::new(A, Bit::THREE));
        builder.extended(0x99, Self::new(B, Bit::THREE));
        builder.extended(0x9A, Self::new(C, Bit::THREE));
        builder.extended(0x9B, Self::new(D, Bit::THREE));
        builder.extended(0x9C, Self::new(E, Bit::THREE));
        builder.extended(0x9D, Self::new(H, Bit::THREE));
        builder.extended(0x9F, Self::new(L, Bit::THREE));

        // RES 3, (HL)
        builder.extended(0x9E, Self::new(Pointer(Pair::HL), Bit::THREE));

        // RES 4, r8
        builder.extended(0xA0, Self::new(A, Bit::FOUR));
        builder.extended(0xA1, Self::new(B, Bit::FOUR));
        builder.extended(0xA2, Self::new(C, Bit::FOUR));
        builder.extended(0xA3, Self::new(D, Bit::FOUR));
        builder.extended(0xA4, Self::new(E, Bit::FOUR));
        builder.extended(0xA5, Self::new(H, Bit::FOUR));
        builder.extended(0xA7, Self::new(L, Bit::FOUR));

        // RES 4, (HL)
        builder.extended(0xA6, Self::new(Pointer(Pair::HL), Bit::FOUR));

        // RES 5, r8
        builder.extended(0xA8, Self::new(A, Bit::FIVE));
        builder.extended(0xA9, Self::new(B, Bit::FIVE));
        builder.extended(0xAA, Self::new(C, Bit::FIVE));
        builder.extended(0xAB, Self::new(D, Bit::FIVE));
        builder.extended(0xAC, Self::new(E, Bit::FIVE));
        builder.extended(0xAD, Self::new(H, Bit::FIVE));
        builder.extended(0xAF, Self::new(L, Bit::FIVE));

        // RES 5, (HL)
        builder.extended(0xAE, Self::new(Pointer(Pair::HL), Bit::FIVE));

        // RES 6, r8
        builder.extended(0xB0, Self::new(A, Bit::SIX));
        builder.extended(0xB1, Self::new(B, Bit::SIX));
        builder.extended(0xB2, Self::new(C, Bit::SIX));
        builder.extended(0xB3, Self::new(D, Bit::SIX));
        builder.extended(0xB4, Self::new(E, Bit::SIX));
        builder.extended(0xB5, Self::new(H, Bit::SIX));
        builder.extended(0xB7, Self::new(L, Bit::SIX));

        // RES 6, (HL)
        builder.extended(0xB6, Self::new(Pointer(Pair::HL), Bit::SIX));

        // RES 7, r8
        builder.extended(0xB8, Self::new(A, Bit::SEVEN));
        builder.extended(0xB9, Self::new(B, Bit::SEVEN));
        builder.extended(0xBA, Self::new(C, Bit::SEVEN));
        builder.extended(0xBB, Self::new(D, Bit::SEVEN));
        builder.extended(0xBC, Self::new(E, Bit::SEVEN));
        builder.extended(0xBD, Self::new(H, Bit::SEVEN));
        builder.extended(0xBF, Self::new(L, Bit::SEVEN));

        // RES 7, (HL)
        builder.extended(0xBE, Self::new(Pointer(Pair::HL), Bit::SEVEN));
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
