use std::collections::HashMap;

use quote::{quote, quote_spanned};
use syn::{parse::Parse, spanned::Spanned};

use crate::attributes::{
    event_attribute::EventAttributeValue,
    normal_attribute::{NormalAttribute, NormalAttributeValue},
    process_specialized_attribute, process_unspecialized_attribute,
    wal_class_attribute::WalClassAttribute,
};

use super::element_attribute::{ElementAttribute, CLASS_ATTR};

pub(crate) struct ElementAttributes {
    normal: HashMap<proc_macro2::Ident, NormalAttributeValue>,
    events: HashMap<proc_macro2::Ident, syn::ExprBlock>,
    key: Option<NormalAttribute>,
    class: Option<NormalAttribute>,
    wal_class: Option<WalClassAttribute>,
}

impl Parse for ElementAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut normal = HashMap::<proc_macro2::Ident, NormalAttributeValue>::new();
        let mut events = HashMap::<proc_macro2::Ident, EventAttributeValue>::new();
        let mut key = None;
        let mut class = None;
        let mut wal_class = None;

        while ElementAttribute::peek(input) {
            let incoming_attribute = input.parse::<ElementAttribute>()?;
            Self::process_attribute(
                &mut normal,
                &mut events,
                &mut key,
                &mut class,
                &mut wal_class,
                incoming_attribute,
            )?;
        }

        Ok(ElementAttributes {
            normal,
            events,
            key,
            class,
            wal_class,
        })
    }
}

impl ElementAttributes {
    fn process_attribute(
        normal: &mut HashMap<proc_macro2::Ident, NormalAttributeValue>,
        events: &mut HashMap<proc_macro2::Ident, EventAttributeValue>,
        key: &mut Option<NormalAttribute>,
        class: &mut Option<NormalAttribute>,
        wal_class: &mut Option<WalClassAttribute>,
        incoming_attribute: ElementAttribute,
    ) -> syn::Result<()> {
        match incoming_attribute {
            ElementAttribute::Normal(incoming_attribute) => {
                process_unspecialized_attribute(normal, &incoming_attribute)
            }
            ElementAttribute::Event(incoming_attribute) => {
                process_unspecialized_attribute(events, &incoming_attribute)
            }
            ElementAttribute::Key(incoming_attribute) => {
                process_specialized_attribute(key, incoming_attribute)
            }
            ElementAttribute::Class(incoming_attribute) => {
                process_specialized_attribute(class, incoming_attribute)
            }
            ElementAttribute::WalClass(incoming_attribute) => {
                process_specialized_attribute(wal_class, incoming_attribute)
            }
        }
    }

    pub(crate) fn get_attributes_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        let mut attributes_token_stream: Vec<proc_macro2::TokenStream> = self
            .normal
            .iter()
            .map(|(ident, value)| -> proc_macro2::TokenStream {
                let ident_str = ident.to_string();
                quote_spanned!(value.error_span() => (::std::string::String::from(#ident_str), #value.to_string()))
            })
            .collect();

        if let Some(class_attribute_token_stream) = self.get_class_attribute_token_stream() {
            attributes_token_stream.push(class_attribute_token_stream);
        }

        attributes_token_stream
    }

    fn get_class_attribute_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        match (&self.class, &self.wal_class) {
            (Some(class), Some(wal_class)) => {
                let class_value = &class.value;
                let class_value =
                    quote_spanned!(class_value.error_span() => #class_value.to_string());
                let wal_class_value = wal_class.get_values_token_stream();
                Some(quote!((
                    ::std::string::String::from(#CLASS_ATTR),
                    ::std::format!("{} {}", #class_value, ::std::vec![#(#wal_class_value),*].join(" "))
                )))
            }
            (Some(class), None) => {
                let value = &class.value;
                Some(quote_spanned!(value.error_span() => (
                    ::std::string::String::from(#CLASS_ATTR),
                    #value.to_string()
                )))
            }
            (None, Some(wal_class)) => {
                let values = wal_class.get_values_token_stream();
                Some(quote!((
                    ::std::string::String::from(#CLASS_ATTR),
                    ::std::vec![#(#values),*].join(" ")
                )))
            }
            (None, None) => None,
        }
    }

    pub(crate) fn get_key_attribute_token_stream(&self) -> proc_macro2::TokenStream {
        NormalAttribute::get_key_attribute_token_stream(self.key.as_ref())
    }

    pub(crate) fn get_event_handlers_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        self.events
            .iter()
            .map(|(ident, expr_block)| -> proc_macro2::TokenStream {
                quote_spanned!(expr_block.span() => ::wal::events::EventHandler::new(
                    #[allow(unused_braces)]
                    ::wal::events::#ident(#expr_block)
                ))
            })
            .collect()
    }
}
