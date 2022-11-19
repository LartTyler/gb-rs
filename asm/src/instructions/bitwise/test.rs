use super::Bitwise;
use crate::containers::{Bit, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{bitwise as op, Operation};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("BIT {bit}, {target}")]
pub struct BitwiseTest {
    pub target: BitwiseTestTarget,
    pub bit: Bit,
}

impl BitwiseTest {
    pub const fn new<T>(target: T, bit: Bit) -> Self
    where
        T: ~const Into<BitwiseTestTarget>,
    {
        Self {
            target: target.into(),
            bit,
        }
    }
}

impl const From<BitwiseTest> for Instruction {
    fn from(value: BitwiseTest) -> Self {
        Self::Bitwise(Bitwise::Test(value))
    }
}

impl From<BitwiseTest> for Operation {
    fn from(value: BitwiseTest) -> Self {
        Operation::Bitwise(op::Bitwise::Test(value))
    }
}

impl Parse for BitwiseTest {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

impl const SetRegister for BitwiseTest {
    fn register(builder: &mut Builder) {
        use Register::*;

        // BIT 0, r8
        builder.extended(0x40, Self::new(A, Bit::ZERO));
        builder.extended(0x41, Self::new(B, Bit::ZERO));
        builder.extended(0x42, Self::new(C, Bit::ZERO));
        builder.extended(0x43, Self::new(D, Bit::ZERO));
        builder.extended(0x44, Self::new(E, Bit::ZERO));
        builder.extended(0x45, Self::new(H, Bit::ZERO));
        builder.extended(0x47, Self::new(L, Bit::ZERO));

        // BIT 0, (HL)
        builder.extended(0x46, Self::new(Pointer(Pair::HL), Bit::ZERO));

        // BIT 1, r8
        builder.extended(0x48, Self::new(A, Bit::ONE));
        builder.extended(0x49, Self::new(B, Bit::ONE));
        builder.extended(0x4A, Self::new(C, Bit::ONE));
        builder.extended(0x4B, Self::new(D, Bit::ONE));
        builder.extended(0x4C, Self::new(E, Bit::ONE));
        builder.extended(0x4D, Self::new(H, Bit::ONE));
        builder.extended(0x4F, Self::new(L, Bit::ONE));

        // BIT 1, (HL)
        builder.extended(0x4E, Self::new(Pointer(Pair::HL), Bit::ONE));

        // BIT 2, r8
        builder.extended(0x50, Self::new(A, Bit::TWO));
        builder.extended(0x51, Self::new(B, Bit::TWO));
        builder.extended(0x52, Self::new(C, Bit::TWO));
        builder.extended(0x53, Self::new(D, Bit::TWO));
        builder.extended(0x54, Self::new(E, Bit::TWO));
        builder.extended(0x55, Self::new(H, Bit::TWO));
        builder.extended(0x57, Self::new(L, Bit::TWO));

        // BIT 2, (HL)
        builder.extended(0x56, Self::new(Pointer(Pair::HL), Bit::TWO));

        // BIT 3, r8
        builder.extended(0x58, Self::new(A, Bit::THREE));
        builder.extended(0x59, Self::new(B, Bit::THREE));
        builder.extended(0x5A, Self::new(C, Bit::THREE));
        builder.extended(0x5B, Self::new(D, Bit::THREE));
        builder.extended(0x5C, Self::new(E, Bit::THREE));
        builder.extended(0x5D, Self::new(H, Bit::THREE));
        builder.extended(0x5F, Self::new(L, Bit::THREE));

        // BIT 3, (HL)
        builder.extended(0x5E, Self::new(Pointer(Pair::HL), Bit::THREE));

        // BIT 4, r8
        builder.extended(0x60, Self::new(A, Bit::FOUR));
        builder.extended(0x61, Self::new(B, Bit::FOUR));
        builder.extended(0x62, Self::new(C, Bit::FOUR));
        builder.extended(0x63, Self::new(D, Bit::FOUR));
        builder.extended(0x64, Self::new(E, Bit::FOUR));
        builder.extended(0x65, Self::new(H, Bit::FOUR));
        builder.extended(0x67, Self::new(L, Bit::FOUR));

        // BIT 4, (HL)
        builder.extended(0x66, Self::new(Pointer(Pair::HL), Bit::FOUR));

        // BIT 5, r8
        builder.extended(0x68, Self::new(A, Bit::FIVE));
        builder.extended(0x69, Self::new(B, Bit::FIVE));
        builder.extended(0x6A, Self::new(C, Bit::FIVE));
        builder.extended(0x6B, Self::new(D, Bit::FIVE));
        builder.extended(0x6C, Self::new(E, Bit::FIVE));
        builder.extended(0x6D, Self::new(H, Bit::FIVE));
        builder.extended(0x6F, Self::new(L, Bit::FIVE));

        // BIT 5, (HL)
        builder.extended(0x6E, Self::new(Pointer(Pair::HL), Bit::FIVE));

        // BIT 6, r8
        builder.extended(0x70, Self::new(A, Bit::SIX));
        builder.extended(0x71, Self::new(B, Bit::SIX));
        builder.extended(0x72, Self::new(C, Bit::SIX));
        builder.extended(0x73, Self::new(D, Bit::SIX));
        builder.extended(0x74, Self::new(E, Bit::SIX));
        builder.extended(0x75, Self::new(H, Bit::SIX));
        builder.extended(0x77, Self::new(L, Bit::SIX));

        // BIT 6, (HL)
        builder.extended(0x76, Self::new(Pointer(Pair::HL), Bit::SIX));

        // BIT 7, r8
        builder.extended(0x78, Self::new(A, Bit::SEVEN));
        builder.extended(0x79, Self::new(B, Bit::SEVEN));
        builder.extended(0x7A, Self::new(C, Bit::SEVEN));
        builder.extended(0x7B, Self::new(D, Bit::SEVEN));
        builder.extended(0x7C, Self::new(E, Bit::SEVEN));
        builder.extended(0x7D, Self::new(H, Bit::SEVEN));
        builder.extended(0x7F, Self::new(L, Bit::SEVEN));

        // BIT 7, (HL)
        builder.extended(0x7E, Self::new(Pointer(Pair::HL), Bit::SEVEN));
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum BitwiseTestTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
}

impl const From<Register> for BitwiseTestTarget {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for BitwiseTestTarget {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}
