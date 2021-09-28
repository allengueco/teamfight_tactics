use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::{quote};

mod synergy;

#[proc_macro_derive(Synergy, attributes(breakpoints))]
pub fn derive_synergy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    synergy::derive(input).into()
}