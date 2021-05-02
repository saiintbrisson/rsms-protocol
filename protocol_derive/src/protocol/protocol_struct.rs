use quote::quote;
use syn::{spanned::Spanned, Attribute, FieldsNamed};

use super::{field::FieldOptions, Item};

pub(crate) fn expand_struct(
    data_struct: &syn::DataStruct,
    attrs: &Vec<Attribute>,
) -> crate::Result<Item> {
    let packet_id = super::field::extract_packet_id(attrs)?;
    let (min_size, max_size) = super::field::extract_packet_range(attrs);

    match &data_struct.fields {
        syn::Fields::Named(named) => parse_fields(&named, packet_id, min_size, max_size),
        syn::Fields::Unit => Ok(Item {
            protocol_support: (quote! { 0 }, quote! { Ok(()) }, quote! { Ok(Self) }),
            packet_id,
            min_size,
            max_size,
        }),
        _ => {
            return Err(syn::Error::new(
                data_struct.fields.span(),
                "ProtocolSupport expected named fields",
            ))
        }
    }
}

fn parse_fields(
    FieldsNamed { named, .. }: &FieldsNamed,
    packet_id: Option<i32>,
    min_size: Option<i32>,
    max_size: Option<i32>,
) -> crate::Result<Item> {
    let mut fields = vec![];
    for field in named {
        let mut value = super::field::parse_field(field)?;
        value.is_struct = true;

        fields.push(value);
    }

    let v_calc_len = fields.iter().map(FieldOptions::calculate_len);
    let calc_len = quote! { 0 #(+ #v_calc_len)* };

    let v_encode = fields.iter().map(FieldOptions::encode);
    let ser = quote! {
        #(#v_encode)*
        Ok(())
    };

    let v_decode = fields.iter().map(FieldOptions::decode);
    let de = quote! {
        Ok(Self {
            #(#v_decode)*
        })
    };

    Ok(Item {
        protocol_support: (calc_len, ser, de),
        packet_id,
        min_size,
        max_size,
    })
}
