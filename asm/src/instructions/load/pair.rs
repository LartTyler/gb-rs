use super::Load;
use crate::containers::{Data, Pair};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::{load as op, Operation};
use crate::{parse, read::Read};
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("LD {target}, {source}")]
pub struct PairLoad {
    pub target: Pair,
    pub source: PairLoadSource,
}

impl PairLoad {
    pub const fn new<S>(target: Pair, source: S) -> Self
    where
        S: ~const Into<PairLoadSource>,
    {
        Self {
            target,
            source: source.into(),
        }
    }
}

impl parse::Parse for PairLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        use PairLoadSource::*;

        let source = match self.source {
            Data(d) => d.parse(data, offset + 1)?,
        };

        Ok(op::PairLoad::create(self.target, source))
    }
}

impl const SetRegister for PairLoad {
    fn register(builder: &mut crate::sets::Builder) {
        use Pair::*;

        builder.base(0x01, Self::new(BC, Data::new()));
        builder.base(0x11, Self::new(DE, Data::new()));
        builder.base(0x21, Self::new(HL, Data::new()));
        builder.base(0x31, Self::new(SP, Data::new()));
    }
}

impl const From<PairLoad> for Instruction {
    fn from(value: PairLoad) -> Self {
        Instruction::Load(Load::Pair(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum PairLoadSource {
    Data(Data<u16>),
}

impl const From<Data<u16>> for PairLoadSource {
    fn from(value: Data<u16>) -> Self {
        Self::Data(value)
    }
}
