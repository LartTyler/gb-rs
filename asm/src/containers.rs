use crate::{parse, read::Read};
use parse_display::Display;
use std::fmt::{Display, UpperHex};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[display("{}")]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[display("{}")]
pub enum Pair {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    #[display("Z")]
    Zero = 0x80,

    #[display("S")]
    Subtract = 0x40,

    #[display("H")]
    HalfCarry = 0x20,

    #[display("C")]
    Carry = 0x10,
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
pub enum Condition {
    #[display("")]
    Always,

    #[display("{0}")]
    Set(Flag),

    #[display("N{0}")]
    Unset(Flag),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data<T>(PhantomData<T>);

impl<T> Data<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Data<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub type ByteData = Data<u8>;
pub type WordData = Data<u16>;

impl Data<u8> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Value<u8>> {
        Ok(data.read_byte(offset)?.into())
    }
}

impl Data<u16> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Value<u16>> {
        Ok(data.read_word(offset)?.into())
    }
}

impl Display for Data<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d8")
    }
}

impl Display for Data<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d16")
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub struct Value<T>(pub T);

impl<T> Deref for Value<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for Value<u8> {
    fn from(n: u8) -> Self {
        Self(n)
    }
}

impl From<u16> for Value<u16> {
    fn from(n: u16) -> Self {
        Self(n)
    }
}

impl From<Value<u8>> for u8 {
    fn from(value: Value<u8>) -> Self {
        *value
    }
}

impl From<Value<u16>> for u16 {
    fn from(value: Value<u16>) -> Self {
        *value
    }
}

impl<T: UpperHex> UpperHex for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed<T>(T);

impl<T> Signed<Data<T>> {
    pub fn new() -> Self {
        Self(Data::new())
    }
}

impl<T> Default for Signed<Data<T>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Signed<Value<u8>> {
    /// Returns the underlying `u8` that represents this signed value.
    ///
    /// The Z80 encodes signed values as two's complement. Instead of converting the `u8` to an
    /// `i8`, it can sometimes be more convenient to work with the raw two's complement `u8`
    /// instead, e.g. in the case that you need to
    /// [add a signed value to an unsigned value](https://en.wikipedia.org/wiki/Two%27s_complement#Addition).
    pub fn as_twos_complement(&self) -> u8 {
        *self.0
    }
}

impl Signed<Data<u8>> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Signed<Value<u8>>> {
        Ok(Signed(data.read_byte(offset)?.into()))
    }
}

impl Signed<Data<u16>> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Signed<Value<u16>>> {
        Ok(Signed(data.read_word(offset)?.into()))
    }
}

impl Display for Signed<Data<u8>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s8")
    }
}

impl Display for Signed<Value<u8>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: i8 = (*self).into();
        write!(f, "{}", val)
    }
}

impl From<Signed<Value<u8>>> for i8 {
    fn from(value: Signed<Value<u8>>) -> Self {
        *value.0 as i8
    }
}

impl From<Signed<Value<u8>>> for i16 {
    fn from(value: Signed<Value<u8>>) -> Self {
        (*value.0 as i8).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer<T>(pub T);

impl Pointer<Data<u16>> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Pointer<Value<u16>>> {
        Ok(Pointer(data.read_word(offset)?.into()))
    }
}

impl Pointer<Data<u8>> {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Pointer<Value<u8>>> {
        Ok(Pointer(data.read_byte(offset)?.into()))
    }
}

impl<T> Deref for Pointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Pointer<Value<u8>>> for u8 {
    fn from(value: Pointer<Value<u8>>) -> Self {
        (*value).into()
    }
}

impl From<Pointer<Value<u8>>> for u16 {
    fn from(value: Pointer<Value<u8>>) -> Self {
        (**value).into()
    }
}

impl From<Pointer<Value<u16>>> for u16 {
    fn from(value: Pointer<Value<u16>>) -> Self {
        (*value).into()
    }
}

macro_rules! pointer_display_helper {
    ( $( $inner:ty => $pattern:literal $(,)? ),* ) => {
        $(
            impl Display for Pointer<$inner> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, $pattern, self.0)
                }
            }
        )*
    };
}

