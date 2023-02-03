use super::Stack;
use crate::containers::Pair;
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::OperationKind;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("POP {target}")]
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

impl const From<PopStack> for InstructionKind {
    fn from(value: PopStack) -> Self {
        Self::Stack(Stack::Pop(value))
    }
}

impl From<PopStack> for OperationKind {
    fn from(value: PopStack) -> Self {
        Self::Stack(Stack::Pop(value))
    }
}

impl const SetRegister for PopStack {
    fn register(builder: &mut Builder) {
        use Pair::*;

        builder.base(0xC1, Self::new(BC), 1, 3);
        builder.base(0xD1, Self::new(DE), 1, 3);
        builder.base(0xE1, Self::new(HL), 1, 3);
        builder.base(0xF1, Self::new(PopStackTarget::AccumulatorAndFlags), 1, 3);
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
