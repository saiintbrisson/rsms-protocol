#[macro_use]
extern crate quote;

mod packet;
mod protocol;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Error, ItemMod};

type Result<T = proc_macro2::TokenStream> = std::result::Result<T, Error>;
type LSDResult = Result<(
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)>;

#[proc_macro_derive(ProtocolSupportDerive, attributes(protocol_field, packet))]
pub fn derive_protocol_support(input: TokenStream) -> TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);

    protocol::expand(&mut derive_input)
        .unwrap_or_else(|err| syn::Error::to_compile_error(&err))
        .into()
}

#[proc_macro_attribute]
pub fn packets(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemMod);

    packet::expand(attr, item)
        .unwrap_or_else(|err| syn::Error::to_compile_error(&err))
        .into()
}
