use std::convert::TryFrom;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    spanned::Spanned, Attribute, Field, FieldsNamed, FieldsUnnamed, Lit, Meta, MetaList,
    NestedMeta, Result, Type,
};

pub fn parse(input: TokenStream) -> TokenStream {
    let s: syn::ItemStruct = syn::parse2(input).unwrap();
    let (impl_generics, ty_generics, where_clause) = s.generics.split_for_impl();

    let (decoder, encoder) = match s.fields {
        syn::Fields::Named(named) => parse_named(named).unwrap(),
        syn::Fields::Unnamed(unnamed) => parse_unnamed(unnamed).unwrap(),
        syn::Fields::Unit => (TokenStream::new(), TokenStream::new()),
    };

    let ident = s.ident;
    quote! {
        impl #impl_generics ::protocol_primitives::Decoder for #ident #ty_generics #where_clause {
            type Output = #ident #ty_generics;

            fn decode<R>(
                src: &mut R,
                c: &::protocol_primitives::Constraints,
                ctx: &::protocol_primitives::CodecContext
            ) -> ::std::io::Result<Self::Output>
            where
                R: ::std::io::Read,
            {
                Ok(#ident #decoder)
            }
        }

        impl #impl_generics ::protocol_primitives::Encoder<#ident#ty_generics> for #ident #ty_generics #where_clause {
            fn encode<W>(dst: &mut W, i: &#ident#ty_generics, ctx: &::protocol_primitives::CodecContext) -> ::std::io::Result<usize>
            where
                W: ::std::io::Write,
            {
                let mut written = 0;
                #encoder
                Ok(written)
            }
        }
    }
}

