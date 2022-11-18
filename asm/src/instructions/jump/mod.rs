use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use relative::*;

mod relative;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Jump {
    Relative(RelativeJump),
}

impl Parse for Jump {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Relative(inner))
    }
}

impl const SetRegister for Jump {
    fn register(builder: &mut Builder) {
        register_helper!(builder, RelativeJump);
    }
}
