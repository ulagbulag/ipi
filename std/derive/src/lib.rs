#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod class;

#[proc_macro_derive(Class, attributes(class))]
pub fn derive_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    self::class::expand_derive_serialize(input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
