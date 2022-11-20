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
#[display("PUSH {target}")]
pub struct PushStack {
    pub target: PushStackTarget,
}

impl PushStack {
    pub const fn new<T>(target: T) -> Self
    where
        T: ~const Into<PushStackTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl const From<PushStack> for Instruction {
    fn from(value: PushStack) -> Self {
        Self::Stack(Stack::Push(value))
    }
}

impl From<PushStack> for Operation {
    fn from(value: PushStack) -> Self {
        Self::Stack(Stack::Push(value))
    }
}

impl const SetRegister for PushStack {
    fn register(builder: &mut Builder) {
        use Pair::*;

        builder.base(0xC5, Self::new(BC));
        builder.base(0xD5, Self::new(DE));
        builder.base(0xE5, Self::new(HL));
        builder.base(0xF5, Self::new(PushStackTarget::AccumulatorAndFlags));
    }
}

impl Parse for PushStack {
    fn parse<R: Read>(&self, _data: &R, _offset: u16) -> ParseResult {
        Ok((*self).into())
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum PushStackTarget {
    #[display("{0}")]
    Pair(Pair),

    #[display("AF")]
    AccumulatorAndFlags,
}

enum_from_helper!(const Pair => PushStackTarget::Pair);
