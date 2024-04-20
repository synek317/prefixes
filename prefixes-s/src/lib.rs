//! A procedural macro that allows you to use the `s` attribute to create a `Duration` from a number of seconds.
//! Part of the [prefixes](https://crates.io/crates/prefixes) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, LitFloat, LitInt};

#[proc_macro_attribute]
pub fn s(_: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(i) = parse::<LitInt>(input.clone()) {
        return quote! { ::core::time::Duration::from_secs(#i) }.into();
    }

    if let Ok(i) = parse::<LitFloat>(input.clone()) {
        match i.suffix() {
            "f32" => return quote! { ::core::time::Duration::from_secs_f32(#i) }.into(),
            "f64" | "" => return quote! { ::core::time::Duration::from_secs_f64(#i) }.into(),
            s => {
                return compile_error(
                    input,
                    &format!("unexpected float suffix: {s}. Expected 'f32' or 'f64'"),
                )
            }
        }
    }

    compile_error(input, "expected integer or float literal")
}

fn compile_error(input: TokenStream, msg: &str) -> TokenStream {
    syn::Error::new_spanned(
        proc_macro2::TokenStream::from(input),
        format!("Could not apply #[s] prefix: {msg}"),
    )
    .to_compile_error()
    .into()
}
