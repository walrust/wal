use quote::{quote, ToTokens};
use syn::parse::Parse;

pub struct HtmlElementStartTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    // properties: ElementProperties,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for HtmlElementStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;
        // here parse properties
        let slash = input.parse().ok();
        let gt = input.parse()?;

        Ok(HtmlElementStartTag {
            lt,
            name,
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
        self.name == "area"
            || self.name == "base"
            || self.name == "br"
            || self.name == "col"
            || self.name == "embed"
            || self.name == "hr"
            || self.name == "img"
            || self.name == "input"
            || self.name == "link"
            || self.name == "meta"
            || self.name == "param"
            || self.name == "source"
            || self.name == "track"
            || self.name == "wbr"
    }

    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
