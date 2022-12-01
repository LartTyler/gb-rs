use crate::containers::{BitPosition, Pair, Pointer, Register};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::{bit as op, OperationKind};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("BIT {bit}, {target}")]
pub struct BitwiseTest {
    pub target: BitwiseTestTarget,
    pub bit: BitPosition,
}

impl BitwiseTest {
    pub const fn new<T>(target: T, bit: BitPosition) -> Self
    where
        T: ~const Into<BitwiseTestTarget>,
    {
        Self {
            target: target.into(),
            bit,
        }
    }
}

impl const From<BitwiseTest> for InstructionKind {
    fn from(value: BitwiseTest) -> Self {
        Self::Bit(super::Bit::Test(value))
    }
}

impl From<BitwiseTest> for OperationKind {
    fn from(value: BitwiseTest) -> Self {
        OperationKind::Bit(op::Bit::Test(value))
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
        builder.extended(0x40, Self::new(A, BitPosition::ZERO), 2, 2);
        builder.extended(0x41, Self::new(B, BitPosition::ZERO), 2, 2);
        builder.extended(0x42, Self::new(C, BitPosition::ZERO), 2, 2);
        builder.extended(0x43, Self::new(D, BitPosition::ZERO), 2, 2);
        builder.extended(0x44, Self::new(E, BitPosition::ZERO), 2, 2);
        builder.extended(0x45, Self::new(H, BitPosition::ZERO), 2, 2);
        builder.extended(0x47, Self::new(L, BitPosition::ZERO), 2, 2);

        // BIT 0, (HL)
        builder.extended(0x46, Self::new(Pointer(Pair::HL), BitPosition::ZERO), 2, 4);

        // BIT 1, r8
        builder.extended(0x48, Self::new(A, BitPosition::ONE), 2, 2);
        builder.extended(0x49, Self::new(B, BitPosition::ONE), 2, 2);
        builder.extended(0x4A, Self::new(C, BitPosition::ONE), 2, 2);
        builder.extended(0x4B, Self::new(D, BitPosition::ONE), 2, 2);
        builder.extended(0x4C, Self::new(E, BitPosition::ONE), 2, 2);
        builder.extended(0x4D, Self::new(H, BitPosition::ONE), 2, 2);
        builder.extended(0x4F, Self::new(L, BitPosition::ONE), 2, 2);

        // BIT 1, (HL)
        builder.extended(0x4E, Self::new(Pointer(Pair::HL), BitPosition::ONE), 2, 4);

        // BIT 2, r8
        builder.extended(0x50, Self::new(A, BitPosition::TWO), 2, 2);
        builder.extended(0x51, Self::new(B, BitPosition::TWO), 2, 2);
        builder.extended(0x52, Self::new(C, BitPosition::TWO), 2, 2);
        builder.extended(0x53, Self::new(D, BitPosition::TWO), 2, 2);
        builder.extended(0x54, Self::new(E, BitPosition::TWO), 2, 2);
        builder.extended(0x55, Self::new(H, BitPosition::TWO), 2, 2);
        builder.extended(0x57, Self::new(L, BitPosition::TWO), 2, 2);

        // BIT 2, (HL)
        builder.extended(0x56, Self::new(Pointer(Pair::HL), BitPosition::TWO), 2, 4);

        // BIT 3, r8
        builder.extended(0x58, Self::new(A, BitPosition::THREE), 2, 2);
        builder.extended(0x59, Self::new(B, BitPosition::THREE), 2, 2);
        builder.extended(0x5A, Self::new(C, BitPosition::THREE), 2, 2);
        builder.extended(0x5B, Self::new(D, BitPosition::THREE), 2, 2);
        builder.extended(0x5C, Self::new(E, BitPosition::THREE), 2, 2);
        builder.extended(0x5D, Self::new(H, BitPosition::THREE), 2, 2);
        builder.extended(0x5F, Self::new(L, BitPosition::THREE), 2, 2);

        // BIT 3, (HL)
        builder.extended(0x5E, Self::new(Pointer(Pair::HL), BitPosition::THREE), 2, 4);

        // BIT 4, r8
        builder.extended(0x60, Self::new(A, BitPosition::FOUR), 2, 2);
        builder.extended(0x61, Self::new(B, BitPosition::FOUR), 2, 2);
        builder.extended(0x62, Self::new(C, BitPosition::FOUR), 2, 2);
        builder.extended(0x63, Self::new(D, BitPosition::FOUR), 2, 2);
        builder.extended(0x64, Self::new(E, BitPosition::FOUR), 2, 2);
        builder.extended(0x65, Self::new(H, BitPosition::FOUR), 2, 2);
        builder.extended(0x67, Self::new(L, BitPosition::FOUR), 2, 2);

        // BIT 4, (HL)
        builder.extended(0x66, Self::new(Pointer(Pair::HL), BitPosition::FOUR), 2, 4);

        // BIT 5, r8
        builder.extended(0x68, Self::new(A, BitPosition::FIVE), 2, 2);
        builder.extended(0x69, Self::new(B, BitPosition::FIVE), 2, 2);
        builder.extended(0x6A, Self::new(C, BitPosition::FIVE), 2, 2);
        builder.extended(0x6B, Self::new(D, BitPosition::FIVE), 2, 2);
        builder.extended(0x6C, Self::new(E, BitPosition::FIVE), 2, 2);
        builder.extended(0x6D, Self::new(H, BitPosition::FIVE), 2, 2);
        builder.extended(0x6F, Self::new(L, BitPosition::FIVE), 2, 2);

        // BIT 5, (HL)
        builder.extended(0x6E, Self::new(Pointer(Pair::HL), BitPosition::FIVE), 2, 4);

        // BIT 6, r8
        builder.extended(0x70, Self::new(A, BitPosition::SIX), 2, 2);
        builder.extended(0x71, Self::new(B, BitPosition::SIX), 2, 2);
        builder.extended(0x72, Self::new(C, BitPosition::SIX), 2, 2);
        builder.extended(0x73, Self::new(D, BitPosition::SIX), 2, 2);
        builder.extended(0x74, Self::new(E, BitPosition::SIX), 2, 2);
        builder.extended(0x75, Self::new(H, BitPosition::SIX), 2, 2);
        builder.extended(0x77, Self::new(L, BitPosition::SIX), 2, 2);

        // BIT 6, (HL)
        builder.extended(0x76, Self::new(Pointer(Pair::HL), BitPosition::SIX), 2, 4);

        // BIT 7, r8
        builder.extended(0x78, Self::new(A, BitPosition::SEVEN), 2, 2);
        builder.extended(0x79, Self::new(B, BitPosition::SEVEN), 2, 2);
        builder.extended(0x7A, Self::new(C, BitPosition::SEVEN), 2, 2);
        builder.extended(0x7B, Self::new(D, BitPosition::SEVEN), 2, 2);
        builder.extended(0x7C, Self::new(E, BitPosition::SEVEN), 2, 2);
        builder.extended(0x7D, Self::new(H, BitPosition::SEVEN), 2, 2);
        builder.extended(0x7F, Self::new(L, BitPosition::SEVEN), 2, 2);

        // BIT 7, (HL)
        builder.extended(0x7E, Self::new(Pointer(Pair::HL), BitPosition::SEVEN), 2, 4);
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
