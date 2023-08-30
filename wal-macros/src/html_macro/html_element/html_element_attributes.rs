use crate::html_macro::html_attribute::HtmlAttributeValue;
use std::collections::HashMap;

pub struct HtmlElementAttributes {
    attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>,
    key: Option<HtmlAttributeValue>,
}

impl HtmlElementAttributes {
    pub fn new(mut attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>) -> Self {
        let key = attributes.remove(&proc_macro2::Ident::new(
            "key",
            proc_macro2::Span::call_site(),
        ));

        HtmlElementAttributes { attributes, key }
    }
}

impl From<&HtmlElementAttributes> for HashMap<String, String> {
    fn from(element_attributes: &HtmlElementAttributes) -> HashMap<String, String> {
        let mut attributes: HashMap<String, String> = element_attributes
            .attributes
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        if let Some(key_val) = &element_attributes.key {
            attributes.insert(String::from("key"), key_val.to_string());
        }

        attributes
    }
}
