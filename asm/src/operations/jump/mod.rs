use parse_display::Display;

pub use relative::*;

mod relative;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Jump {
    Relative(RelativeJump),
}
