use crate::registers::{ByteRegister, PairRegister};
use syn::parse::{Parse, ParseStream};

pub struct IncrementR16Input {
    pub target: PairRegister,
}

impl Parse for IncrementR16Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target: PairRegister = input.parse()?;
        Ok(Self { target })
    }
}

pub struct IncrementR8Input {
    pub target: ByteRegister,
}

impl Parse for IncrementR8Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target: ByteRegister = input.parse()?;
        Ok(Self { target })
    }
}
