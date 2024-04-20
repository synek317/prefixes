//! A procedural macro that allows you to use the `P` attribute to create a `PathBuf` from a string literal.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn P(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    quote! { ::std::path::PathBuf::from(format!(#input)) }.into()
}
