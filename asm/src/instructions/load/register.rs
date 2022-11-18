use super::{Action, Load};
use crate::containers::{Data, Pair, Pointer, Register};
use crate::instructions::{Instruction, SetRegister};
use crate::operations::load as op;
use crate::parse::{Parse, ParseResult};
use crate::{read::Read, sets::Builder};
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("LD {target}, {source}")]
pub struct RegisterLoad {
    pub target: Register,
    pub source: RegisterLoadSource,
}

impl RegisterLoad {
    pub const fn new<S>(target: Register, source: S) -> Self
    where
        S: ~const Into<RegisterLoadSource>,
    {
        Self {
            target,
            source: source.into(),
        }
    }
}

impl const From<RegisterLoad> for Instruction {
    fn from(value: RegisterLoad) -> Self {
        Self::Load(Load::Register(value))
    }
}

impl Parse for RegisterLoad {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use RegisterLoadSource::*;

        let source: op::RegisterLoadSource = match self.source {
            Data(d) => d.parse(data, offset)?.into(),
            PairPointer(s) => s.into(),
            Register(r) => r.into(),
            DataPointer(p) => p.parse(data, offset)?.into(),
            RegisterPointer(p) => p.into(),
        };

        Ok(op::RegisterLoad::create(self.target, source))
    }
}

impl const SetRegister for RegisterLoad {
    fn register(builder: &mut Builder) {
        use Register::*;

        // LD r8, d8
        builder.base(0x06, Self::new(B, Data::new()));
        builder.base(0x0E, Self::new(C, Data::new()));
        builder.base(0x16, Self::new(D, Data::new()));
        builder.base(0x1E, Self::new(E, Data::new()));
        builder.base(0x26, Self::new(H, Data::new()));
        builder.base(0x2E, Self::new(L, Data::new()));
        builder.base(0x3E, Self::new(A, Data::new()));

        use Action::*;
        use Pair::*;
        use PairPointerRegisterLoadSource as PairSource;

        // LD r8, (r16)
        builder.base(0x0A, Self::new(A, PairSource::new(Pointer(BC), None)));
        builder.base(0x1A, Self::new(A, PairSource::new(Pointer(DE), None)));
        builder.base(0x2A, Self::new(A, PairSource::new(Pointer(HL), Increment)));
        builder.base(0x3A, Self::new(A, PairSource::new(Pointer(HL), Decrement)));
        builder.base(0x46, Self::new(B, PairSource::new(Pointer(HL), None)));
        builder.base(0x4E, Self::new(C, PairSource::new(Pointer(HL), None)));
        builder.base(0x56, Self::new(D, PairSource::new(Pointer(HL), None)));
        builder.base(0x5E, Self::new(E, PairSource::new(Pointer(HL), None)));
        builder.base(0x66, Self::new(H, PairSource::new(Pointer(HL), None)));
        builder.base(0x6E, Self::new(L, PairSource::new(Pointer(HL), None)));
        builder.base(0x7E, Self::new(A, PairSource::new(Pointer(HL), None)));

        // LD r8, r8
        builder.base(0x40, Self::new(B, B));
        builder.base(0x41, Self::new(B, C));
        builder.base(0x42, Self::new(B, D));
        builder.base(0x43, Self::new(B, E));
        builder.base(0x44, Self::new(B, H));
        builder.base(0x45, Self::new(B, L));
        builder.base(0x47, Self::new(B, A));

        builder.base(0x48, Self::new(C, B));
        builder.base(0x49, Self::new(C, C));
        builder.base(0x4A, Self::new(C, D));
        builder.base(0x4B, Self::new(C, E));
        builder.base(0x4C, Self::new(C, H));
        builder.base(0x4D, Self::new(C, L));
        builder.base(0x4F, Self::new(C, A));

        builder.base(0x50, Self::new(D, B));
        builder.base(0x51, Self::new(D, C));
        builder.base(0x52, Self::new(D, D));
        builder.base(0x53, Self::new(D, E));
        builder.base(0x54, Self::new(D, H));
        builder.base(0x55, Self::new(D, L));
        builder.base(0x57, Self::new(D, A));

        builder.base(0x58, Self::new(E, B));
        builder.base(0x59, Self::new(E, C));
        builder.base(0x5A, Self::new(E, D));
        builder.base(0x5B, Self::new(E, E));
        builder.base(0x5C, Self::new(E, H));
        builder.base(0x5D, Self::new(E, L));
        builder.base(0x5F, Self::new(E, A));

        builder.base(0x60, Self::new(H, B));
        builder.base(0x61, Self::new(H, C));
        builder.base(0x62, Self::new(H, D));
        builder.base(0x63, Self::new(H, E));
        builder.base(0x64, Self::new(H, H));
        builder.base(0x65, Self::new(H, L));
        builder.base(0x67, Self::new(H, A));

        builder.base(0x68, Self::new(L, B));
        builder.base(0x69, Self::new(L, C));
        builder.base(0x6A, Self::new(L, D));
        builder.base(0x6B, Self::new(L, E));
        builder.base(0x6C, Self::new(L, H));
        builder.base(0x6D, Self::new(L, L));
        builder.base(0x6F, Self::new(L, A));

        builder.base(0x78, Self::new(A, B));
        builder.base(0x79, Self::new(A, C));
        builder.base(0x7A, Self::new(A, D));
        builder.base(0x7B, Self::new(A, E));
        builder.base(0x7C, Self::new(A, H));
        builder.base(0x7D, Self::new(A, L));
        builder.base(0x7F, Self::new(A, A));

        // Others
        builder.base(0xFA, Self::new(A, Pointer(Data::new())));
        builder.base(0xF2, Self::new(A, Pointer(C)));
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum RegisterLoadSource {
    Data(Data<u8>),
    PairPointer(PairPointerRegisterLoadSource),
    Register(Register),
    DataPointer(Pointer<Data<u16>>),
    RegisterPointer(Pointer<Register>),
}

impl const From<Data<u8>> for RegisterLoadSource {
    fn from(value: Data<u8>) -> Self {
        Self::Data(value)
    }
}

impl const From<PairPointerRegisterLoadSource> for RegisterLoadSource {
    fn from(value: PairPointerRegisterLoadSource) -> Self {
        Self::PairPointer(value)
    }
}

impl const From<Register> for RegisterLoadSource {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl const From<Pointer<Data<u16>>> for RegisterLoadSource {
    fn from(value: Pointer<Data<u16>>) -> Self {
        Self::DataPointer(value)
    }
}

impl const From<Pointer<Register>> for RegisterLoadSource {
    fn from(value: Pointer<Register>) -> Self {
        Self::RegisterPointer(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PairPointerRegisterLoadSource {
    pub source: Pointer<Pair>,
    pub action: Action,
}

impl PairPointerRegisterLoadSource {
    pub const fn new(source: Pointer<Pair>, action: Action) -> Self {
        Self { source, action }
    }
}

impl Display for PairPointerRegisterLoadSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.source.0, self.action)
    }
}
