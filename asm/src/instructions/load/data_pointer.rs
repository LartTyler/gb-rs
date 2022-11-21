use super::Load;
use crate::containers::{ByteData, Data, Pair, Pointer, Register, WordData};
use crate::enum_from_helper;
use crate::instructions::{Instruction, SetRegister};
use crate::operations::load as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct DataPointerLoad {
    pub target: DataPointerLoadTarget,
    pub source: DataPointerLoadSource,
}

impl DataPointerLoad {
    pub const fn new<T, S>(target: T, source: S) -> Self
    where
        T: ~const Into<DataPointerLoadTarget>,
        S: ~const Into<DataPointerLoadSource>,
    {
        Self {
            target: target.into(),
            source: source.into(),
        }
    }
}

impl Display for DataPointerLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let DataPointerLoadTarget::High(p) = self.target {
            write!(f, "LDH {p}, {}", self.source)
        } else {
            write!(f, "LD {}, {}", self.target, self.source)
        }
    }
}

impl Parse for DataPointerLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use DataPointerLoadTarget::*;

        let target: op::DataPointerLoadTarget = match self.target {
            Absolute(p) => p.parse(data, offset)?.into(),
            High(p) => p.parse(data, offset)?.into(),
        };

        Ok(op::DataPointerLoad::create(target, self.source))
    }
}

impl const SetRegister for DataPointerLoad {
    fn register(builder: &mut crate::sets::Builder) {
        builder.base(0x08, Self::new(Pointer(WordData::new()), Pair::SP));
        builder.base(0xEA, Self::new(Pointer(WordData::new()), Register::A));

        builder.base(0xE0, Self::new(Pointer(ByteData::new()), Register::A));
    }
}

impl const From<DataPointerLoad> for Instruction {
    fn from(value: DataPointerLoad) -> Self {
        Self::Load(Load::DataPointer(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum DataPointerLoadSource {
    Register(Register),
    Pair(Pair),
}

enum_from_helper!(
    const Register => DataPointerLoadSource::Register,
    const Pair => DataPointerLoadSource::Pair,
);

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum DataPointerLoadTarget {
    Absolute(Pointer<Data<u16>>),
    High(Pointer<Data<u8>>),
}

enum_from_helper!(
    const Pointer<Data<u16>> => DataPointerLoadTarget::Absolute,
    const Pointer<Data<u8>> => DataPointerLoadTarget::High,
);