fn parse_named(input: FieldsNamed) -> Result<(TokenStream, TokenStream)> {
    let mut decoder = TokenStream::new();
    let mut encoder = TokenStream::new();

    for field in input.named {
        let ident = field.ident.clone();
        let field = CodecField::try_from(&field)?;
        let decoder_fn = field.decoder_fn();
        let encoder_fn = field.encoder_fn(&quote! { i.#ident });

        decoder.append_all(quote! {
            #ident: #decoder_fn,
        });

        encoder.append_all(quote! {
            written += #encoder_fn;
        });
    }

    Ok((quote! { { #decoder } }, encoder))
}

fn parse_unnamed(input: FieldsUnnamed) -> Result<(TokenStream, TokenStream)> {
    let mut decoder = TokenStream::new();
    let mut encoder = TokenStream::new();

    for (i, field) in input.unnamed.into_iter().enumerate() {
        let i = syn::Index::from(i);
        let field = CodecField::try_from(&field)?;
        let decoder_fn = field.decoder_fn();
        let encoder_fn = field.encoder_fn(&quote! { i.#i });

        decoder.append_all(quote! { #decoder_fn, });

        encoder.append_all(quote! {
            written += #encoder_fn;
        });
    }

    Ok((quote! { (#decoder) }, encoder))
}

struct CodecField<'a> {
    r#type: &'a Type,
    codec: CodecOption,
    constraints: Vec<PacketFieldConstraint>,
}

impl<'a> CodecField<'a> {
    fn decoder_fn(&self) -> TokenStream {
        let c = match self.constraints.len() {
            0 => quote! { ::protocol_primitives::Constraints::DEFAULT },
            _ => {
                let c = self
                    .constraints
                    .iter()
                    .fold(TokenStream::new(), |mut e, c| {
                        match c {
                            PacketFieldConstraint::Range { min, max } => {
                                e.append_all(quote! { range: (#min, #max), })
                            }
                            PacketFieldConstraint::Regex(_) => {}
                            PacketFieldConstraint::Custom(_) => {}
                        }
                        e
                    });
                quote! { ::protocol_primitives::Constraints { #c..Default::default() } }
            }
        };

        self.codec.decoder_fn(c, self.r#type)
    }

    fn encoder_fn(&self, i: &TokenStream) -> TokenStream {
        self.codec.encoder_fn(self.r#type, &quote! { #i })
    }
}

impl<'a> TryFrom<&'a Field> for CodecField<'a> {
    type Error = syn::Error;

    fn try_from(value: &'a Field) -> Result<Self> {
        let constraints = match value
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("constraints"))
        {
            Some(attr) => PacketFieldConstraint::parse(attr)?,
            None => vec![],
        };

        Ok(Self {
            r#type: &value.ty,
            codec: CodecOption::parse(&value.attrs)?.unwrap_or_else(|| CodecOption::Default),
            constraints,
        })
    }
}

impl std::fmt::Debug for CodecField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CodecField")
            .field("type", &self.r#type.to_token_stream().to_string())
            .field("codec", &self.codec)
            .field("constraints", &self.constraints)
            .finish()
    }
}

#[derive(Clone, Debug)]
enum CodecOption {
    Default,
    DynamicArray,
    Position,
    Varint,
    Custom { decoder: String, encoder: String },
}

impl CodecOption {
    fn parse(attrs: &Vec<Attribute>) -> Result<Option<CodecOption>> {
        Ok(match attrs
            .iter()
            .find_map(|attr| attr.path.is_ident("codec").then(|| attr.parse_meta()))
        {
            Some(Ok(Meta::List(list))) if list.nested.len() == 1 => list.nested.into_iter().next(),
            Some(Ok(_)) => panic!("codec expected one value"),
            Some(Err(err)) => return Err(err),
            _ => return Ok(None),
        }
        .map(|meta| match meta {
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("dyn_array") => Self::DynamicArray,
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("position") => Self::Position,
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("varint") => Self::Varint,
            NestedMeta::Meta(Meta::List(list)) if list.path.is_ident("custom") => {
                let (mut decoder, mut encoder) = (None, None);
                list.nested.iter().for_each(|meta| match meta {
                    NestedMeta::Meta(Meta::NameValue(val)) if val.path.is_ident("decoder") => {
                        match &val.lit {
                            Lit::Str(str) => decoder = Some(str.value()),
                            _ => panic!("codec decoder expected string"),
                        }
                    }
                    NestedMeta::Meta(Meta::NameValue(val)) if val.path.is_ident("encoder") => {
                        match &val.lit {
                            Lit::Str(str) => encoder = Some(str.value()),
                            _ => panic!("codec encoder expected string"),
                        }
                    }
                    _ => panic!("custom codec expected decoder and encoder"),
                });
                Self::Custom {
                    decoder: decoder.expect("custom codec expected decoder"),
                    encoder: encoder.expect("custom codec expected encoder"),
                }
            }
            _ => panic!("codec expected dynamic_array, position, varint or custom"),
        }))
    }

    fn decoder_fn(&self, c: TokenStream, ty: &Type) -> TokenStream {
        match self {
            Self::Custom { decoder, .. } => {
                let ident = Ident::new(&decoder, ty.span());
                quote! { #ident(src, &#c, ctx)? }
            }
            _ => {
                let path = self.path(ty).unwrap_or_else(|| quote! { #ty });
                quote! { <#path as ::protocol_primitives::Decoder>::decode(src, &#c, ctx)? }
            }
        }
    }

    fn encoder_fn(&self, ty: &Type, i: &TokenStream) -> TokenStream {
        match self {
            Self::Custom { encoder, .. } => {
                let ident = Ident::new(&encoder, ty.span());
                quote! { #ident(dst, &#i, ctx)? }
            }
            _ => {
                let path = self.path(ty).unwrap_or_else(|| quote! { #ty });
                quote! { <#path as ::protocol_primitives::Encoder<#ty>>::encode(dst, &#i, ctx)? }
            }
        }
    }

    fn path(&self, ty: &Type) -> Option<TokenStream> {
        Some(match self {
            Self::DynamicArray => quote! { ::protocol_primitives::DynArray<#ty> },
            Self::Position => quote! { ::protocol_primitives::Position },
            Self::Varint => quote! { ::protocol_primitives::Varint<#ty> },
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
enum PacketFieldConstraint {
    Range { min: isize, max: isize },
    Regex(String),
    Custom(String),
}

impl PacketFieldConstraint {
    fn parse(attr: &Attribute) -> Result<Vec<PacketFieldConstraint>> {
        let list = match attr.parse_meta()? {
            Meta::List(list) => list,
            _ => panic!("constraint expected list"),
        };

        Ok(list
            .nested
            .iter()
            .filter_map(|meta| match meta {
                NestedMeta::Meta(meta) => Some(meta),
                _ => panic!("constraint expected meta"),
            })
            .map(|meta| match meta {
                Meta::List(list) if list.path.is_ident("range") => {
                    Self::extract_size(list).unwrap()
                }
                Meta::NameValue(val) if val.path.is_ident("regex") => match &val.lit {
                    Lit::Str(str) => Self::Regex(str.value()),
                    _ => panic!("regex constraint expected string"),
                },
                Meta::NameValue(val) if val.path.is_ident("custom") => match &val.lit {
                    Lit::Str(str) => Self::Custom(str.value()),
                    _ => panic!("custom constraint expected string"),
                },
                _ => panic!("expected constraints: range, regex, custom"),
            })
            .collect())
    }

    fn extract_size(list: &MetaList) -> Result<Self> {
        let (mut min, mut max) = (0, i32::MAX as isize);
        let val = match list.nested.first() {
            Some(NestedMeta::Meta(Meta::NameValue(val))) => val,
            _ => panic!("size constraint expected int"),
        };

        let int = match &val.lit {
            Lit::Int(int) => int.base10_parse()?,
            _ => panic!("size constraint expected int"),
        };

        if val.path.is_ident("min") {
            min = int;
        } else if val.path.is_ident("max") {
            max = int;
        } else if val.path.is_ident("eq") {
            min = int;
            max = int;
        }

        Ok(Self::Range { min, max })
    }
}
