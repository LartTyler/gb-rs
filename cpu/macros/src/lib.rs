use self::inputs::*;
use self::util::create_instruction_fn;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod inputs;
mod registers;
mod util;

#[proc_macro]
pub fn load_r8_into_r8(input: TokenStream) -> TokenStream {
    let R8IntoR8Input { src, dest } = parse_macro_input!(input as R8IntoR8Input);

    let fn_name = format_ident!("load_{}_into_{}", src.ident, dest.ident);
    let src = src.as_enum_expr();
    let dest = dest.ident;

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            load_byte_into_byte_register(r, #src, r.#dest)
        },
    ))
}

#[proc_macro]
pub fn load_r8_into_r8_all(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);
    let target = target.ident;

    TokenStream::from(quote! {
        gb_rs_cpu_macros::load_r8_into_r8!(a, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(b, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(c, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(d, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(e, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(h, #target);
        gb_rs_cpu_macros::load_r8_into_r8!(l, #target);
    })
}

#[proc_macro]
pub fn load_immediate_into_r8(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("load_immediate_into_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            load_immediate_into_byte_register(r, m, #target)
        },
    ))
}

#[proc_macro]
pub fn load_immediate_into_r16(input: TokenStream) -> TokenStream {
    let R16TargetInput { target } = parse_macro_input!(input as R16TargetInput);

    let fn_name = format_ident!("load_immediate_into_{}", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            load_immediate_into_pair(r, m, #target)
        },
    ))
}

#[proc_macro]
pub fn load_r16_pointer_into_r8(input: TokenStream) -> TokenStream {
    let R16IntoR8Input { src, dest } = parse_macro_input!(input as R16IntoR8Input);

    let fn_name = format_ident!("load_{}_pointer_into_{}", src.ident, dest.ident);
    let src = src.as_enum_expr();
    let dest = dest.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            load_pair_pointer_into_byte_register(r, m, #src, #dest)
        },
    ))
}

#[proc_macro]
pub fn load_r8_into_r16_pointer(input: TokenStream) -> TokenStream {
    let R8IntoR16Input { src, dest } = parse_macro_input!(input as R8IntoR16Input);

    let fn_name = format_ident!("load_{}_into_{}_pointer", src.ident, dest.ident);
    let src = src.ident;
    let dest = dest.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            load_byte_into_pair_address(r, m, #dest, r.#src)
        },
    ))
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

#[proc_macro]
pub fn rotate_left_carry_extended(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("rotate_left_carry_{}_extended", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            rotate_left_carry_extended(r, #target)
        },
    ))
}

#[proc_macro]
pub fn rotate_right_extended(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("rotate_right_{}_extended", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            rotate_right_extended(r, #target)
        },
    ))
}

#[proc_macro]
pub fn rotate_right_carry_extended(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("rotate_right_carry_{}_extended", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            rotate_right_carry_extended(r, #target)
        },
    ))
}

#[proc_macro]
pub fn add_r8_to_a(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("add_{}_to_a", target.ident);
    let target = target.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            add_byte_register_to_a(r, #target)
        },
    ))
}

#[proc_macro]
pub fn add_r16_to_r16(input: TokenStream) -> TokenStream {
    let R16IntoR16Input { src, dest } = parse_macro_input!(input as R16IntoR16Input);

    let fn_name = format_ident!("add_{}_to_{}", src.ident, dest.ident);
    let src = src.as_enum_expr();
    let dest = dest.as_enum_expr();

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            add_pair_to_pair(r, #src, #dest)
        },
    ))
}

#[proc_macro]
pub fn add_r8_and_carry_to_a(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("add_{}_and_carry_to_a", target.ident);
    let target = target.ident;

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            add_byte_and_carry_to_a(r, r.#target)
        },
    ))
}

#[proc_macro]
pub fn sub_r8_from_a(input: TokenStream) -> TokenStream {
    let R8TargetInput { target } = parse_macro_input!(input as R8TargetInput);

    let fn_name = format_ident!("sub_{}_from_a", target.ident);
    let target = target.ident;

    TokenStream::from(create_instruction_fn(
        fn_name,
        quote! {
            sub_from_a(r, r.#target)
        },
    ))
}
