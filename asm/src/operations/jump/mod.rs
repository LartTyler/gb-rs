use parse_display::Display;

pub use absolute::*;
pub use relative::*;

mod absolute;
mod relative;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Jump {
    Absolute(AbsoluteJump),
    Relative(RelativeJump),
}
