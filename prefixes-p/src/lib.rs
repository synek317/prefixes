//! A procedural macro that allows you to use the `p` attribute to create a `Path` from a string literal.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
pub fn p(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    quote! { ::std::path::Path::new(#input) }.into()
}
