use crate::module::Module;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn gen_mod(module: &Module) -> TokenStream {
    let name = syn::Ident::new(&module.name, Span::call_site());
    let body = module
        .body
        .as_ref()
        .map(|body| syn::parse_file(body).unwrap());
    let sub_modules = module
        .sub_modules
        .iter()
        .map(|m| gen_mod(m))
        .collect::<TokenStream>();

    quote! {
        pub mod #name {

            #body

            #sub_modules
        }
    }
}
