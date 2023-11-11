use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::html_macro::{html_attribute::HtmlAttribute, KEY_ATTR, TO_ATTR};

pub struct HtmlLinkStartTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    pub to: HtmlAttribute,
    pub key: Option<HtmlAttribute>,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for HtmlLinkStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;

        let mut key = None;
        let mut to = None;
        while HtmlAttribute::peek(input) {
            let attribute = input.parse::<HtmlAttribute>()?;
            Self::process_attribute(&mut key, &mut to, attribute, &name)?;
        }

        if to.is_none() {
            return Err(syn::Error::new(
                name.span(),
                format!("`{}` requires `{}` attribute", name, TO_ATTR),
            ));
        }

        let slash = input.parse().ok();
        let gt = input.parse()?;

        Ok(HtmlLinkStartTag {
            lt,
            name,
            to: to.unwrap(),
            key,
            slash,
            gt,
        })
    }
}

impl HtmlLinkStartTag {
    fn process_attribute(
        key: &mut Option<HtmlAttribute>,
        to: &mut Option<HtmlAttribute>,
        attribute: HtmlAttribute,
        name: &proc_macro2::Ident,
    ) -> syn::Result<()> {
        if attribute.ident == KEY_ATTR {
            Self::process_supported_attribute(key, attribute)
        } else if attribute.ident == TO_ATTR {
            Self::process_supported_attribute(to, attribute)
        } else {
            Err(syn::Error::new(
                attribute.ident.span(),
                format!(
                    "Unsupported attribute `{}`. `{}` supports only `{}` and `{}` attributes",
                    attribute.ident, name, KEY_ATTR, TO_ATTR
                ),
            ))
        }
    }

    fn process_supported_attribute(
        attribute_storage: &mut Option<HtmlAttribute>,
        attribute: HtmlAttribute,
    ) -> syn::Result<()> {
        if attribute_storage.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *attribute_storage = Some(attribute);
        Ok(())
    }
}

impl HtmlLinkStartTag {
    pub fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
