mod field;
mod protocol_enum;
mod protocol_struct;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{spanned::Spanned, DeriveInput};

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

    let (calc_len, ser, de) = match &data {
        syn::Data::Struct(data_struct) => {
            let str = protocol_struct::expand_struct(data_struct, attrs)?;
            if let Some((calc_len, ser, de)) = str.packet {
                output.append_all(quote! {
                    impl #impl_generics ::protocol_internal::PacketSerializer for #ident #ty_generics #where_clause {
                        fn calculate_len(&self) -> usize {
                            #calc_len
                        }

                        fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                            #ser
                        }
                    }

                    impl #impl_generics ::protocol_internal::PacketDeserializer for #ident #ty_generics #where_clause {
                        fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                            #de
                        }
                    }
                })
            }

            str.protocol_support
        }
        syn::Data::Enum(data_enum) => protocol_enum::expand_enum(ident, &data_enum, attrs)?,
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "ProtocolSupport expected struct or enum",
            ))
        }
    };

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
