use quote::quote;
use syn::{parse_quote, spanned::Spanned, Attribute, Error, FieldsNamed};

use super::{field::FieldOptions, Item};

pub(crate) fn expand_struct(
    data_struct: &syn::DataStruct,
    attrs: &Vec<Attribute>,
) -> crate::Result<Item> {
    let packet_id = match attrs.iter().find(|attr| attr.path == parse_quote!(packet)) {
        Some(attr) => match attr.parse_meta()? {
            syn::Meta::List(meta) => match meta.nested.into_iter().collect::<Vec<_>>().first() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(id))) => match id.base10_parse::<i32>() {
                    Ok(id) => Some(id),
                    _ => return Err(Error::new_spanned(attr, "packet expected id")),
                },
                _ => return Err(Error::new_spanned(attr, "packet expected id")),
            },
            _ => return Err(Error::new_spanned(attr, "packet expected id")),
        },
        _ => None,
    };

    match &data_struct.fields {
        syn::Fields::Named(named) => parse_fields(&named, packet_id),
        syn::Fields::Unit => Ok(Item {
            protocol_support: (quote! { 0 }, quote! { Ok(()) }, quote! { Ok(Self) }),
            packet_id,
        }),
        _ => {
            return Err(syn::Error::new(
                data_struct.fields.span(),
                "ProtocolSupport expected named fields",
            ))
        }
    }
}

fn parse_fields(FieldsNamed { named, .. }: &FieldsNamed, packet_id: Option<i32>) -> crate::Result<Item> {
    let mut fields = vec![];
    for field in named {
        let mut value = super::field::parse_field(field)?;
        value.is_struct = true;

        fields.push(value);
    }

    let v_calc_len = fields.iter().map(FieldOptions::calculate_len);
    let calc_len = quote! { 0 #(+ #v_calc_len)* };

    let v_serialize = fields.iter().map(FieldOptions::serialize);
    let ser = quote! {
        #(#v_serialize)*
        Ok(())
    };

    let v_deserialize = fields.iter().map(FieldOptions::deserialize);
    let de = quote! {
        Ok(Self {
            #(#v_deserialize)*
        })
    };

    Ok(Item {
        protocol_support: (calc_len, ser, de),
        packet_id,
    })
}
