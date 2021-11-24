/// "Splits" a `u16` into a byte pair.
///
/// The returned tuple contains the values in the order `(low, high)`.
pub fn word_to_bytes(value: u16) -> (u8, u8) {
    (value as u8, (value >> 8) as u8)
}

/// "Combines" two bytes into a 16-bit (word) representation.
pub fn bytes_to_word(low: u8, high: u8) -> u16 {
    ((high as u16) << 8) | low as u16
}

/// A "half carry" occurs during an add if there is a carry from bit 3 to 4 (i.e. from the low
/// nibble to the high nibble).
///
/// In the original hardware, the ALU actually performs two 4 bit operations for addition, which
/// use the half carry flag to widen the operation to 8 bits.
pub fn is_half_carry_add(value: u8, operand: u8) -> bool {
    ((value & 0xF) + (operand & 0xF)) & 0x10 > 0
}

/// A "half carry" occurs during subtraction if there is a borrow from bit 4 to 3 (i.e. from the high
/// nibble to the low nibble).
///
/// In the original hardware, the ALU actually performs two 4 bit operations for subtraction, which
/// use the half carry flag to widen the operation to 8 bits.
pub fn is_half_carry_sub(value: u8, operand: u8) -> bool {
    ((value & 0xF) - (operand & 0xF)) & 0x10 > 0
}
