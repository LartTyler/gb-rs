use crate::{operations::OperationKind, read};
use parse_display::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type ParseResult = Result<OperationKind>;

/// Trait implemented by instructions to facilitate conversion to an [Operation].
pub trait Parse {
    /// Parse zero or more bytes from a data source. The `offset` argument should always be
    /// pointing to the byte _after_ the instruction byte.
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> ParseResult;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("read error: {0}")]
    Read(#[from] read::Error),

    #[error("unknown opcode {0}")]
    UnknownOpcode(Opcode),
}

#[derive(Debug, Display)]
pub enum Opcode {
    #[display("{0:#04X}")]
    Base(u8),

    #[display("0xCB {0:#04X}")]
    Extended(u8),
}

#[macro_export(local_inner_macros)]
macro_rules! parse_helper {
    ( $self:ident , $data:ident [ $offset:ident ] , $( $enum:ident :: $variant:ident $( ( $inner:ident ) )? $( => $rhs:expr )? $(,)? ),* ) => {
        match $self {
            $( $enum::$variant $( ($inner) )? => parse_match_arm_rhs!($data[$offset], $( ($inner) )? $( $rhs )?) ),*
        }
    };
}

#[macro_export]
macro_rules! parse_match_arm_rhs {
    ( $data:ident [ $offset:ident ] , ( $inner:ident ) $rhs:expr ) => {
        Ok($rhs)
    };

    ( $data:ident [ $offset:ident ] , ( $inner:ident ) ) => {
        $inner.parse($data, $offset)
    };

    ( $data:ident [ $offset:ident ] , $rhs:expr ) => {
        Ok($rhs)
    };
}
