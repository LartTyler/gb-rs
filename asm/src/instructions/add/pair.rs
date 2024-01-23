use super::Add;
use crate::containers::{Data, Pair, Signed};
use crate::instructions::{InstructionKind, SetRegister};
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
    pub fn new<S>(target: Pair, source: S) -> Self
    where
        S: Into<PairAddSource>,
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

impl SetRegister for PairAdd {
    fn register(builder: &mut Builder) {
        use Pair::*;

        // ADD r16, r16
        builder.base(0x09, Self::new(HL, BC), 1, 2);
        builder.base(0x19, Self::new(HL, DE), 1, 2);
        builder.base(0x29, Self::new(HL, HL), 1, 2);
        builder.base(0x39, Self::new(HL, SP), 1, 2);

        // Others
        builder.base(0xE8, Self::new(SP, Signed::new()), 2, 4);
    }
}

impl From<PairAdd> for InstructionKind {
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

impl From<Pair> for PairAddSource {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl From<Signed<Data<u8>>> for PairAddSource {
    fn from(value: Signed<Data<u8>>) -> Self {
        Self::SignedData(value)
    }
}
