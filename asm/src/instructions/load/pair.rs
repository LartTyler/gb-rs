use super::Load;
use crate::containers::{Data, Pair, Signed};
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::load as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
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

impl Parse for PairLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use PairLoadSource::*;

        let source: op::PairLoadSource = match self.source {
            Data(d) => d.parse(data, offset)?.into(),
            Pair(p) => p.into(),
            SignedData(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::PairLoad::create(self.target, source))
    }
}

impl const SetRegister for PairLoad {
    fn register(builder: &mut crate::sets::Builder) {
        use Pair::*;

        // LD r16, d16
        builder.base(0x01, Self::new(BC, Data::new()), 3, 3);
        builder.base(0x11, Self::new(DE, Data::new()), 3, 3);
        builder.base(0x21, Self::new(HL, Data::new()), 3, 3);
        builder.base(0x31, Self::new(SP, Data::new()), 3, 3);

        // LD HL, SP+s8
        builder.base(0xF8, Self::new(HL, Signed::new()), 2, 3);

        // LD SP, HL
        builder.base(0xF9, Self::new(SP, HL), 1, 2);
    }
}

impl const From<PairLoad> for InstructionKind {
    fn from(value: PairLoad) -> Self {
        InstructionKind::Load(Load::Pair(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum PairLoadSource {
    Data(Data<u16>),
    Pair(Pair),

    #[display("SP + {0}")]
    SignedData(Signed<Data<u8>>),
}

impl const From<Data<u16>> for PairLoadSource {
    fn from(value: Data<u16>) -> Self {
        Self::Data(value)
    }
}

impl const From<Pair> for PairLoadSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl const From<Signed<Data<u8>>> for PairLoadSource {
    fn from(value: Signed<Data<u8>>) -> Self {
        Self::SignedData(value)
    }
}
