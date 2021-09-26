use proc_macro2::TokenStream;
use quote::quote;

pub fn parse(input: TokenStream) -> TokenStream {
    let _: syn::ItemEnum = syn::parse2(input).unwrap();
    quote! {}
}
