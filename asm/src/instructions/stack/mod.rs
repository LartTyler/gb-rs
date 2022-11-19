use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use pop::*;

mod pop;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Stack {
    Pop(PopStack),
}

impl const SetRegister for Stack {
    fn register(builder: &mut Builder) {
        register_helper!(builder, PopStack);
    }
}

impl Parse for Stack {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Pop(inner))
    }
}
