use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::element_attributes::ElementAttributes;

pub(crate) struct ElementStartTag {
    lt: syn::token::Lt,
    pub(crate) name: proc_macro2::Ident,
    pub(crate) attributes: ElementAttributes,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for ElementStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;
        let attributes = input.parse()?;
        let slash = input.parse().ok();
        let gt = input.parse()?;

        Ok(ElementStartTag {
            lt,
            name,
            attributes,
            slash,
            gt,
        })
    }
}

impl ElementStartTag {
    pub(crate) fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub(crate) fn is_void(&self) -> bool {
        const VOID_ELEMENTS: [&str; 14] = [
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
            "source", "track", "wbr",
        ];
        VOID_ELEMENTS.contains(&self.name.to_string().as_str())
    }

    pub(crate) fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
