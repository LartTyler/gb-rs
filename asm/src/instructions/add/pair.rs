use super::Add;
use crate::containers::{Data, Pair, Signed};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::add as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("ADD {target}, {source}")]
pub struct PairAdd {
    pub target: Pair,
    pub source: PairAddSource,
}

impl PairAdd {
    pub const fn new<S>(target: Pair, source: S) -> Self
    where
        S: ~const Into<PairAddSource>,
    {
        Self {
            target,
            source: source.into(),
        }
    }
}

impl Parse for PairAdd {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use PairAddSource::*;

        let source: op::PairAddSource = match self.source {
            Pair(p) => p.into(),
            SignedData(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::PairAdd::create(self.target, source))
    }
}

impl const SetRegister for PairAdd {
    fn register(builder: &mut Builder) {
        use Pair::*;

        // ADD r16, r16
        builder.base(0x09, Self::new(HL, BC));
        builder.base(0x19, Self::new(HL, DE));
        builder.base(0x29, Self::new(HL, HL));
        builder.base(0x39, Self::new(HL, SP));

        // Others
        builder.base(0xE8, Self::new(SP, Signed::new()));
    }
}

impl const From<PairAdd> for Instruction {
    fn from(value: PairAdd) -> Self {
        Self::Add(Add::Pair(value))
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum PairAddSource {
    Pair(Pair),
    SignedData(Signed<Data<u8>>),
}

impl const From<Pair> for PairAddSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl const From<Signed<Data<u8>>> for PairAddSource {
    fn from(value: Signed<Data<u8>>) -> Self {
        Self::SignedData(value)
    }
}
