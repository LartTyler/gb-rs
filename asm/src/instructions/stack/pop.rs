use super::Stack;
use crate::containers::Pair;
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
pub struct PopStack {
    pub target: PopStackTarget,
}

impl PopStack {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<PopStackTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<PopStack> for Instruction {
    fn from(value: PopStack) -> Self {
        Self::Stack(Stack::Pop(value))
    }
}

impl From<PopStack> for Operation {
    fn from(value: PopStack) -> Self {
        Self::Stack(Stack::Pop(value))
    }
}

impl const SetRegister for PopStack {
    fn register(builder: &mut Builder) {
        use Pair::*;

        builder.base(0xC1, Self::new(BC));
        builder.base(0xD1, Self::new(DE));
        builder.base(0xE1, Self::new(HL));
        builder.base(0xF1, Self::new(PopStackTarget::AccumulatorAndFlags));
    }
}

impl Parse for PopStack {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum PopStackTarget {
    #[display("{0}")]
    Pair(Pair),

    #[display("AF")]
    AccumulatorAndFlags,
}

enum_from_helper!(const Pair => PopStackTarget::Pair);
