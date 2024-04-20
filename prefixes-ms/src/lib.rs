//! A procedural macro that allows you to use the `ms` attribute to create a `Duration` from a number of milliseconds.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro_attribute]
pub fn ms(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);

    quote! { ::core::time::Duration::from_millis(#input) }.into()
}
