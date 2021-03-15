use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, spanned::Spanned, Attribute, DeriveInput, Error, Field, FieldsNamed, MetaList,
};

use crate::packet::{FieldValidator, PacketField};

type LSDResult = Result<(TokenStream, TokenStream, TokenStream), Error>;

pub fn expand(
    DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    }: &mut syn::DeriveInput,
) -> crate::Result {
    let (calc_len, ser, de) = match &data {
        syn::Data::Struct(data_struct) => expand_struct(data_struct)?,
        syn::Data::Enum(data_enum) => expand_enum(ident, &data_enum, attrs)?,
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "PacketDerive expected struct",
            ))
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::protocol_internal::ProtocolSupport for #ident #ty_generics #where_clause {
            #calc_len

            #ser

            #de
        }
    })
}

fn expand_struct(data_struct: &syn::DataStruct) -> LSDResult {
    let fields_named = match &data_struct.fields {
        syn::Fields::Named(named) => named,
        _ => {
            return Err(syn::Error::new(
                data_struct.fields.span(),
                "PacketDerive expected named fields",
            ))
        }
    };

    parse_fields(&fields_named)
}

fn parse_fields(FieldsNamed { named, .. }: &FieldsNamed) -> LSDResult {
    let mut fields = vec![];
    for field in named {
        fields.push(parse_field(field)?);
    }

    let v_calc_len = fields
        .iter()
        .map(PacketField::calculate_len)
        .collect::<Vec<_>>();
    let calc_len = quote! {
        fn calculate_len(&self) -> usize {
            0 #(+ #v_calc_len)*
        }
    };

    let v_serialize = fields
        .iter()
        .map(PacketField::serialize)
        .collect::<Vec<_>>();
    let ser = quote! {
        fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
            #(#v_serialize)*
            Ok(())
        }
    };

    let v_deserialize = fields
        .iter()
        .map(PacketField::deserialize)
        .collect::<Vec<_>>();
    let de = quote! {
        fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
            Ok(Self {
                #(#v_deserialize)*
            })
        }
    };

    Ok((calc_len, ser, de))
}

fn parse_field(field: &Field) -> crate::Result<crate::packet::PacketField> {
    let ident = &field.ident.as_ref().ok_or(syn::Error::new(
        field.span(),
        "PacketDerive expected named field",
    ))?;

    let path = match &field.ty {
        syn::Type::Path(path) => path,
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "PacketDerive expected type path",
            ))
        }
    };

    let type_ident = path.path.to_token_stream();

    let mut packet_field = PacketField {
        ident,
        ty: type_ident,
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
    let nested = match attr.parse_meta()? {
        syn::Meta::List(list) => list,
        _ => {
            return Err(syn::Error::new(
                attr.span(),
                "PacketDerive expected attribute parameters",
            ))
        }
    }
    .nested;

    let meta_items = nested.iter().collect::<Vec<_>>();

    let mut is_varnum = false;
    let mut is_dynarray = false;

    for meta_item in meta_items {
        let meta = match meta_item {
            syn::NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new(
                    attr.span(),
                    "PacketDerive expected attribute meta",
                ))
            }
        };

        match meta {
            syn::Meta::Path(path) => {
                match path
                    .get_ident()
                    .ok_or(syn::Error::new(
                        attr.span(),
                        "PacketDerive expected attribute meta path ident",
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
                        "PacketDerive expected attribute meta path ident",
                    ))?
                    .to_string()
                    .as_str()
                {
                    "range" => return Ok((Some(extract_range(list)?), is_varnum, is_dynarray)),
                    path => {
                        return Err(syn::Error::new(
                            attr.span(),
                            format!("PacketDerive did not expect {}", path),
                        ))
                    }
                }
            }
            syn::Meta::NameValue(_) => {}
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
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(value) => {
                    let int: usize = match &value.lit {
                        syn::Lit::Int(int) => int,
                        _ => {
                            return Err(syn::Error::new(
                                list.span(),
                                format!("PacketDerive range expected int"),
                            ))
                        }
                    }
                    .base10_parse()?;

                    match value
                        .path
                        .get_ident()
                        .ok_or(syn::Error::new(
                            list.span(),
                            "PacketDerive range expected meta path ident",
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
                                format!("PacketDerive range expected min/max"),
                            ))
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        list.span(),
                        format!("PacketDerive range expected name value"),
                    ))
                }
            },
            _ => {
                return Err(syn::Error::new(
                    list.span(),
                    format!("PacketDerive range expected meta"),
                ))
            }
        }
    }

    Ok(FieldValidator::Range { min, max })
}

fn expand_enum(ident: &syn::Ident, data_enum: &syn::DataEnum, attrs: &Vec<Attribute>) -> LSDResult {
    let variants = data_enum
        .variants
        .iter()
        .map(|v| {
            let expr: &syn::Expr = &v.discriminant.as_ref().unwrap().1;
            let ident = &v.ident;

            quote! {
                #expr => Self::#ident,
            }
        })
        .collect::<Vec<_>>();

    let ty = extract_repr(ident, attrs);

    let calc_len;
    let ser;
    let de;

    if extract_enum_meta(attrs).is_some() {
        calc_len = quote! {
            fn calculate_len(&self) -> usize {
                ::protocol_internal::VarNum::<#ty>::calculate_len(*self as #ty)
            }
        };

        ser = quote! {
            fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                ::protocol_internal::VarNum::<#ty>::serialize(*self as #ty, dst)
            }
        };

        de = quote! {
            fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                Ok(match ::protocol_internal::VarNum::<#ty>::deserialize(src)? {
                    #(#variants)*
                    next_state => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("did not expect next state {}", next_state)))
                })
            }
        };
    } else {
        calc_len = quote! {
            fn calculate_len(&self) -> usize {
                ::protocol_internal::ProtocolSupport::calculate_len(&(*self as #ty))
            }
        };

        ser = quote! {
            fn serialize<W: std::io::Write>(&self, mut dst: &mut W) -> std::io::Result<()> {
                ::protocol_internal::ProtocolSupport::serialize(&(*self as #ty), dst)
            }
        };

        de = quote! {
            fn deserialize<R: std::io::Read>(mut src: &mut R) -> std::io::Result<Self> {
                Ok(match ::protocol_internal::ProtocolSupport::deserialize(src)? {
                    #(#variants)*
                    next_state => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("did not expect next state {}", next_state)))
                })
            }
        };
    }

    Ok((calc_len, ser, de))
}

fn extract_repr(ident: &syn::Ident, attrs: &Vec<Attribute>) -> syn::Ident {
    match attrs.iter().find(|attr| attr.path == parse_quote!(repr)) {
        Some(attr) => attr
            .parse_args::<syn::Ident>()
            .unwrap_or(syn::Ident::new("i32", ident.span())),
        None => syn::Ident::new("i32", ident.span()),
    }
}

fn extract_enum_meta(attrs: &Vec<Attribute>) -> Option<()> {
    attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(protocol_field))?
        .parse_args::<syn::Ident>()
        .map(|i| i.to_string())
        .ok()
        .and_then(|i| {
            if i.as_str() == "varnum" {
                Some(())
            } else {
                None
            }
        })
}
