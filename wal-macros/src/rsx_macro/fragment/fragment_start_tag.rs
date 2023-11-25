use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::rsx_macro::attributes::{normal_attribute::NormalAttribute, KEY_ATTR};

pub(crate) struct FragmentStartTag {
    lt: syn::token::Lt,
    key: Option<NormalAttribute>,
    gt: syn::token::Gt,
}

impl Parse for FragmentStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;

        if input.peek(syn::token::Gt) {
            return Ok(FragmentStartTag {
                lt,
                key: None,
                gt: input.parse()?,
            });
        }

        let attribute = input.parse::<NormalAttribute>()?;

        if attribute.ident != KEY_ATTR {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Fragment supports only the `{KEY_ATTR}` attribute"),
            ));
        }

        if !input.peek(syn::token::Gt) {
            return Err(input.error(format!(
                "Fragment supports only a single `{KEY_ATTR}` attribute"
            )));
        }

        let gt = input.parse()?;

        Ok(FragmentStartTag {
            lt,
            key: Some(attribute),
            gt,
        })
    }
}

impl FragmentStartTag {
    pub(crate) fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub(crate) fn get_key_attribute_token_stream(&self) -> proc_macro2::TokenStream {
        NormalAttribute::get_key_attribute_token_stream(self.key.as_ref())
    }
}
