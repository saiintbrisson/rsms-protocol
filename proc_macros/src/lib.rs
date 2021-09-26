mod codec;
mod packet;

use proc_macro::TokenStream;

#[proc_macro_derive(Codec, attributes(codec, constraints))]
pub fn codec_proc_macro(input: TokenStream) -> TokenStream {
    codec::parse(input.into()).into()
}

#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet_proc_macro(input: TokenStream) -> TokenStream {
    packet::parse(input.into()).into()
}
