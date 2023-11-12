use quote::{quote, quote_spanned, ToTokens};
use syn::parse::Parse;

use crate::html_macro::{html_attribute::HtmlAttribute, KEY_ATTR};

pub struct HtmlFragmentStartTag {
    lt: syn::token::Lt,
    key: Option<HtmlAttribute>,
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
            key: Some(attribute),
            gt,
        })
    }
}

impl HtmlFragmentStartTag {
    pub(crate) fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub(crate) fn get_key_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_val = &key.value;
            quote_spanned!(key.ident.span() => Some(#key_val.to_string()))
        } else {
            quote!(None)
        }
    }
}
