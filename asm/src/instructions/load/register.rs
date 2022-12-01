use super::{Action, Load};
use crate::containers::{ByteData, Data, Pair, Pointer, Register, WordData};
use crate::enum_from_helper;
use crate::instructions::{InstructionKind, SetRegister};
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

impl const From<RegisterLoad> for InstructionKind {
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
            HighDataPointer(p) => p.parse(data, offset)?.into(),
            RegisterPointer(p) => p.into(),
        };

        Ok(op::RegisterLoad::create(self.target, source))
    }
}

impl const SetRegister for RegisterLoad {
    fn register(builder: &mut Builder) {
        use Register::*;

        // LD r8, d8
        builder.base(0x06, Self::new(B, Data::new()), 2, 2);
        builder.base(0x0E, Self::new(C, Data::new()), 2, 2);
        builder.base(0x16, Self::new(D, Data::new()), 2, 2);
        builder.base(0x1E, Self::new(E, Data::new()), 2, 2);
        builder.base(0x26, Self::new(H, Data::new()), 2, 2);
        builder.base(0x2E, Self::new(L, Data::new()), 2, 2);
        builder.base(0x3E, Self::new(A, Data::new()), 2, 2);

        use Action::*;
        use Pair::*;
        use PairPointerRegisterLoadSource as PairSource;

        // LD r8, (r16)
        builder.base(0x0A, Self::new(A, PairSource::new(Pointer(BC), None)), 1, 2);
        builder.base(0x1A, Self::new(A, PairSource::new(Pointer(DE), None)), 1, 2);

        builder.base(
            0x2A,
            Self::new(A, PairSource::new(Pointer(HL), Increment)),
            1,
            2,
        );

        builder.base(
            0x3A,
            Self::new(A, PairSource::new(Pointer(HL), Decrement)),
            1,
            2,
        );

        builder.base(0x46, Self::new(B, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x4E, Self::new(C, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x56, Self::new(D, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x5E, Self::new(E, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x66, Self::new(H, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x6E, Self::new(L, PairSource::new(Pointer(HL), None)), 1, 2);
        builder.base(0x7E, Self::new(A, PairSource::new(Pointer(HL), None)), 1, 2);

        // LD r8, r8
        builder.base(0x40, Self::new(B, B), 1, 1);
        builder.base(0x41, Self::new(B, C), 1, 1);
        builder.base(0x42, Self::new(B, D), 1, 1);
        builder.base(0x43, Self::new(B, E), 1, 1);
        builder.base(0x44, Self::new(B, H), 1, 1);
        builder.base(0x45, Self::new(B, L), 1, 1);
        builder.base(0x47, Self::new(B, A), 1, 1);

        builder.base(0x48, Self::new(C, B), 1, 1);
        builder.base(0x49, Self::new(C, C), 1, 1);
        builder.base(0x4A, Self::new(C, D), 1, 1);
        builder.base(0x4B, Self::new(C, E), 1, 1);
        builder.base(0x4C, Self::new(C, H), 1, 1);
        builder.base(0x4D, Self::new(C, L), 1, 1);
        builder.base(0x4F, Self::new(C, A), 1, 1);

        builder.base(0x50, Self::new(D, B), 1, 1);
        builder.base(0x51, Self::new(D, C), 1, 1);
        builder.base(0x52, Self::new(D, D), 1, 1);
        builder.base(0x53, Self::new(D, E), 1, 1);
        builder.base(0x54, Self::new(D, H), 1, 1);
        builder.base(0x55, Self::new(D, L), 1, 1);
        builder.base(0x57, Self::new(D, A), 1, 1);

        builder.base(0x58, Self::new(E, B), 1, 1);
        builder.base(0x59, Self::new(E, C), 1, 1);
        builder.base(0x5A, Self::new(E, D), 1, 1);
        builder.base(0x5B, Self::new(E, E), 1, 1);
        builder.base(0x5C, Self::new(E, H), 1, 1);
        builder.base(0x5D, Self::new(E, L), 1, 1);
        builder.base(0x5F, Self::new(E, A), 1, 1);

        builder.base(0x60, Self::new(H, B), 1, 1);
        builder.base(0x61, Self::new(H, C), 1, 1);
        builder.base(0x62, Self::new(H, D), 1, 1);
        builder.base(0x63, Self::new(H, E), 1, 1);
        builder.base(0x64, Self::new(H, H), 1, 1);
        builder.base(0x65, Self::new(H, L), 1, 1);
        builder.base(0x67, Self::new(H, A), 1, 1);

        builder.base(0x68, Self::new(L, B), 1, 1);
        builder.base(0x69, Self::new(L, C), 1, 1);
        builder.base(0x6A, Self::new(L, D), 1, 1);
        builder.base(0x6B, Self::new(L, E), 1, 1);
        builder.base(0x6C, Self::new(L, H), 1, 1);
        builder.base(0x6D, Self::new(L, L), 1, 1);
        builder.base(0x6F, Self::new(L, A), 1, 1);

        builder.base(0x78, Self::new(A, B), 1, 1);
        builder.base(0x79, Self::new(A, C), 1, 1);
        builder.base(0x7A, Self::new(A, D), 1, 1);
        builder.base(0x7B, Self::new(A, E), 1, 1);
        builder.base(0x7C, Self::new(A, H), 1, 1);
        builder.base(0x7D, Self::new(A, L), 1, 1);
        builder.base(0x7F, Self::new(A, A), 1, 1);

        // Others
        builder.base(0xFA, Self::new(A, Pointer(WordData::new())), 3, 4);
        builder.base(0xF2, Self::new(A, Pointer(C)), 2, 2);
        builder.base(0xF0, Self::new(A, Pointer(ByteData::new())), 2, 3);
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum RegisterLoadSource {
    Data(Data<u8>),
    PairPointer(PairPointerRegisterLoadSource),
    Register(Register),
    DataPointer(Pointer<Data<u16>>),
    HighDataPointer(Pointer<Data<u8>>),
    RegisterPointer(Pointer<Register>),
}

enum_from_helper!(
    const Data<u8> => RegisterLoadSource::Data,
    const PairPointerRegisterLoadSource => RegisterLoadSource::PairPointer,
    const Register => RegisterLoadSource::Register,
    const Pointer<Data<u16>> => RegisterLoadSource::DataPointer,
    const Pointer<Data<u8>> => RegisterLoadSource::HighDataPointer,
    const Pointer<Register> => RegisterLoadSource::RegisterPointer,
);

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
