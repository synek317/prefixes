//! A procedural macro that allows you to use the `os` attribute to create an `OsStr` from a string literal.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.
//! 
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
pub fn os(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    quote! { ::std::ffi::OsStr::new(#input) }.into()
}
