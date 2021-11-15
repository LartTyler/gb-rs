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
