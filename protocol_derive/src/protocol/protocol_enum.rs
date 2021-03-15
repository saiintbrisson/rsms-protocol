use quote::quote;
use syn::{parse_quote, Attribute, Ident};

pub(crate) fn expand_enum(ident: &Ident, data_enum: &syn::DataEnum, attrs: &Vec<Attribute>) -> crate::LSDResult {
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

    let path = if extract_enum_meta(attrs).is_some() {
        quote! { ::protocol_internal::VarNum::<#ty> }
    } else {
        quote! { :protocol_internal::ProtocolSupport }
    };

    Ok((
        quote! { #path::calculate_len(&(*self as #ty)) }, 
        quote! { #path::serialize(&(*self as #ty), dst) }, 
        quote! {
            Ok(match #path::deserialize(src)? {
                #(#variants)*
                next_state => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("did not expect next state {}", next_state)))
            })
        }
    ))
}

fn extract_repr(ident: &Ident, attrs: &Vec<Attribute>) -> syn::Ident {
    match attrs.iter().find(|attr| attr.path == parse_quote!(repr)) {
        Some(attr) => attr.parse_args::<Ident>().ok(),
        None => None,
    }.unwrap_or(Ident::new("i32", ident.span()))
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
