use self::add::Add;
use self::bit::Bit;
use self::compare::Compare;
use self::decrement::Decrement;
use self::increment::Increment;
use self::jump::Jump;
use self::load::Load;
use self::rotate_left::RotateLeft;
use self::rotate_right::RotateRight;
use self::stack::Stack;
use self::subroutine::Subroutine;
use self::subtract::Subtract;
use crate::containers::Cycles;
use crate::operations::{Operation, OperationKind};
use crate::parse::{self, Parse, ParseResult};
use crate::{parse_helper, read::Read, register_helper, sets};
use parse_display::Display;

pub mod add;
pub mod bit;
pub mod compare;
pub mod decrement;
pub mod increment;
pub mod jump;
pub mod load;
pub mod rotate_left;
pub mod rotate_right;
pub mod stack;
pub mod subroutine;
pub mod subtract;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub width: u8,
    pub cycles: Cycles,
}

impl Instruction {
    pub fn parse<R: Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        Ok(Operation {
            kind: self.kind.parse(data, offset)?,
            width: self.width,
            cycles: self.cycles,
        })
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum InstructionKind {
    #[display("NOP")]
    Nop,

    #[display("STOP")]
    Stop,

    #[display("DAA")]
    DecimalAdjust,

    #[display("HALT")]
    Halt,

    #[display("DI")]
    DisableInterrupts,

    #[display("EI")]
    EnableInterrupts,

    Load(Load),
    Increment(Increment),
    Decrement(Decrement),
    RotateLeft(RotateLeft),
    RotateRight(RotateRight),
    Add(Add),
    Subtract(Subtract),
    Compare(Compare),
    Jump(Jump),
    Bit(Bit),
    Subroutine(Subroutine),
    Stack(Stack),
}

impl InstructionKind {
    pub fn set() -> sets::Instructions {
        let mut builder = sets::Builder::default();

        builder.base(0x00, InstructionKind::Nop, 1, 1);
        builder.base(0x10, InstructionKind::Stop, 1, 1);
        builder.base(0x27, InstructionKind::DecimalAdjust, 1, 1);
        builder.base(0x76, InstructionKind::Halt, 1, 1);
        builder.base(0xF3, InstructionKind::DisableInterrupts, 1, 1);
        builder.base(0xFB, InstructionKind::EnableInterrupts, 1, 1);

        register_helper!(
            &mut builder,
            Load,
            Increment,
            Decrement,
            RotateLeft,
            Add,
            Subtract,
            Compare,
            RotateRight,
            Jump,
            Bit,
            Subroutine,
            Stack,
        );

        builder.build()
    }
}

impl Parse for InstructionKind {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Nop => OperationKind::Nop,
            Self::Stop => OperationKind::Stop,
            Self::DecimalAdjust => OperationKind::DecimalAdjust,
            Self::Halt => OperationKind::Halt,
            Self::DisableInterrupts => OperationKind::DisableInterrupts,
            Self::EnableInterrupts => OperationKind::EnableInterrupts,
            Self::Load(inner),
            Self::Increment(inner),
            Self::Decrement(inner),
            Self::RotateLeft(inner),
            Self::RotateRight(inner),
            Self::Add(inner),
            Self::Subtract(inner),
            Self::Compare(inner),
            Self::Jump(inner),
            Self::Bit(inner),
            Self::Subroutine(inner),
            Self::Stack(inner)
        )
    }
}

trait SetRegister {
    fn register(builder: &mut sets::Builder);
}

#[macro_export]
macro_rules! register_helper {
    ( $builder:ident , $( $item:ty $(,)? ),* ) => {
        $( <$item>::register($builder) );*
    };

    ( &mut $builder:ident , $( $item:ty $(,)? ),* ) => {
        $( <$item>::register(&mut $builder) );*
    }
}
