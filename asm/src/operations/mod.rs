use self::add::Add;
use self::bit::Bit;
use self::jump::Jump;
use self::load::Load;
use self::subroutine::Subroutine;
use self::subtract::Subtract;
use crate::containers::Cycles;
use crate::instructions::decrement::Decrement;
use crate::instructions::increment::Increment;
use crate::instructions::rotate_left::RotateLeft;
use crate::instructions::rotate_right::RotateRight;
use crate::instructions::stack::Stack;
use parse_display::Display;

pub mod add;
pub mod bit;
pub mod jump;
pub mod load;
pub mod subroutine;
pub mod subtract;

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub width: u8,
    pub cycles: Cycles,
}

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum OperationKind {
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
