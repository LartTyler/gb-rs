use super::Add;
use crate::containers::{Data, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::add as op;
use crate::parse::{Parse, ParseResult};
use crate::{read::Read, sets::Builder};
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct RegisterAdd {
    pub source: RegisterAddSource,
    pub with_carry: bool,
}

impl RegisterAdd {
    pub const fn new<S>(source: S, with_carry: bool) -> Self
    where
        S: ~const Into<RegisterAddSource>,
    {
        Self {
            source: source.into(),
            with_carry,
        }
    }
}

impl Parse for RegisterAdd {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use RegisterAddSource::*;

        let source: op::RegisterAddSource = match self.source {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::RegisterAdd::create(source, self.with_carry))
    }
}

impl const SetRegister for RegisterAdd {
    fn register(builder: &mut Builder) {
        use Register::*;

        // ADD r8, r8
        builder.base(0x80, Self::new(B, false));
        builder.base(0x81, Self::new(C, false));
        builder.base(0x82, Self::new(D, false));
        builder.base(0x83, Self::new(E, false));
        builder.base(0x84, Self::new(H, false));
        builder.base(0x85, Self::new(L, false));
        builder.base(0x87, Self::new(A, false));

        // ADC r8, r8
        builder.base(0x88, Self::new(B, true));
        builder.base(0x89, Self::new(C, true));
        builder.base(0x8A, Self::new(D, true));
        builder.base(0x8B, Self::new(E, true));
        builder.base(0x8C, Self::new(H, true));
        builder.base(0x8D, Self::new(L, true));
        builder.base(0x8F, Self::new(A, true));

        // Others
        builder.base(0x86, Self::new(Pointer(Pair::HL), false));
        builder.base(0x8E, Self::new(Pointer(Pair::HL), true));
        builder.base(0xC6, Self::new(Data::new(), false));
        builder.base(0xCE, Self::new(Data::new(), true));
    }
}

impl const From<RegisterAdd> for Instruction {
    fn from(value: RegisterAdd) -> Self {
        Self::Add(Add::Register(value))
    }
}

impl Display for RegisterAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.with_carry { 'C' } else { 'D' };
        write!(f, "AD{c} A, {}", self.source)
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum RegisterAddSource {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

impl const From<Register> for RegisterAddSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Pair>> for RegisterAddSource {
    fn from(value: Pointer<Pair>) -> Self {
        Self::PairPointer(value)
    }
}

impl const From<Data<u8>> for RegisterAddSource {
    fn from(value: Data<u8>) -> Self {
        Self::Data(value)
    }
}
