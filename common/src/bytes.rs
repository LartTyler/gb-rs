use std::ops::{Add, BitAnd};

/// "Splits" a `u16` into a byte pair.
///
/// The returned tuple contains the values in the order `(high, low)`.
pub fn word_to_bytes(value: u16) -> [u8; 2] {
    value.to_be_bytes()
}

/// "Combines" two bytes into a 16-bit (word) representation.
pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    u16::from_be_bytes([high, low])
}

/// A "half carry" occurs during an add if there is a carry from bit 3 to 4 (i.e. from the low
/// nibble to the high nibble).
///
/// In the original hardware, the ALU actually performs two 4 bit operations for addition, which
/// use the half carry flag to widen the operation to 8 bits.
pub fn is_half_carry<T: RegisterValue>(lhs: T, rhs: T) -> bool {
    ((lhs & T::low_nibble_mask()) + (rhs & T::low_nibble_mask())) & T::half_carry_mask() > T::zero()
}

pub trait RegisterValue: Add<Output = Self> + BitAnd<Output = Self> + PartialOrd + Sized {
    fn low_nibble_mask() -> Self;
    fn half_carry_mask() -> Self;
    fn zero() -> Self;
}

impl RegisterValue for u8 {
    #[inline]
    fn low_nibble_mask() -> Self {
        0xF
    }

    #[inline]
    fn half_carry_mask() -> Self {
        0x10
    }

    #[inline]
    fn zero() -> Self {
        0
    }
}

impl RegisterValue for u16 {
    #[inline]
    fn low_nibble_mask() -> Self {
        0xF
    }

    #[inline]
    fn half_carry_mask() -> Self {
        0x10
    }

    #[inline]
    fn zero() -> Self {
        0
    }
}
