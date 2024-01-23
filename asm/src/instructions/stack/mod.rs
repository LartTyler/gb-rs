use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use pop::*;
pub use push::*;

mod pop;
mod push;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Stack {
    Pop(PopStack),
    Push(PushStack),
}

impl SetRegister for Stack {
    fn register(builder: &mut Builder) {
        register_helper!(builder, PopStack, PushStack);
    }
}

impl Parse for Stack {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Pop(inner), Self::Push(inner))
    }
}
