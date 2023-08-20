use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};
use std::collections::HashMap;

pub struct HtmlElementAttributes {
    _attributes: Vec<HtmlAttribute>,
    _key: Option<HtmlAttributeValue>,
}

impl HtmlElementAttributes {
    pub fn new(mut attributes: HashMap<proc_macro2::Ident, HtmlAttributeValue>) -> Self {
        let key = attributes.remove(&proc_macro2::Ident::new(
            "key",
            proc_macro2::Span::call_site(),
        ));

        HtmlElementAttributes {
            _attributes: attributes
                .into_iter()
                .map(|(ident, value)| HtmlAttribute { ident, value })
                .collect(),
            _key: key,
        }
    }
}
