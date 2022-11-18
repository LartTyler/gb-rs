use crate::{parse, read::Read};
use parse_display::Display;
use std::{
    fmt::{Display, UpperHex},
    marker::PhantomData,
    ops::Deref,
};

#[derive(Debug, Display, Clone, Copy)]
#[display("{:?}")]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Display, Clone, Copy)]
#[display("{:?}")]
pub enum Pair {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Clone, Copy)]
pub struct Data<T>(PhantomData<T>);

impl<T> Data<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

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

#[derive(Debug, Display, Clone, Copy)]
#[display("{0}")]
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

#[derive(Debug, Clone, Copy)]
pub struct Signed<T>(T);

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

#[derive(Debug, Clone, Copy)]
pub struct Pointer<T>(T);

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
    Data<u8> => "($FF00 + {})",
    Data<u16> => "({})",
    Value<u8> => "($FF00 + {})",
    Value<u16> => "({})",
    Register => "($FF00 + {})",
    Pair => "({})"
);
