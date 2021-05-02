mod field;
mod protocol_enum;
mod protocol_struct;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{spanned::Spanned, DeriveInput};

pub struct Item {
    pub protocol_support: (TokenStream, TokenStream, TokenStream),
    pub packet_id: Option<i32>,
    pub min_size: Option<i32>,
    pub max_size: Option<i32>,
}

pub(crate) fn expand(
    DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    }: &mut syn::DeriveInput,
) -> crate::Result {
    let mut output = TokenStream::new();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Item {
        protocol_support: (calc_len, ser, de),
        packet_id,
        min_size,
        max_size,
    } = match &data {
        syn::Data::Struct(data_struct) => protocol_struct::expand_struct(data_struct, attrs),
        syn::Data::Enum(data_enum) => protocol_enum::expand_enum(ident, &data_enum, attrs),
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "ProtocolSupport expected struct or enum",
            ))
        }
    }?;

    if let Some(id) = packet_id {
        let min_size = min_size.map(|size| {
            quote! {
                fn min_size(_: &protocol_internal::ProtocolVersion) -> i32 {
                    #size
                }
            }
        });

        let max_size = max_size.map(|size| {
            quote! {
                fn max_size(_: &protocol_internal::ProtocolVersion) -> i32 {
                    #size
                }
            }
        });

        output.append_all(quote! {
            impl #impl_generics ::protocol_internal::PacketEncoder for #ident #ty_generics #where_clause {
                fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                    ::protocol_internal::VarNum::<i32>::calculate_len(&#id) + ::protocol_internal::ProtocolSupportEncoder::calculate_len(self, version)
                }

                fn encode<W: std::io::Write>(&self, mut dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                    ::protocol_internal::VarNum::<i32>::encode(&#id, dst)?;
                    ::protocol_internal::ProtocolSupportEncoder::encode(self, dst, version)
                }
            }

            impl #impl_generics ::protocol_internal::PacketDecoder for #ident #ty_generics #where_clause {
                fn decode<R: std::io::Read + AsRef<[u8]>>(src: &mut std::io::Cursor<R>, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                    let id = ::protocol_internal::VarNum::<i32>::decode(src)?;
                    if id != #id {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", #id, id)));
                    }

                    ::protocol_internal::ProtocolSupportDecoder::decode(src, version)
                }
            }

            impl #impl_generics ::protocol_internal::PacketSizer for #ident #ty_generics #where_clause {
                #min_size
                #max_size
            }
        })
    }

    output.append_all(quote! {
        impl #impl_generics ::protocol_internal::ProtocolSupportEncoder for #ident #ty_generics #where_clause {
            fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                #calc_len
            }

            fn encode<W: std::io::Write>(&self, mut dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                #ser
            }
        }

        impl #impl_generics ::protocol_internal::ProtocolSupportDecoder for #ident #ty_generics #where_clause {
            fn decode<R: std::io::Read + AsRef<[u8]>>(src: &mut std::io::Cursor<R>, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                #de
            }
        }
    });

    Ok(output)
}
