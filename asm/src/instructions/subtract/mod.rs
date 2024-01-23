use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use register::*;

mod register;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Subtract {
    Register(RegisterSubtract),
}

impl Parse for Subtract {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Register(inner))
    }
}

impl SetRegister for Subtract {
    fn register(builder: &mut crate::sets::Builder) {
        register_helper!(builder, RegisterSubtract);
    }
}
