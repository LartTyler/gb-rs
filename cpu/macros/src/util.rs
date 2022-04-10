use quote::quote;
use syn::Ident;

pub fn create_instruction_fn(
    name: Ident,
    content: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        pub fn #name(r: &mut crate::registers::Registers, m: &mut gb_rs_memory::Memory) -> crate::instructions::Effect {
            #content
        }
    }
}
