use super::Load;
use crate::containers::{Data, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::load as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("LD {target}, {source}")]
pub struct DataPointerLoad {
    pub target: Pointer<Data<u16>>,
    pub source: DataPointerLoadSource,
}

impl DataPointerLoad {
    pub const fn new<S>(target: Pointer<Data<u16>>, source: S) -> Self
    where
        S: ~const Into<DataPointerLoadSource>,
    {
        Self {
            target,
            source: source.into(),
        }
    }
}

impl Parse for DataPointerLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        let target = self.target.parse(data, offset)?;
        Ok(op::DataPointerLoad::create(target, self.source))
    }
}

impl const SetRegister for DataPointerLoad {
    fn register(builder: &mut crate::sets::Builder) {
        builder.base(0x08, Self::new(Pointer(Data::new()), Pair::SP));
        builder.base(0xEA, Self::new(Pointer(Data::new()), Register::A));
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

impl const From<Register> for DataPointerLoadSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pair> for DataPointerLoadSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}
