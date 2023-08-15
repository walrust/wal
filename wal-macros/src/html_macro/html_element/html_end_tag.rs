use quote::{quote, ToTokens};
use syn::{ext::IdentExt, parse::Parse};

pub struct HtmlEndTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for HtmlEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = proc_macro2::Ident::parse_any(&input)?;
        let gt = input.parse()?;
        Ok(HtmlEndTag { lt, name, gt })
    }
}

impl HtmlEndTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
