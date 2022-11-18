use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use complement::*;

mod complement;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Bitwise {
    Complement(BitwiseComplement),
}

impl Parse for Bitwise {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Complement(inner))
    }
}

impl const SetRegister for Bitwise {
    fn register(builder: &mut Builder) {
        register_helper!(builder, BitwiseComplement);
    }
}
