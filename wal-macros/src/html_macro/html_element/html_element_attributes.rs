use std::collections::HashMap;

use quote::quote_spanned;
use syn::parse::Parse;

use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};

pub struct HtmlElementAttributes {
    attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>,
    key: Option<HtmlAttribute>,
}

impl Parse for HtmlElementAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attributes = HashMap::<proc_macro2::Ident, HtmlAttributeValue>::new();
        let mut key = None;

        while HtmlAttribute::peek(input) {
            let attribute = input.parse::<HtmlAttribute>()?;
            if attribute.ident == "key" {
                if key.is_some() {
                    return Err(syn::Error::new(
                        attribute.ident.span(),
                        format!("Duplicate attribute `{}`", attribute.ident),
                    ));
                }
                key = Some(attribute);
            } else {
                let ident = attribute.ident.clone();
                if attributes
                    .insert(attribute.ident, attribute.value)
                    .is_some()
                {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Duplicate attribute `{}`", ident),
                    ));
                }
            }
        }

        Ok(HtmlElementAttributes { attributes, key })
    }
}

impl HtmlElementAttributes {
    pub fn new(mut attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>) -> Self {
        let key = attributes
            .remove_entry(&proc_macro2::Ident::new(
                "key",
                proc_macro2::Span::call_site(),
            ))
            .map(|(k, v)| HtmlAttribute { ident: k, value: v });

        HtmlElementAttributes { attributes, key }
    }
}

// TODO: instead of ident.span maybe use whole html attribute here
impl From<&HtmlElementAttributes> for Vec<proc_macro2::TokenStream> {
    fn from(element_attributes: &HtmlElementAttributes) -> Vec<proc_macro2::TokenStream> {
        let mut attributes: Vec<proc_macro2::TokenStream> = element_attributes
            .attributes
            .iter()
            .map(|(ident, value)| -> proc_macro2::TokenStream {
                let ident_str = ident.to_string();
                quote_spanned!(ident.span() => (::std::string::String::from(#ident_str), #value.to_string()))
            })
            .collect();

        if let Some(key_attr) = &element_attributes.key {
            let key_ident = &key_attr.ident;
            let key_ident_str = key_ident.to_string();
            let key_val = &key_attr.value;
            attributes
                .push(quote_spanned!(key_ident.span() => (::std::string::String::from(#key_ident_str), #key_val.to_string())));
        }

        attributes
    }
}
