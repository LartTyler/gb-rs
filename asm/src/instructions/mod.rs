use self::add::Add;
use self::bit::Bit;
use self::decrement::Decrement;
use self::increment::Increment;
use self::jump::Jump;
use self::load::Load;
use self::rotate_left::RotateLeft;
use self::rotate_right::RotateRight;
use self::stack::Stack;
use self::subroutine::Subroutine;
use self::subtract::Subtract;
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::{parse_helper, read::Read, register_helper, sets};
use parse_display::Display;

pub mod add;
pub mod bit;
pub mod decrement;
pub mod increment;
pub mod jump;
pub mod load;
pub mod rotate_left;
pub mod rotate_right;
pub mod stack;
pub mod subroutine;
pub mod subtract;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Instruction {
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
    Jump(Jump),
    Bit(Bit),
    Subroutine(Subroutine),
    Stack(Stack),
}

impl Instruction {
    pub const fn set() -> sets::Instructions {
        let mut builder = sets::Builder::default();

        builder.base(0x00, Instruction::Nop);
        builder.base(0x10, Instruction::Stop);
        builder.base(0x27, Instruction::DecimalAdjust);
        builder.base(0x76, Instruction::Halt);
        builder.base(0xF3, Instruction::DisableInterrupts);
        builder.base(0xFB, Instruction::EnableInterrupts);

        register_helper!(
            &mut builder,
            Load,
            Increment,
            Decrement,
            RotateLeft,
            Add,
            Subtract,
            RotateRight,
            Jump,
            Bit,
            Subroutine,
            Stack,
        );

        builder.build()
    }
}

impl Parse for Instruction {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Nop => Operation::Nop,
            Self::Stop => Operation::Stop,
            Self::DecimalAdjust => Operation::DecimalAdjust,
            Self::Halt => Operation::Halt,
            Self::DisableInterrupts => Operation::DisableInterrupts,
            Self::EnableInterrupts => Operation::EnableInterrupts,
            Self::Load(inner),
            Self::Increment(inner),
            Self::Decrement(inner),
            Self::RotateLeft(inner),
            Self::RotateRight(inner),
            Self::Add(inner),
            Self::Subtract(inner),
            Self::Jump(inner),
            Self::Bit(inner),
            Self::Subroutine(inner),
            Self::Stack(inner)
        )
    }
}

#[const_trait]
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
