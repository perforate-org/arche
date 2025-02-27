extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Request)]
pub fn derive_request(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput syntax tree.
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Request for #name
        where #name: candid::CandidType + Clone + Send + Sync + 'static
        {}
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl Response for #name
        where #name: candid::CandidType + serde::de::DeserializeOwned + Send + Sync + 'static
        {}
    };
    TokenStream::from(expanded)
}
