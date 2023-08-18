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

        let key = input.parse::<HtmlAttribute>().ok();

        if !input.peek(syn::token::Gt) {
            return Err(input.error("Fragment supprots only a single `key` attribute"));
        }

        if let Some(key_attr) = &key {
            if key_attr.ident != "key" {
                return Err(syn::Error::new_spanned(
                    key_attr.to_spanned(),
                    "Fragment supports only the `key` attribute",
                ));
            }
        }

        let key = key.map(|key| key.value);

        let gt = input.parse()?;

        Ok(HtmlFragmentStartTag { lt, key, gt })
    }
}

impl HtmlFragmentStartTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
