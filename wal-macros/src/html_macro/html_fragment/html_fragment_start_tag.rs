use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};

pub struct HtmlFragmentStartTag {
    lt: syn::token::Lt,
    key: Option<HtmlAttributeValue>,
    gt: syn::token::Gt,
}

impl Parse for HtmlFragmentStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;

        if input.peek(syn::token::Gt) {
            return Ok(HtmlFragmentStartTag {
                lt,
                key: None,
                gt: input.parse()?,
            });
        }

        let attribute = input.parse::<HtmlAttribute>()?;

        if !input.peek(syn::token::Gt) {
            let error_message = "Fragment supports only a single `key` attribute";
            if attribute.ident == "key" {
                return Err(input.error(error_message));
            } else {
                return Err(syn::Error::new(attribute.ident.span(), error_message));
            }
        }

        if attribute.ident != "key" {
            return Err(syn::Error::new(
                attribute.ident.span(),
                "Fragment supports only the `key` attribute",
            ));
        }

        let gt = input.parse()?;

        Ok(HtmlFragmentStartTag {
            lt,
            key: Some(attribute.value),
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
