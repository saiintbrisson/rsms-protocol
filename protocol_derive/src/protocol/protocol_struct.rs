use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, spanned::Spanned, Attribute, Error, Field, FieldsNamed, MetaList};

use super::field::{FieldValidator, PacketField};

pub struct Struct {
    pub protocol_support: (TokenStream, TokenStream, TokenStream),
    pub packet: Option<(TokenStream, TokenStream, TokenStream)>,
}

pub(crate) fn expand_struct(
    data_struct: &syn::DataStruct,
    attrs: &Vec<Attribute>,
) -> crate::Result<Struct> {
    let id = match attrs.iter().find(|attr| attr.path == parse_quote!(packet)) {
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
        syn::Fields::Named(named) => parse_fields(&named, id),
        syn::Fields::Unit => Ok(Struct {
            protocol_support: (quote! { 0 }, quote! { Ok(()) }, quote! { Ok(Self) }),
            packet: id.map(|id| {
                (
                    quote! { ::protocol_internal::VarNum::<i32>::calculate_len(&#id) },
                    quote! { ::protocol_internal::VarNum::<i32>::serialize(&#id, dst) },
                    quote! { ::protocol_internal::VarNum::<i32>::deserialize(src)?; Ok(Self) },
                )
            }),
        }),
        _ => {
            return Err(syn::Error::new(
                data_struct.fields.span(),
                "ProtocolSupport expected named fields",
            ))
        }
    }
}

fn parse_fields(FieldsNamed { named, .. }: &FieldsNamed, id: Option<i32>) -> crate::Result<Struct> {
    let mut fields = vec![];
    for field in named {
        fields.push(parse_field(field)?);
    }

    let v_calc_len = fields.iter().map(PacketField::calculate_len);
    let calc_len = quote! { 0 #(+ #v_calc_len)* };

    let v_serialize = fields.iter().map(PacketField::serialize);
    let ser = quote! {
        #(#v_serialize)*
        Ok(())
    };

    let v_deserialize = fields.iter().map(PacketField::deserialize);
    let de = quote! {
        Ok(Self {
            #(#v_deserialize)*
        })
    };

    Ok(Struct {
        protocol_support: (calc_len, ser, de),
        packet: id.map(|id| (
            quote! {
                ::protocol_internal::VarNum::<i32>::calculate_len(&#id) + ::protocol_internal::ProtocolSupport::calculate_len(self)
            },
            quote! {
                ::protocol_internal::VarNum::<i32>::serialize(&#id, dst)?;
                ::protocol_internal::ProtocolSupport::serialize(self, dst)
            },
            quote! {
                let id = ::protocol_internal::VarNum::<i32>::deserialize(src)?;
                if id != #id {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", #id, id)));
                }

                ::protocol_internal::ProtocolSupport::deserialize(src)
            }
        )),
    })
}

fn parse_field(field: &Field) -> crate::Result<super::field::PacketField> {
    let ident = &field.ident.as_ref().ok_or(Error::new(
        field.span(),
        "ProtocolSupport expected named field",
    ))?;

    let path = match &field.ty {
        syn::Type::Path(path) => path,
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "ProtocolSupport expected type path",
            ))
        }
    };

    let mut packet_field = PacketField {
        ident,
        ty: path.path.to_token_stream(),
        is_varnum: false,
        is_dynarray: false,
        validator: None,
    };

    let attr = match field
        .attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(protocol_field))
    {
        Some(attr) => attr,
        None => return Ok(packet_field),
    };

    let (validator, is_varnum, is_dynarray) = parse_field_meta(attr)?;
    packet_field.validator = validator;
    packet_field.is_varnum = is_varnum;
    packet_field.is_dynarray = is_dynarray;

    Ok(packet_field)
}

fn parse_field_meta(attr: &Attribute) -> Result<(Option<FieldValidator>, bool, bool), Error> {
    let meta_items = match attr.parse_meta()? {
        syn::Meta::List(list) => list.nested.into_iter().collect::<Vec<_>>(),
        _ => {
            return Err(syn::Error::new(
                attr.span(),
                "ProtocolSupport expected attribute parameters",
            ))
        }
    };

    let mut is_varnum = false;
    let mut is_dynarray = false;

    for meta_item in meta_items {
        let meta = match meta_item {
            syn::NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new(
                    attr.span(),
                    "ProtocolSupport expected attribute meta",
                ))
            }
        };

        match meta {
            syn::Meta::Path(path) => {
                match path
                    .get_ident()
                    .ok_or(syn::Error::new(
                        attr.span(),
                        "ProtocolSupport expected attribute meta path ident",
                    ))?
                    .to_string()
                    .as_str()
                {
                    "varnum" => is_varnum = true,
                    "dynarray" => is_dynarray = true,
                    _ => {}
                }
            }
            syn::Meta::List(list) => {
                match list
                    .path
                    .get_ident()
                    .ok_or(syn::Error::new(
                        attr.span(),
                        "ProtocolSupport expected attribute meta path ident",
                    ))?
                    .to_string()
                    .as_str()
                {
                    "range" => return Ok((Some(extract_range(&list)?), is_varnum, is_dynarray)),
                    path => {
                        return Err(syn::Error::new(
                            attr.span(),
                            format!("ProtocolSupport did not expect {}", path),
                        ))
                    }
                }
            }
            _ => {}
        }
    }

    Ok((None, is_varnum, is_dynarray))
}

fn extract_range(list: &MetaList) -> crate::Result<FieldValidator> {
    let meta_items = list.nested.iter().collect::<Vec<_>>();

    let mut min = 0usize;
    let mut max = 0usize;

    for meta_item in meta_items {
        match meta_item {
            syn::NestedMeta::Meta(syn::Meta::NameValue(value)) => {
                let int: usize = match &value.lit {
                    syn::Lit::Int(int) => int,
                    _ => {
                        return Err(syn::Error::new(
                            list.span(),
                            format!("ProtocolSupport range expected int"),
                        ))
                    }
                }
                .base10_parse()?;

                match value
                    .path
                    .get_ident()
                    .ok_or(syn::Error::new(
                        list.span(),
                        "ProtocolSupport range expected meta path ident",
                    ))?
                    .to_string()
                    .as_str()
                {
                    "min" => min = int,
                    "max" => max = int,
                    "eq" => {
                        min = int;
                        max = int;
                    }
                    _ => {
                        return Err(syn::Error::new(
                            list.span(),
                            format!("ProtocolSupport range expected min/max"),
                        ))
                    }
                }
            }
            _ => {
                return Err(syn::Error::new(
                    list.span(),
                    format!("ProtocolSupport range expected meta"),
                ))
            }
        }
    }

    Ok(FieldValidator::Range { min, max })
}
