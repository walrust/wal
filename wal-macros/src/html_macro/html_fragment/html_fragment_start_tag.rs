use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::html_macro::{
    html_attribute::{HtmlAttribute, HtmlAttributeValue},
    KEY_ATTR,
};

pub struct HtmlFragmentStartTag {
    lt: syn::token::Lt,
    _key: Option<HtmlAttributeValue>,
    gt: syn::token::Gt,
}

impl Parse for HtmlFragmentStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;

        if input.peek(syn::token::Gt) {
            return Ok(HtmlFragmentStartTag {
                lt,
                _key: None,
                gt: input.parse()?,
            });
        }

        let attribute = input.parse::<HtmlAttribute>()?;

        if attribute.ident != KEY_ATTR {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Fragment supports only the `{}` attribute", KEY_ATTR),
            ));
        }

        if !input.peek(syn::token::Gt) {
            return Err(input.error(format!(
                "Fragment supports only a single `{}` attribute",
                KEY_ATTR
            )));
        }

        let gt = input.parse()?;

        Ok(HtmlFragmentStartTag {
            lt,
            _key: Some(attribute.value),
            gt,
        })
    }
}

impl HtmlFragmentStartTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
