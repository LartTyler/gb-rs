use super::Subtract;
use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
use crate::operations::subtract as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct RegisterSubtract {
    pub source: RegisterSubtractSource,
    pub with_carry: bool,
}

impl RegisterSubtract {
    pub fn new<S>(source: S, with_carry: bool) -> Self
    where
        S: Into<RegisterSubtractSource>,
    {
        Self {
            source: source.into(),
            with_carry,
        }
    }
}

impl From<RegisterSubtract> for InstructionKind {
    fn from(value: RegisterSubtract) -> Self {
        Self::Subtract(Subtract::Register(value))
    }
}

impl Display for RegisterSubtract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.with_carry { "SBC" } else { "SUB" };
        write!(f, "{mnemonic} A, {}", self.source)
    }
}

impl Parse for RegisterSubtract {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use RegisterSubtractSource::*;

        let source: op::RegisterSubtractSource = match self.source {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::RegisterSubtract::create(source, self.with_carry))
    }
}

impl SetRegister for RegisterSubtract {
    fn register(builder: &mut Builder) {
        use Register::*;

        // SUB A, r8
        builder.base(0x90, Self::new(B, false), 1, 1);
        builder.base(0x91, Self::new(C, false), 1, 1);
        builder.base(0x92, Self::new(D, false), 1, 1);
        builder.base(0x93, Self::new(E, false), 1, 1);
        builder.base(0x94, Self::new(H, false), 1, 1);
        builder.base(0x95, Self::new(L, false), 1, 1);
        builder.base(0x97, Self::new(A, false), 1, 1);

        // SBC A, r8
        builder.base(0x98, Self::new(B, true), 1, 1);
        builder.base(0x99, Self::new(C, true), 1, 1);
        builder.base(0x9A, Self::new(D, true), 1, 1);
        builder.base(0x9B, Self::new(E, true), 1, 1);
        builder.base(0x9C, Self::new(H, true), 1, 1);
        builder.base(0x9D, Self::new(L, true), 1, 1);
        builder.base(0x9F, Self::new(A, true), 1, 1);

        // Others
        builder.base(0x96, Self::new(Pointer(Pair::HL), false), 1, 2);
        builder.base(0x9E, Self::new(Pointer(Pair::HL), true), 1, 2);
        builder.base(0xD6, Self::new(Data::new(), false), 2, 2);
        builder.base(0xDE, Self::new(Data::new(), true), 2, 2);
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum RegisterSubtractSource {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

enum_from_helper!(
    Register => RegisterSubtractSource::Register,
    Pointer<Pair> => RegisterSubtractSource::PairPointer,
    Data<u8> => RegisterSubtractSource::Data,
);
