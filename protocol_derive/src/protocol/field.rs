use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct StructField<'a> {
    pub ident: &'a syn::Ident,
    pub ty: TokenStream,
    pub protocol_type: FieldType,
    pub validator: Option<FieldValidator>,
}

impl<'a> StructField<'a> {
    pub fn calculate_len(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.protocol_type.get_path_ser(&self.ty);
        quote! { #path::calculate_len(&self.#ident) }
    }

    pub fn serialize(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.protocol_type.get_path_ser(&self.ty);
        quote! { #path::serialize(&self.#ident, &mut dst)?; }
    }

    pub fn deserialize(&self) -> TokenStream {
        let ident = &self.ident;

        let method = if let Some(validator) = &self.validator {
            let path = match validator {
                FieldValidator::Range { .. } => {
                    self.protocol_type.get_range_validator_path(&self.ty)
                }
                _ => self.protocol_type.get_path_de(&self.ty),
            };

            validator.deserialize(&path)
        } else {
            let path = self.protocol_type.get_path_de(&self.ty);
            quote! { #path::deserialize(&mut src) }
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
    pub fn deserialize(&self, path: &TokenStream) -> proc_macro2::TokenStream {
        match self {
            FieldValidator::Fixed(len) => quote! {
                #path::deserialize(&mut src, #len)
            },
            FieldValidator::Range { min, max } => quote! {
                #path::deserialize(&mut src, #min, #max)
            },
            FieldValidator::Regex(regex) => quote! {
                {
                    ::lazy_static::lazy_static! { static ref REGEX: ::regex::Regex = regex::Regex::new(#regex).unwrap(); };
                    #path::deserialize(&mut src, &REGEX)
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
                quote! { <#ty as ::protocol_internal::ProtocolSupportSerializer> }
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
                quote! { <#ty as ::protocol_internal::ProtocolSupportDeserializer> }
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
