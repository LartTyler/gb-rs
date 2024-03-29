use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use pair::*;
pub use register::*;

mod pair;
mod register;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Add {
    Pair(PairAdd),
    Register(RegisterAdd),
}

impl Parse for Add {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Pair(inner), Self::Register(inner))
    }
}

impl const SetRegister for Add {
    fn register(builder: &mut Builder) {
        register_helper!(builder, PairAdd, RegisterAdd);
    }
}
