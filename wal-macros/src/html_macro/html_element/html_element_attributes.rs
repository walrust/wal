use std::collections::HashMap;

use quote::quote_spanned;
use syn::parse::Parse;

use crate::html_macro::{
    html_attribute::{HtmlAttribute, HtmlAttributeValue},
    KEY_ATTR,
};

pub struct HtmlElementAttributes {
    attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>,
    events: HashMap<proc_macro2::Ident, syn::ExprBlock>,
    key: Option<HtmlAttribute>,
}

impl Parse for HtmlElementAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attributes = HashMap::<proc_macro2::Ident, HtmlAttributeValue>::new();
        let mut events = HashMap::<proc_macro2::Ident, syn::ExprBlock>::new();
        let mut key = None;

        while HtmlAttribute::peek(input) {
            let attribute = input.parse::<HtmlAttribute>()?;
            Self::process_attribute(&mut attributes, &mut events, &mut key, attribute)?;
        }

        Ok(HtmlElementAttributes {
            attributes,
            events,
            key,
        })
    }
}

impl HtmlElementAttributes {
    fn process_attribute(
        attributes: &mut HashMap<proc_macro2::Ident, HtmlAttributeValue>,
        events: &mut HashMap<proc_macro2::Ident, syn::ExprBlock>,
        key: &mut Option<HtmlAttribute>,
        attribute: HtmlAttribute,
    ) -> syn::Result<()> {
        if attribute.ident == KEY_ATTR {
            Self::process_key_attribute(key, attribute)
        } else if attribute.is_event() {
            Self::process_event_attribute(events, attribute)
        } else {
            Self::process_normal_attribute(attributes, attribute)
        }
    }

    fn process_key_attribute(
        key: &mut Option<HtmlAttribute>,
        attribute: HtmlAttribute,
    ) -> syn::Result<()> {
        if key.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *key = Some(attribute);
        Ok(())
    }

    fn process_event_attribute(
        events: &mut HashMap<proc_macro2::Ident, syn::ExprBlock>,
        attribute: HtmlAttribute,
    ) -> syn::Result<()> {
        if let HtmlAttributeValue::ExpressionBlock(expr_block) = attribute.value {
            if events.insert(attribute.ident.clone(), expr_block).is_some() {
                return Err(syn::Error::new(
                    attribute.ident.span(),
                    format!("Duplicate event attribute `{}`", attribute.ident),
                ));
            }
            Ok(())
        } else {
            Err(syn::Error::new(
                attribute.ident.span(),
                format!(
                    "Expected an expression block for event attribute `{}`",
                    attribute.ident
                ),
            ))
        }
    }

    fn process_normal_attribute(
        attributes: &mut HashMap<proc_macro2::Ident, HtmlAttributeValue>,
        attribute: HtmlAttribute,
    ) -> syn::Result<()> {
        if attributes
            .insert(attribute.ident.clone(), attribute.value)
            .is_some()
        {
            Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_attributes_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        let mut attributes: Vec<proc_macro2::TokenStream> = self
            .attributes
            .iter()
            .map(|(ident, value)| -> proc_macro2::TokenStream {
                let ident_str = ident.to_string();
                quote_spanned!(ident.span() => (::std::string::String::from(#ident_str), #value.to_string()))
            })
            .collect();

        if let Some(key_attr) = &self.key {
            let key_ident = &key_attr.ident;
            let key_ident_str = key_ident.to_string();
            let key_val = &key_attr.value;
            attributes
                .push(quote_spanned!(key_ident.span() => (::std::string::String::from(#key_ident_str), #key_val.to_string())));
        }

        attributes
    }

    pub(crate) fn get_event_handlers_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        self.events
            .iter()
            .map(|(ident, expr_block)| -> proc_macro2::TokenStream {
                quote_spanned!(ident.span() => ::wal::events::EventHandler::new(
                    #[allow(unused_braces)]
                    ::wal::events::#ident(#expr_block)
                ))
            })
            .collect()
    }
}
