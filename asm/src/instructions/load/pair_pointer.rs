use super::{Action, Load};
use crate::containers::{Data, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::load as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct PairPointerLoad {
    pub target: Pointer<Pair>,
    pub source: PairPointerLoadSource,
    pub action: Action,
}

impl PairPointerLoad {
    pub const fn new<S>(target: Pointer<Pair>, source: S, action: Action) -> Self
    where
        S: ~const Into<PairPointerLoadSource>,
    {
        Self {
            target,
            source: source.into(),
            action,
        }
    }
}

impl const From<PairPointerLoad> for Instruction {
    fn from(value: PairPointerLoad) -> Self {
        Self::Load(Load::PairPointer(value))
    }
}

impl Parse for PairPointerLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use PairPointerLoadSource::*;

        let source: op::PairPointerLoadSource = match self.source {
            Register(r) => r.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::PairPointerLoad::create(
            self.target,
            source,
            self.action,
        ))
    }
}

impl const SetRegister for PairPointerLoad {
    fn register(builder: &mut crate::sets::Builder) {
        use Action::*;
        use Pair::*;
        use Register::*;

        // LD (r16), r8
        builder.base(0x02, Self::new(Pointer(BC), A, None));
        builder.base(0x12, Self::new(Pointer(DE), A, None));
        builder.base(0x70, Self::new(Pointer(HL), B, None));
        builder.base(0x71, Self::new(Pointer(HL), C, None));
        builder.base(0x72, Self::new(Pointer(HL), D, None));
        builder.base(0x73, Self::new(Pointer(HL), E, None));
        builder.base(0x74, Self::new(Pointer(HL), H, None));
        builder.base(0x75, Self::new(Pointer(HL), L, None));
        builder.base(0x77, Self::new(Pointer(HL), A, None));

        // LD (HL±), A
        builder.base(0x22, Self::new(Pointer(HL), A, Increment));
        builder.base(0x32, Self::new(Pointer(HL), A, Decrement));

        // Others
        builder.base(0x36, Self::new(Pointer(HL), Data::new(), None));
    }
}

impl Display for PairPointerLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LD ({}{}), {}", self.target.0, self.action, self.source)
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum PairPointerLoadSource {
    Register(Register),
    Data(Data<u8>),
}

impl const From<Register> for PairPointerLoadSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Data<u8>> for PairPointerLoadSource {
    fn from(value: Data<u8>) -> Self {
        Self::Data(value)
    }
}
