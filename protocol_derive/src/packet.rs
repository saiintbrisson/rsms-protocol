use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Error, ItemMod, NestedMeta};

pub(crate) fn expand(attr: Vec<NestedMeta>, item: ItemMod) -> crate::Result {
    let m = item.to_token_stream();
    let content = match item.content {
        Some(content) => content.1,
        None => return Ok(TokenStream::new()),
    };

    let mut variants = vec![];

    for item in content {
        let item = match item {
            syn::Item::Struct(item_struct) => item_struct,
            _ => continue,
        };

        let ident = item.ident;
        let attr = match item
            .attrs
            .into_iter()
            .find(|attr| attr.path == parse_quote!(packet))
        {
            Some(attr) => attr,
            None => continue,
        };

        let nested = match attr.parse_meta()? {
            syn::Meta::List(meta) => meta.nested.into_iter().collect::<Vec<_>>(),
            _ => return Err(Error::new_spanned(attr, "packet expected id")),
        };

        let id = match nested.first() {
            Some(NestedMeta::Lit(syn::Lit::Int(id))) => match id.base10_parse::<i32>() {
                Ok(id) => id,
                _ => return Err(Error::new_spanned(attr, "packet expected id")),
            },
            _ => return Err(Error::new_spanned(attr, "packet expected id")),
        };

        variants.push((ident, id));
    }

    let variants_ident = variants.iter().map(|(ident, _)| ident);
    let variants_calc_len = variants.iter().map(|(ident, _)| {
        quote! {
            Self::#ident(packet) => ::protocol_internal::ProtocolSupportEncoder::calculate_len(packet)
        }
    });
    let variants_ser = variants.iter().map(|(ident, _)| {
        quote! {
            Self::#ident(packet) => ::protocol_internal::ProtocolSupportEncoder::encode(packet, dst, version)
        }
    });

    let variants_packet_calc_len = variants.iter().map(|(ident, _)| {
        quote! {
            Self::#ident(packet) => ::protocol_internal::PacketEncoder::calculate_len(packet, version)
        }
    });
    let variants_packet_ser = variants.iter().map(|(ident, _)| {
        quote! {
            Self::#ident(packet) => ::protocol_internal::PacketEncoder::encode(packet, dst, version)
        }
    });
    let variants_packet_de = variants.iter().map(|(ident, id)| {
        quote! {
            #id => Ok(Self::#ident(::protocol_internal::ProtocolSupportDecoder::decode(src, version)?))
        }
    });

    let mod_ident = item.ident;
    let ident = match attr.first() {
        Some(NestedMeta::Lit(syn::Lit::Str(str))) => str.parse::<syn::Ident>().unwrap(),
        _ => panic!("expected string literal"),
    };

    Ok(quote! {
        #m

        #[derive(Debug)]
        pub enum #ident {
            #(#variants_ident(#mod_ident::#variants_ident)),*
        }

        impl ::protocol_internal::ProtocolSupportEncoder for #ident {
            fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                match self {
                    #(#variants_calc_len),*
                }
            }

            fn encode<W: std::io::Write>(&self, mut dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                Ok(match self {
                    #(#variants_ser?),*
                })
            }
        }

        impl ::protocol_internal::ProtocolSupportDecoder for #ident {
            fn decode<R: std::io::Read + AsRef<[u8]>>(_: &mut std::io::Cursor<R>, _: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                unimplemented!();
            }
        }

        impl ::protocol_internal::PacketEncoder for #ident {
            fn calculate_len(&self) -> usize {
                match self {
                    #(#variants_packet_calc_len),*
                }
            }

            fn encode<W: std::io::Write>(&self, mut dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                match self {
                    #(#variants_packet_ser),*
                }
            }
        }

        impl ::protocol_internal::PacketDecoder for #ident {
            fn decode<R: std::io::Read + AsRef<[u8]>>(src: &mut std::io::Cursor<R>, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                match ::protocol_internal::VarNum::<i32>::decode(src)? {
                    #(#variants_packet_de),*,
                    id => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("invalid packet id {}", id)))
                }
            }
        }
    })
}
