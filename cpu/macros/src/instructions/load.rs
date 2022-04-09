use crate::registers::{ByteRegister, PairRegister};
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub struct LoadR8IntoR8Input {
    pub src: ByteRegister,
    pub dest: ByteRegister,
}

impl Parse for LoadR8IntoR8Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let src: ByteRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: ByteRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}

pub struct LoadR8IntoR8AllInput {
    pub dest: ByteRegister,
}

impl Parse for LoadR8IntoR8AllInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dest: ByteRegister = input.parse()?;
        Ok(Self { dest })
    }
}

pub struct LoadImmediateIntoR8Input {
    pub dest: ByteRegister,
}

impl Parse for LoadImmediateIntoR8Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dest: ByteRegister = input.parse()?;
        Ok(Self { dest })
    }
}

pub struct LoadImmediateIntoR16Input {
    pub dest: PairRegister,
}

impl Parse for LoadImmediateIntoR16Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dest: PairRegister = input.parse()?;
        Ok(Self { dest })
    }
}

pub struct LoadR16PointerIntoR8Input {
    pub src: PairRegister,
    pub dest: ByteRegister,
}

impl Parse for LoadR16PointerIntoR8Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let src: PairRegister = input.parse()?;
        input.parse::<Token![,]>()?;
        let dest: ByteRegister = input.parse()?;

        Ok(Self { src, dest })
    }
}
