use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct PacketField<'a> {
    pub ident: &'a syn::Ident,
    pub ty: TokenStream,
    pub is_varnum: bool,
    pub is_dynarray: bool,
    pub validator: Option<FieldValidator>,
}

impl<'a> PacketField<'a> {
    pub fn get_path(&self) -> TokenStream {
        let ty = &self.ty;
        if self.is_varnum {
            quote! { ::protocol_internal::VarNum::<#ty> }
        } else {
            if self.is_dynarray {
                quote! { ::protocol_internal::DynArray }
            } else {
                quote! { ::protocol_internal::ProtocolSupport }
            }
        }
    }

    pub fn calculate_len(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.get_path();
        quote! { #path::calculate_len(&self.#ident) }
    }

    pub fn serialize(&self) -> TokenStream {
        let ident = &self.ident;
        let path = self.get_path();
        quote! { #path::serialize(&self.#ident, &mut dst)?; }
    }

    pub fn deserialize(&self) -> TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        let method = if let Some(validator) = &self.validator {
            validator.deserialize(ty, self.is_varnum, self.is_dynarray)
        } else {
            let path = self.get_path();
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
        ty: &TokenStream,
        is_varnum: bool,
        is_dynarray: bool,
    ) -> proc_macro2::TokenStream {
        match self {
            FieldValidator::Range { min, max } => {
                if is_varnum {
                    quote! {
                        <::protocol_internal::VarNum<#ty> as ::protocol_internal::RangeValidatedSupport<#ty>>::deserialize(&mut src, #min, #max)
                    }
                } else {
                    if is_dynarray {
                        quote! {
                            <::protocol_internal::DynArray as ::protocol_internal::RangeValidatedSupport<#ty>>::deserialize(&mut src, #min, #max)
                        }
                    } else {
                        quote! {
                            <#ty as ::protocol_internal::RangeValidatedSupport>::deserialize(&mut src, #min, #max)
                        }
                    }
                }
            }
        }
    }
}