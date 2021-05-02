use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Attribute, Expr, Ident, Variant};

use super::{field::FieldOptions, Item};

pub(crate) fn expand_enum(
    ident: &Ident,
    data_enum: &syn::DataEnum,
    attrs: &Vec<Attribute>,
) -> syn::Result<Item> {
    let ty = match extract_repr(attrs) {
        Some(ty) => ty,
        None => {
            return Err(syn::Error::new(
                ident.span(),
                format!("ProtocolSupport expected named fields or units"),
            ))
        }
    };
    let is_varnum = extract_varnum(attrs);

    let packet_id = super::field::extract_packet_id(attrs)?;
    let (min_size, max_size) = super::field::extract_packet_range(attrs);

    let mut calc_len: Vec<TokenStream> = Vec::new();
    let mut encode: Vec<TokenStream> = Vec::new();
    let mut decode: Vec<TokenStream> = Vec::new();

    for variant in &data_enum.variants {
        let expr = match variant.discriminant.as_ref().map(|(_, expr)| expr.clone()) {
            Some(expr) => expr,
            None => extract_variant_discriminant(variant)?,
        };

        let fields: Vec<FieldOptions> = match &variant.fields {
            syn::Fields::Named(named) => named
                .named
                .iter()
                .map(super::field::parse_field)
                .map(Result::unwrap)
                .collect(),
            syn::Fields::Unit => Vec::new(),
            _ => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("ProtocolSupport expected named fields or units"),
                ));
            }
        };

        calc_len.push(expand_variant_calculate_len(
            is_varnum,
            &ty,
            &expr,
            &variant.ident,
            &fields,
        ));
        encode.push(expand_variant_encode(
            is_varnum,
            &ty,
            &expr,
            &variant.ident,
            &fields,
        ));
        decode.push(expand_variant_decode(&expr, &variant.ident, &fields));
    }

    let ty_path = if is_varnum {
        quote! { ::protocol_internal::VarNum::<#ty>::decode(src) }
    } else {
        quote! { <#ty as ::protocol_internal::ProtocolSupportDecoder>::decode(src, version) }
    };

    Ok(Item {
        protocol_support: (
            quote! { match self { #(#calc_len)* } },
            quote! { match self { #(#encode)* } Ok(()) },
            quote! {
                Ok(match #ty_path? {
                    #(#decode)*
                    discriminant => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("did not expect {}", discriminant)
                        ));
                    }
                })
            },
        ),
        packet_id,
        min_size,
        max_size,
    })
}

fn extract_variant_discriminant(variant: &Variant) -> syn::Result<Expr> {
    let attr = variant
        .attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(protocol_field))
        .ok_or(syn::Error::new(
            variant.ident.span(),
            format!("ProtocolSupport expected enum discriminant"),
        ))?;

    let meta = match attr.parse_meta()? {
        syn::Meta::List(list) => list.nested.into_iter().next().ok_or(syn::Error::new(
            variant.ident.span(),
            format!("ProtocolSupport expected enum discriminant"),
        ))?,
        _ => {
            return Err(syn::Error::new(
                variant.ident.span(),
                format!("ProtocolSupport expected enum discriminant"),
            ))?
        }
    };
    let meta = match meta {
        syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) => meta,
        _ => {
            return Err(syn::Error::new(
                variant.ident.span(),
                format!("ProtocolSupport expected enum discriminant"),
            ))?
        }
    };

    let path = meta.path.get_ident().ok_or(syn::Error::new(
        variant.ident.span(),
        format!("ProtocolSupport expected enum discriminant"),
    ))?;
    if path != "enum_discriminant" {
        return Err(syn::Error::new(
            variant.ident.span(),
            format!("ProtocolSupport expected enum discriminant"),
        ))?;
    }

    Ok(Expr::Lit(syn::ExprLit {
        lit: meta.lit,
        attrs: Vec::new(),
    }))
}

fn expand_variant_calculate_len(
    is_varnum: bool,
    ty: &Ident,
    i: &Expr,
    ident: &Ident,
    fields: &Vec<FieldOptions>,
) -> TokenStream {
    let id_cl = match is_varnum {
        true => quote! { ::protocol_internal::VarNum::<#ty>::calculate_len(&(#i)) },
        false => {
            quote! { <#ty as ::protocol_internal::ProtocolSupportEncoder>::calculate_len(&(#i), version) }
        }
    };

    let calculate_len = fields.iter().map(FieldOptions::calculate_len);
    let fields = fields.iter().map(|f| f.ident);

    quote! {
        Self::#ident { #(#fields),* } => {
            #id_cl #(+ #calculate_len)*
        },
    }
}

fn expand_variant_encode(
    is_varnum: bool,
    ty: &Ident,
    i: &Expr,
    ident: &Ident,
    fields: &Vec<FieldOptions>,
) -> TokenStream {
    let id_encode = match is_varnum {
        true => quote! { ::protocol_internal::VarNum::<#ty>::encode(&(#i), dst)?; },
        false => {
            quote! { <#ty as ::protocol_internal::ProtocolSupportEncoder>::encode(&(#i), dst, version)?; }
        }
    };

    let encode = fields.iter().map(FieldOptions::encode);
    let fields = fields.iter().map(|f| f.ident);

    quote! {
        Self::#ident { #(#fields),* } => {
            #id_encode
            #(#encode)*
        },
    }
}

fn expand_variant_decode(i: &Expr, ident: &Ident, fields: &Vec<FieldOptions>) -> TokenStream {
    let decode = fields.iter().map(FieldOptions::decode);

    quote! {
        #i => Self::#ident {
            #(#decode)*
        },
    }
}

fn extract_repr(attrs: &Vec<Attribute>) -> Option<syn::Ident> {
    attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(repr))
        .map(|attr| attr.parse_args::<Ident>().ok())
        .flatten()
}

fn extract_varnum(attrs: &Vec<Attribute>) -> bool {
    match attrs
        .iter()
        .find(|attr| attr.path == parse_quote!(protocol_field))
        .map(|attr| {
            attr.parse_args::<Ident>()
                .map(|ident| ident.to_string())
                .ok()
        })
        .flatten()
    {
        Some(s) => &s == "varnum",
        None => false,
    }
}
