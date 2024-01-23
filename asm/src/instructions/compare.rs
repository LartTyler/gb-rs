use super::{InstructionKind, SetRegister};
use crate::containers::{Data, Pair, Pointer, Register};
use crate::enum_from_helper;
use crate::operations::compare as op;
use crate::parse::{Parse, ParseResult};
use crate::read::Read;
use crate::sets::Builder;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("CP {target}")]
pub struct Compare {
    pub target: CompareTarget,
}

impl Compare {
    pub fn new<T>(target: T) -> Self
    where
        T: Into<CompareTarget>,
    {
        Self {
            target: target.into(),
        }
    }
}

impl SetRegister for Compare {
    fn register(builder: &mut Builder) {
        // CP r8
        builder.base(0xB8, Self::new(Register::B), 1, 1);
        builder.base(0xB9, Self::new(Register::C), 1, 1);
        builder.base(0xBA, Self::new(Register::D), 1, 1);
        builder.base(0xBB, Self::new(Register::E), 1, 1);
        builder.base(0xBC, Self::new(Register::H), 1, 1);
        builder.base(0xBD, Self::new(Register::L), 1, 1);
        builder.base(0xBF, Self::new(Register::A), 1, 1);

        // CP (r16)
        builder.base(0xBE, Self::new(Pointer(Pair::HL)), 1, 2);

        // CP d8
        builder.base(0xFE, Self::new(Data::new()), 2, 2);
    }
}

impl Parse for Compare {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        use CompareTarget::*;

        let target: op::CompareTarget = match self.target {
            Register(r) => r.into(),
            PairPointer(p) => p.into(),
            Data(d) => d.parse(data, offset)?.into(),
        };

        Ok(op::Compare::create(target))
    }
}

enum_from_helper!(
    Compare => InstructionKind::Compare,
);

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum CompareTarget {
    Register(Register),
    PairPointer(Pointer<Pair>),
    Data(Data<u8>),
}

enum_from_helper!(
    Register => CompareTarget::Register,
    Pointer<Pair> => CompareTarget::PairPointer,
    Data<u8> => CompareTarget::Data,
);
