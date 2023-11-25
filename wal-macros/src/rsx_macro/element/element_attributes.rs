use std::collections::HashMap;

use quote::{quote, quote_spanned};
use syn::parse::Parse;

use crate::rsx_macro::attributes::{
    event_attribute::{EventAttribute, EventAttributeValue},
    normal_attribute::{NormalAttribute, NormalAttributeValue},
    wal_class_attribute::WalClassAttribute,
};

use super::element_attribute::{ElementAttribute, CLASS_ATTR};

pub struct ElementAttributes {
    normal: HashMap<proc_macro2::Ident, NormalAttributeValue>,
    events: HashMap<proc_macro2::Ident, syn::ExprBlock>,
    pub key: Option<NormalAttribute>,
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
            let attribute = input.parse::<ElementAttribute>()?;
            Self::process_attribute(
                &mut normal,
                &mut events,
                &mut key,
                &mut class,
                &mut wal_class,
                attribute,
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
        attribute: ElementAttribute,
    ) -> syn::Result<()> {
        match attribute {
            ElementAttribute::Normal(attribute) => {
                Self::process_normal_attribute(normal, attribute)
            }
            ElementAttribute::Event(attribute) => Self::process_event_attribute(events, attribute),
            ElementAttribute::Key(attribute) => Self::process_key_attribute(key, attribute),
            ElementAttribute::Class(attribute) => Self::process_class_attribute(class, attribute),
            ElementAttribute::WalClass(attribute) => {
                Self::process_wal_class_attribute(wal_class, attribute)
            }
        }
    }

    fn process_normal_attribute(
        normal: &mut HashMap<proc_macro2::Ident, NormalAttributeValue>,
        attribute: NormalAttribute,
    ) -> syn::Result<()> {
        if normal
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

    fn process_event_attribute(
        events: &mut HashMap<proc_macro2::Ident, EventAttributeValue>,
        attribute: EventAttribute,
    ) -> syn::Result<()> {
        if events
            .insert(attribute.ident.clone(), attribute.value)
            .is_some()
        {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate event attribute `{}`", attribute.ident),
            ));
        }
        Ok(())
    }

    fn process_key_attribute(
        key: &mut Option<NormalAttribute>,
        attribute: NormalAttribute,
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

    fn process_class_attribute(
        class: &mut Option<NormalAttribute>,
        attribute: NormalAttribute,
    ) -> syn::Result<()> {
        if class.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *class = Some(attribute);
        Ok(())
    }

    fn process_wal_class_attribute(
        wal_class: &mut Option<WalClassAttribute>,
        attribute: WalClassAttribute,
    ) -> syn::Result<()> {
        if wal_class.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *wal_class = Some(attribute);
        Ok(())
    }

    pub(crate) fn get_attributes_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        let mut atttributes_token_stream: Vec<proc_macro2::TokenStream> = self
            .normal
            .iter()
            .map(|(ident, value)| -> proc_macro2::TokenStream {
                let ident_str = ident.to_string();
                quote_spanned!(ident.span() => (::std::string::String::from(#ident_str), #value.to_string()))
            })
            .collect();

        if let Some(class_attribute_token_stream) = self.get_class_attribute_token_stream() {
            atttributes_token_stream.push(class_attribute_token_stream);
        }

        atttributes_token_stream
    }

    fn get_class_attribute_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        match (&self.class, &self.wal_class) {
            (Some(class), Some(wal_class)) => {
                let class_value = &class.value;
                let class_value = quote_spanned!(class_value.span() => #class_value.to_string());
                let wal_class_value = wal_class.get_values_token_stream();
                Some(quote!((
                    ::std::string::String::from(#CLASS_ATTR),
                    ::std::format!("{} {}", #class_value, ::std::vec![#(#wal_class_value),*].join(" "))
                )))
            }
            (Some(class), None) => {
                let value = &class.value;
                Some(quote_spanned!(value.span() => (
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

    pub(crate) fn get_key_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_val = &key.value;
            quote_spanned!(key.ident.span() => Some(#key_val.to_string()))
        } else {
            quote!(None)
        }
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