pointer_display_helper!(
    u16 => "${:04X}",
    Data<u8> => "($FF00 + {})",
    Data<u16> => "({})",
    Value<u8> => "($FF00 + {:#04X})",
    Value<u16> => "(${:04X})",
    Register => "($FF00 + {})",
    Pair => "({})",
);

/// Represents a bit position in a byte. The contained value will _always_ be between 0 and 7,
/// inclusive.
#[derive(Debug, Clone, Copy, Display)]
pub struct BitPosition(u8);

impl BitPosition {
    pub fn with_set(&self, value: u8) -> u8 {
        value | (1 << self.0)
    }

    pub fn with_unset(&self, value: u8) -> u8 {
        value & !(1 << self.0)
    }

    pub fn is_set(&self, value: u8) -> bool {
        value & (1 << self.0) > 0
    }
}

macro_rules! bit_position_helper {
    ( $( $bit:expr => $name:ident $(,)? ),* ) => {
        $(
            pub const $name: BitPosition = BitPosition($bit);
        )*
    };
}

impl BitPosition {
    bit_position_helper!(
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
    );
}

impl Deref for BitPosition {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! enum_from_helper {
    ( $( $source:ty => $enum:ident :: $variant:ident $(,)? ),* ) => {
        $(
            impl From<$source> for $enum {
                fn from(value: $source) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, Display)]
pub enum Cycles {
    #[display("{0}")]
    Fixed(u8),

    #[display("{min}/{max}")]
    Variable { min: u8, max: u8 },
}

impl Cycles {
    /// Extracts the contained type using the value of a `toggle`, usually from the result of
    /// comparing a CPU flag against a [`Condition`].
    ///
    /// If [`Cycles::Fixed`], the wrapped value is always returned.
    ///
    /// If [`Cycles::Variable`], then:
    /// - If the toggle is `true`, return the `max` value (the branch was taken)
    /// - If the toggle is `false`, return the `min` value (the branch was NOT taken)
    pub fn with_toggle(self, toggle: bool) -> u8 {
        match self {
            Self::Fixed(n) => n,
            Self::Variable { min, max } => {
                if toggle {
                    max
                } else {
                    min
                }
            }
        }
    }
}

impl From<u8> for Cycles {
    fn from(value: u8) -> Self {
        Self::Fixed(value)
    }
}

impl From<(u8, u8)> for Cycles {
    fn from(value: (u8, u8)) -> Self {
        Self::Variable {
            min: value.0,
            max: value.1,
        }
    }
}

/// Attempts to convert a [`Cycles`] back into a `u8`. Conversion fails only if [`Cycles`] is not
/// [`Cycles::Fixed`].
///
/// This is intended as a convenience for operations that never branch, and can reasonably call
/// [`Result::expect()`] on the returned value.
impl TryFrom<Cycles> for u8 {
    type Error = ();

    fn try_from(value: Cycles) -> Result<Self, Self::Error> {
        use Cycles::*;

        match value {
            Fixed(n) => Ok(n),
            _ => Err(()),
        }
    }
}

/// Attempts to convert a [`Cycles`] back into a `(u8, u8)`. Conversion fails only if [`Cycles`] is
/// not [`Cycles::Variable`].
///
/// This is intended as a convenience for operations that always branch, and can reasonably call
/// [`Result::expect()`] on the returned value.
impl TryFrom<Cycles> for (u8, u8) {
    type Error = ();

    fn try_from(value: Cycles) -> Result<Self, Self::Error> {
        use Cycles::*;

        match value {
            Variable { min, max } => Ok((min, max)),
            _ => Err(()),
        }
    }
}
