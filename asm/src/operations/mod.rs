use self::add::Add;
use self::bitwise::Bitwise;
use self::jump::Jump;
use self::load::Load;
use self::subtract::Subtract;
use crate::instructions::decrement::Decrement;
use crate::instructions::increment::Increment;
use crate::instructions::rotate_left::RotateLeft;
use crate::instructions::rotate_right::RotateRight;
use parse_display::Display;

pub mod add;
pub mod bitwise;
pub mod jump;
pub mod load;
pub mod subtract;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Operation {
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
    Bitwise(Bitwise),
}
