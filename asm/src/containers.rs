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
    Zero,

    #[display("S")]
    Subtract,

    #[display("H")]
    HalfCarry,

    #[display("C")]
    Carry,
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
    pub const fn new() -> Self {
        Self(PhantomData)
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

impl<T: UpperHex> UpperHex for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed<T>(T);

impl<T> Signed<Data<T>> {
    pub const fn new() -> Self {
        Self(Data::new())
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
        write!(f, "{}", *self.0 as i8)
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
    ( $( $( $is_const:tt )? $source:ty => $enum:ident :: $variant:ident $(,)? ),* ) => {
        $(
            impl $( $is_const )? From<$source> for $enum {
                fn from(value: $source) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}
