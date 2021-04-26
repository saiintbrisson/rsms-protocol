mod field;
mod protocol_enum;
mod protocol_struct;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{spanned::Spanned, DeriveInput};

pub struct Item {
    pub protocol_support: (TokenStream, TokenStream, TokenStream),
    pub packet_id: Option<i32>,
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

    let Item { protocol_support: (calc_len, ser, de), packet_id } = match &data {
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
        output.append_all(quote! {
            impl #impl_generics ::protocol_internal::PacketSerializer for #ident #ty_generics #where_clause {
                fn calculate_len(&self) -> usize {
                    ::protocol_internal::VarNum::<i32>::calculate_len(&#id) + ::protocol_internal::ProtocolSupportSerializer::calculate_len(self)
                }

                fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                    ::protocol_internal::VarNum::<i32>::serialize(&#id, dst)?;
                    ::protocol_internal::ProtocolSupportSerializer::serialize(self, dst)
                }
            }

            impl #impl_generics ::protocol_internal::PacketDeserializer for #ident #ty_generics #where_clause {
                fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                    let id = ::protocol_internal::VarNum::<i32>::deserialize(src)?;
                    if id != #id {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", #id, id)));
                    }
    
                    ::protocol_internal::ProtocolSupportDeserializer::deserialize(src)
                }
            }
        })
    }

    output.append_all(quote! {
        impl #impl_generics ::protocol_internal::ProtocolSupportSerializer for #ident #ty_generics #where_clause {
            fn calculate_len(&self) -> usize {
                #calc_len
            }

            fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                #ser
            }
        }

        impl #impl_generics ::protocol_internal::ProtocolSupportDeserializer for #ident #ty_generics #where_clause {
            fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                #de
            }
        }
    });

    Ok(output)
}
