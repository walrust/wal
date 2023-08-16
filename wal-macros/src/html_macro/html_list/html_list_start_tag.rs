use quote::{quote, ToTokens};
use syn::parse::Parse;

pub struct HtmlListStartTag {
    lt: syn::token::Lt,
    // properties: ListProperties,
    gt: syn::token::Gt,
}

impl Parse for HtmlListStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        // here parse properties
        let gt = input.parse()?;

        Ok(HtmlListStartTag { lt, gt })
    }
}

impl HtmlListStartTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
