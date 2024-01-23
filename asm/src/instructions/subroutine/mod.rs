use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use call::*;
pub use ret::*;

mod call;
mod ret;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Subroutine {
    Return(Return),
    Call(Call),
}

impl SetRegister for Subroutine {
    fn register(builder: &mut Builder) {
        register_helper!(builder, Return, Call);
    }
}

impl Parse for Subroutine {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(self, data[offset], Self::Return(inner), Self::Call(inner))
    }
}
