use super::SetRegister;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use crate::{parse_helper, register_helper};
use parse_display::Display;

pub use carrying::*;
pub use cyclic::*;

mod carrying;
mod cyclic;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum RotateRight {
    Cyclic(CyclicRotateRight),
    Carrying(CarryingRotateRight),
}

impl Parse for RotateRight {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Cyclic(inner),
            Self::Carrying(inner)
        )
    }
}

impl const SetRegister for RotateRight {
    fn register(builder: &mut Builder) {
        register_helper!(builder, CyclicRotateRight, CarryingRotateRight);
    }
}
