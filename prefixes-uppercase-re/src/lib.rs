//! A procedural macro that allows you to use the `RE` attribute to create a `Regex` from a string literal.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn RE(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    quote! { ::regex::Regex::new(#input).unwrap() }.into()
}
