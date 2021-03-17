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
        let path = self.protocol_type.get_path(&self.ty);
        quote! { #path::calculate_len(&self.#ident) }
    }

    pub fn serialize(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.protocol_type.get_path(&self.ty);
        quote! { #path::serialize(&self.#ident, &mut dst)?; }
    }

    pub fn deserialize(&self) -> TokenStream {
        let ident = &self.ident;

        let method = if let Some(validator) = &self.validator {
            let path = self.protocol_type.get_range_validator_path(&self.ty);
            validator.deserialize(&path)
        } else {
            let path = self.protocol_type.get_path(&self.ty);
            quote! { #path::deserialize(&mut src) }
        };

        quote! {
            #ident: #method?,
        }
    }
}

#[derive(Debug)]
pub(crate) enum FieldValidator {
    Range { min: usize, max: usize },
}

impl FieldValidator {
    pub fn deserialize(
        &self,
        path: &TokenStream,
    ) -> proc_macro2::TokenStream {
        match self {
            FieldValidator::Range { min, max } => quote! {
                #path::deserialize(&mut src, #min, #max)
            }
        }
    }
}

#[derive(Debug)]
pub enum FieldType {
    VarNum,
    Position,
    DynArray,
    Default,
}

impl FieldType {
    pub fn get_path(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldType::VarNum => quote! { ::protocol_internal::VarNum::<#ty> },
            FieldType::Position => quote! { ::protocol_internal::ProtocolPositionSupport },
            FieldType::DynArray => quote! { ::protocol_internal::DynArray },
            FieldType::Default => quote! { <#ty as ::protocol_internal::ProtocolSupport> },
        }
    }

    pub fn get_range_validator_path(&self, ty: &TokenStream) -> TokenStream {
        match self {
            FieldType::VarNum => quote! { <::protocol_internal::VarNum<#ty> as ::protocol_internal::RangeValidatedSupport<#ty>> },
            FieldType::DynArray => quote! { <::protocol_internal::DynArray as ::protocol_internal::RangeValidatedSupport<#ty>> },
            FieldType::Default => quote! { <#ty as ::protocol_internal::RangeValidatedSupport> },
            _ => panic!(""),
        }
    }
}
