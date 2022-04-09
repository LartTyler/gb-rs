use syn::parse::{Parse, ParseStream};
use syn::{parse_str, Error, Expr, Ident};

pub struct ByteRegister {
    pub ident: Ident,
    pub ident_char: char,
}

impl ByteRegister {
    pub fn as_enum_expr(&self) -> Expr {
        parse_str::<Expr>(&format!(
            "crate::registers::ByteRegister::{}",
            self.ident_char.to_uppercase()
        ))
        .unwrap()
    }
}

impl Parse for ByteRegister {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const VALID: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'h', 'l'];

        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();

        if ident_str.len() != 1 {
            return Err(Error::new(
                ident.span(),
                format!("expected a valid r8 identifier: {:?}", VALID),
            ));
        }

        let ident_char = ident.to_string().chars().next().unwrap();

        if !VALID.contains(&ident_char) {
            return Err(Error::new(
                ident.span(),
                format!("expected a valid r8 identifier: {:?}", VALID),
            ));
        }

        Ok(ByteRegister { ident, ident_char })
    }
}

pub struct PairRegister {
    pub ident: Ident,
    pub ident_str: String,
}

impl PairRegister {
    pub fn as_enum_expr(&self) -> Expr {
        parse_str::<Expr>(&format!(
            "crate::registers::PairRegister::{}",
            self.ident_str.to_uppercase()
        ))
        .unwrap()
    }
}

impl Parse for PairRegister {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        const VALID: [&str; 4] = ["bc", "de", "hl", "sp"];

        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();

        if ident_str.len() != 2 || !VALID.into_iter().any(|item| item == &ident_str) {
            return Err(Error::new(
                ident.span(),
                format!("expected a valid r16 identifier: {:?}", VALID),
            ));
        }

        Ok(PairRegister { ident, ident_str })
    }
}
