use quote::{quote, quote_spanned, ToTokens};
use syn::parse::Parse;

use crate::rsx_macro::attributes::{normal_attribute::NormalAttribute, KEY_ATTR};

use super::TO_ATTR;

pub struct LinkStartTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    pub to: NormalAttribute,
    pub key: Option<NormalAttribute>,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for LinkStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;

        let mut key = None;
        let mut to = None;
        while NormalAttribute::peek(input) {
            let attribute = input.parse::<NormalAttribute>()?;
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

        Ok(LinkStartTag {
            lt,
            name,
            to: to.unwrap(),
            key,
            slash,
            gt,
        })
    }
}

impl LinkStartTag {
    fn process_attribute(
        key: &mut Option<NormalAttribute>,
        to: &mut Option<NormalAttribute>,
        attribute: NormalAttribute,
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
        attribute_storage: &mut Option<NormalAttribute>,
        attribute: NormalAttribute,
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

impl LinkStartTag {
    pub fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub(crate) fn get_to_attribute_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        let mut attributes = Vec::new();

        let to_value = &self.to.value;
        attributes
            .push(quote_spanned!(to_value.error_span() => (::std::string::String::from("href"), #to_value.to_string())));
        attributes
            .push(quote_spanned!(to_value.error_span() => (::std::string::String::from("data_link"), #to_value.to_string())));

        attributes
    }

    pub(crate) fn get_key_attribute_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_value = &key.value;
            quote_spanned!(key_value.error_span() => Some(#key_value.to_string()))
        } else {
            quote!(None)
        }
    }
}
