use self::inputs::*;
use self::instructions::load::{
    LoadImmediateIntoR16Input, LoadImmediateIntoR8Input, LoadR16PointerIntoR8Input,
    LoadR8IntoR8AllInput, LoadR8IntoR8Input,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod inputs;
mod instructions;
mod registers;
mod util;

#[proc_macro]
pub fn load_r8_into_r8(input: TokenStream) -> TokenStream {
    let LoadR8IntoR8Input { src, dest } = parse_macro_input!(input as LoadR8IntoR8Input);

    let fn_name = format_ident!("load_{}_into_{}", src.ident, dest.ident);
    let src = src.as_enum_expr();
    let dest = dest.ident;

    TokenStream::from(quote! {
        pub fn #fn_name(r: &mut crate::registers::Registers, _: &mut gb_rs_memory::Memory) -> crate::instructions::Effect {
            load_byte_into_byte_register(r, #src, r.#dest)
        }
    })
}

#[proc_macro]
pub fn load_r8_into_r8_all(input: TokenStream) -> TokenStream {
    let LoadR8IntoR8AllInput { dest } = parse_macro_input!(input as LoadR8IntoR8AllInput);
    let dest = dest.ident;

    TokenStream::from(quote! {
        gb_rs_cpu_macros::load_r8_into_r8!(a, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(b, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(c, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(d, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(e, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(h, #dest);
        gb_rs_cpu_macros::load_r8_into_r8!(l, #dest);
    })
}

#[proc_macro]
pub fn load_immediate_into_r8(input: TokenStream) -> TokenStream {
    let LoadImmediateIntoR8Input { dest } = parse_macro_input!(input as LoadImmediateIntoR8Input);

    let fn_name = format_ident!("load_immediate_into_{}", dest.ident);
    let dest = dest.as_enum_expr();

    TokenStream::from(quote! {
        pub fn #fn_name(r: &mut crate::registers::Registers, m: &mut gb_rs_memory::Memory) -> crate::instructions::Effect {
            load_immediate_into_byte_register(r, m, #dest)
        }
    })
}

#[proc_macro]
pub fn load_immediate_into_r16(input: TokenStream) -> TokenStream {
    let LoadImmediateIntoR16Input { dest } = parse_macro_input!(input as LoadImmediateIntoR16Input);

    let fn_name = format_ident!("load_immediate_into_{}", dest.ident);
    let dest = dest.as_enum_expr();

    TokenStream::from(quote! {
        pub fn #fn_name(r: &mut crate::registers::Registers, m: &mut gb_rs_memory::Memory) -> crate::instructions::Effect {
            load_immediate_into_pair(r, m, #dest)
        }
    })
}

#[proc_macro]
pub fn load_r16_pointer_into_r8(input: TokenStream) -> TokenStream {
    let LoadR16PointerIntoR8Input { src, dest } =
        parse_macro_input!(input as LoadR16PointerIntoR8Input);

    let fn_name = format_ident!("load_{}_pointer_into_{}", src.ident, dest.ident);
    let src = src.as_enum_expr();
    let dest = dest.as_enum_expr();

    TokenStream::from(quote! {
        pub fn #fn_name(r: &mut crate::registers::Registers, m: &mut gb_rs_memory::Memory) -> crate::instructions::Effect {
            load_pair_pointer_into_byte_register(r, m, #src, #dest)
        }
    })
}

#[proc_macro]
pub fn increment_r8(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("increment_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            increment_byte_register(r, #target)
        },
    ))
}

#[proc_macro]
pub fn increment_r16(input: TokenStream) -> TokenStream {
    let R16TargetInput { target } = parse_macro_input!(input as R16TargetInput);

    let fn_name = format_ident!("increment_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            increment_pair(r, #target)
        },
    ))
}

#[proc_macro]
pub fn decrement_r8(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("decrement_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            decrement_byte_register(r, #target)
        },
    ))
}

#[proc_macro]
pub fn decrement_r16(input: TokenStream) -> TokenStream {
    let R16TargetInput { target } = parse_macro_input!(input as R16TargetInput);

    let fn_name = format_ident!("decrement_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            decrement_pair(r, #target)
        },
    ))
}

#[proc_macro]
pub fn rotate_left(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("rotate_left_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            rotate_left(r, #target)
        },
    ))
}

#[proc_macro]
pub fn rotate_left_extended(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("rotate_left_{}_extended", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(util::create_instruction_fn(
        fn_name,
        quote! {
            rotate_left_extended(r, #target)
        },
    ))
}
