use super::Load;
use crate::instructions::{InstructionKind, SetRegister};
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("LD (C), A")]
pub struct RegisterPointerLoad;

impl From<RegisterPointerLoad> for InstructionKind {
    fn from(value: RegisterPointerLoad) -> Self {
        Self::Load(Load::RegisterPointer(value))
    }
}

impl SetRegister for RegisterPointerLoad {
    fn register(builder: &mut Builder) {
        builder.base(0xE2, Self, 2, 2);
    }
}

impl Parse for RegisterPointerLoad {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}
