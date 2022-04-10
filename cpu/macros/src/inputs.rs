use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

use crate::registers::{ByteRegister, PairRegister};

pub struct R8TargetInput {
    pub target: ByteRegister,
}

impl Parse for R8TargetInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: ByteRegister = input.parse()?;
        Ok(Self { target })
    }
}

pub struct R16TargetInput {
    pub target: PairRegister,
}

impl Parse for R16TargetInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: PairRegister = input.parse()?;
        Ok(Self { target })
    }
}

pub struct R8IntoR16Input {
    pub src: ByteRegister,
    pub dest: PairRegister,
}

impl Parse for R8IntoR16Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let src: ByteRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: PairRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}

pub struct R16IntoR8Input {
    pub src: PairRegister,
    pub dest: ByteRegister,
}

impl Parse for R16IntoR8Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let src: PairRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: ByteRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}

pub struct R8IntoR8Input {
    pub src: ByteRegister,
    pub dest: ByteRegister,
}

impl Parse for R8IntoR8Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let src: ByteRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: ByteRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}

pub struct R16IntoR16Input {
    pub src: PairRegister,
    pub dest: PairRegister,
}

impl Parse for R16IntoR16Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let src: PairRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: PairRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}
