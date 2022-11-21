use parse_display::Display;

pub use register::*;

mod register;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Subtract {
    Register(RegisterSubtract),
}
