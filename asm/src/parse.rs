use crate::{operations::Operation, read};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Parse {
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> Result<Operation>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("read error: {0}")]
    Read(#[from] read::Error),

    #[error("unknown opcode {0:#02?}")]
    UnknownOpcode(u8),
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
