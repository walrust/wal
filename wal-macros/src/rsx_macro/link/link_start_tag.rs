use quote::{quote, quote_spanned, ToTokens};
use syn::parse::Parse;

use crate::rsx_macro::attributes::{
    normal_attribute::NormalAttribute, process_specialized_attribute, KEY_ATTR,
};

use super::TO_ATTR;

pub(crate) struct LinkStartTag {
    lt: syn::token::Lt,
    pub(crate) name: proc_macro2::Ident,
    pub(crate) to: NormalAttribute,
    pub(crate) key: Option<NormalAttribute>,
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
            let incoming_attribute = input.parse::<NormalAttribute>()?;
            Self::process_attribute(&mut key, &mut to, incoming_attribute, &name)?;
        }

        if to.is_none() {
            return Err(syn::Error::new(
                name.span(),
                format!("`{name}` requires `{TO_ATTR}` attribute"),
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
        incoming_attribute: NormalAttribute,
        tag_ident: &proc_macro2::Ident,
    ) -> syn::Result<()> {
        if incoming_attribute.ident == KEY_ATTR {
            process_specialized_attribute(key, incoming_attribute)
        } else if incoming_attribute.ident == TO_ATTR {
            process_specialized_attribute(to, incoming_attribute)
        } else {
            Err(syn::Error::new(
                incoming_attribute.ident.span(),
                format!(
                    "Unsupported attribute `{}`. `{}` supports only `{}` and `{}` attributes",
                    incoming_attribute.ident, tag_ident, KEY_ATTR, TO_ATTR
                ),
            ))
        }
    }
}

impl LinkStartTag {
    pub(crate) fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub(crate) fn error_spanned(&self) -> impl ToTokens {
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
        NormalAttribute::get_key_attribute_token_stream(self.key.as_ref())
    }
}
