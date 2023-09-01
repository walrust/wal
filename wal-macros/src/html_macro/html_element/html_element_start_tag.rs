use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::Parse;

use super::html_element_attributes::HtmlElementAttributes;
use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};

pub struct HtmlElementStartTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    pub attributes: HtmlElementAttributes,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for HtmlElementStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;

        let mut attributes = HashMap::<proc_macro2::Ident, HtmlAttributeValue>::new();
        while HtmlAttribute::peek(input) {
            let attribute = input.parse::<HtmlAttribute>()?;
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

        let attributes = HtmlElementAttributes::new(attributes);

        let slash = input.parse().ok();
        let gt = input.parse()?;

        Ok(HtmlElementStartTag {
            lt,
            name,
            attributes,
            slash,
            gt,
        })
    }
}

impl HtmlElementStartTag {
    pub fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub fn is_void(&self) -> bool {
        const VOID_ELEMENTS: [&str; 14] = [
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
            "source", "track", "wbr",
        ];
        VOID_ELEMENTS.contains(&self.name.to_string().as_str())
    }

    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
