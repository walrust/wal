use std::collections::HashMap;

use quote::quote_spanned;

use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};

pub struct HtmlElementAttributes {
    attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>,
    key: Option<HtmlAttribute>,
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
