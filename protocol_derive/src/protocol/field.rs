use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Error, Field, Ident, MetaList, parse_quote, spanned::Spanned};

#[derive(Debug)]
pub(crate) struct FieldOptions<'a> {
    pub ident: &'a Ident,
    pub ty: TokenStream,
    pub protocol_type: FieldType,
    pub validator: Option<FieldValidator>,
    pub is_struct: bool,
}

impl<'a> FieldOptions<'a> {
    pub fn calculate_len(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.protocol_type.get_path_ser(&self.ty);
        match self.is_struct {
            true => quote! { #path::calculate_len(&self.#ident) },
            false => quote! { #path::calculate_len(#ident) },
        }
    }

    pub fn encode(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.protocol_type.get_path_ser(&self.ty);
        match self.is_struct {
            true => quote! { #path::encode(&self.#ident, &mut dst)?; },
            false => quote! { #path::encode(#ident, &mut dst)?; },
        }
    }

    pub fn decode(&self) -> TokenStream {
        let ident = &self.ident;

        let method = if let Some(validator) = &self.validator {
            let path = match validator {
                FieldValidator::Range { .. } => {
                    self.protocol_type.get_range_validator_path(&self.ty)
                }
                _ => self.protocol_type.get_path_de(&self.ty),
            };

            validator.decode(&path)
        } else {
            let path = self.protocol_type.get_path_de(&self.ty);
            quote! { #path::decode(&mut src) }
        };

        quote! {
            #ident: #method?,
        }
    }
}

#[derive(Debug)]
pub(crate) enum FieldValidator {
    Fixed(usize),
    Range { min: usize, max: usize },
    Regex(String),
}

impl FieldValidator {
    pub fn decode(&self, path: &TokenStream) -> proc_macro2::TokenStream {
        match self {
            FieldValidator::Fixed(len) => quote! {
                #path::decode(&mut src, #len)
            },
            FieldValidator::Range { min, max } => quote! {
                #path::decode(&mut src, #min, #max)
            },
            FieldValidator::Regex(regex) => quote! {
                {
                    ::lazy_static::lazy_static! { static ref REGEX: ::regex::Regex = regex::Regex::new(#regex).unwrap(); };
                    #path::decode(&mut src, &REGEX)
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum FieldType {
    VarNum,
    Position,
    DynArray,
    Fixed,
    Regex,
    Default,
}

impl FieldType {
    pub fn get_path_ser(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldType::VarNum => quote! { ::protocol_internal::VarNum::<#ty> },
            FieldType::Position => quote! { ::protocol_internal::ProtocolPositionSupport },
            FieldType::DynArray => quote! { ::protocol_internal::DynArray },
            _ => {
                quote! { <#ty as ::protocol_internal::ProtocolSupportEncoder> }
            }
        }
    }

    pub fn get_path_de(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldType::VarNum => quote! { ::protocol_internal::VarNum::<#ty> },
            FieldType::Position => quote! { ::protocol_internal::ProtocolPositionSupport },
            FieldType::DynArray => quote! { ::protocol_internal::DynArray },
            FieldType::Fixed => quote! { ::protocol_internal::FixedVec },
            FieldType::Regex => quote! { ::protocol_internal::Regex },
            FieldType::Default => {
                quote! { <#ty as ::protocol_internal::ProtocolSupportDecoder> }
            }
        }
    }

    pub fn get_range_validator_path(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldType::VarNum => {
                quote! { <::protocol_internal::VarNum<#ty> as ::protocol_internal::RangeValidatedSupport<#ty>> }
            }
            FieldType::DynArray => {
                quote! { <::protocol_internal::DynArray as ::protocol_internal::RangeValidatedSupport<#ty>> }
            }
            FieldType::Default => quote! { <#ty as ::protocol_internal::RangeValidatedSupport> },
            _ => panic!(""),
        }
    }
}

pub(crate) fn parse_field(field: &Field) -> crate::Result<FieldOptions> {
    let ident = &field.ident.as_ref().ok_or(Error::new(
        field.span(),
        "ProtocolSupport expected named field",
    ))?;

    let path = match &field.ty {
        syn::Type::Path(path) => path,
        syn::Type::Group(group) => match &*group.elem {
            syn::Type::Path(path) => path,
            _ => {
                return Err(syn::Error::new(
                    field.span(),
                    "ProtocolSupport expected type path",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "ProtocolSupport expected type path",
            ))
        }
    };

    let attr = match field
        .attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(protocol_field))
    {
        Some(attr) => attr,
        None => {
            return Ok(FieldOptions {
                ident,
                ty: path.path.to_token_stream(),
                protocol_type: FieldType::Default,
                validator: None,
                is_struct: false,
            })
        }
    };

    let (validator, protocol_type) = parse_field_meta(attr)?;

    Ok(FieldOptions {
        ident,
        ty: path.path.to_token_stream(),
        protocol_type,
        validator,
        is_struct: false,
    })
}

fn parse_field_meta(attr: &Attribute) -> Result<(Option<FieldValidator>, FieldType), Error> {
    let meta_items = match attr.parse_meta()? {
        syn::Meta::List(list) => list.nested.into_iter().collect::<Vec<_>>(),
        _ => {
            return Err(syn::Error::new(
                attr.span(),
                "ProtocolSupport expected attribute parameters",
            ))
        }
    };

    let mut protocol_type = FieldType::Default;

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
                    "varnum" => protocol_type = FieldType::VarNum,
                    "position" => protocol_type = FieldType::Position,
                    "dynarray" => protocol_type = FieldType::DynArray,
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
                    "range" => return Ok((Some(extract_range(&list)?), protocol_type)),
                    "regex" => return Ok((Some(extract_regex(&list)?), FieldType::Regex)),
                    path => {
                        return Err(syn::Error::new(
                            attr.span(),
                            format!("ProtocolSupport did not expect {}", path),
                        ))
                    }
                }
            }
            syn::Meta::NameValue(value) => {
                match value
                    .path
                    .get_ident()
                    .ok_or(syn::Error::new(
                        attr.span(),
                        "ProtocolSupport expected attribute meta path ident",
                    ))?
                    .to_string()
                    .as_str()
                {
                    "fixed" => return Ok((Some(extract_fixed(&value)?), FieldType::Fixed)),
                    path => {
                        return Err(syn::Error::new(
                            attr.span(),
                            format!("ProtocolSupport did not expect {}", path),
                        ))
                    }
                }
            }
        }
    }

    Ok((None, protocol_type))
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

fn extract_regex(list: &MetaList) -> crate::Result<FieldValidator> {
    let regex = match list.nested.first().unwrap() {
        syn::NestedMeta::Lit(syn::Lit::Str(str)) => str.value(),
        _ => {
            return Err(syn::Error::new(
                list.span(),
                format!("ProtocolSupport regex expected string"),
            ))
        }
    };

    Ok(FieldValidator::Regex(regex))
}

fn extract_fixed(value: &syn::MetaNameValue) -> crate::Result<FieldValidator> {
    let int: usize = match &value.lit {
        syn::Lit::Int(int) => int,
        _ => {
            return Err(syn::Error::new(
                value.span(),
                format!("ProtocolSupport fixed expected int"),
            ))
        }
    }
    .base10_parse()?;

    Ok(FieldValidator::Fixed(int))
}

