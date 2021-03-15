mod field;
mod protocol_enum;
mod protocol_struct;

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
    let (calc_len, ser, de) = match &data {
        syn::Data::Struct(data_struct) => protocol_struct::expand_struct(data_struct)?,
        syn::Data::Enum(data_enum) => protocol_enum::expand_enum(ident, &data_enum, attrs)?,
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "ProtocolSupportDerive expected struct or enum",
            ))
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::protocol_internal::ProtocolSupport for #ident #ty_generics #where_clause {
            fn calculate_len(&self) -> usize {
                #calc_len
            }

            fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                #ser
            }

            fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                #de
            }
        }
    })
}