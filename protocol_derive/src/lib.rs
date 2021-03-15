#[macro_use]
extern crate quote;

mod protocol;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

type Result<T = proc_macro2::TokenStream> = std::result::Result<T, Error>;
type LSDResult = Result<(
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)>;

#[proc_macro_derive(ProtocolSupportDerive, attributes(protocol_field))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);

    protocol::expand(&mut derive_input)
        .unwrap_or_else(|err| syn::Error::to_compile_error(&err))
        .into()
}
