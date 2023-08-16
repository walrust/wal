use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub struct HtmlListEndTag {
    lt: syn::token::Lt,
    gt: syn::token::Gt,
}

impl Parse for HtmlListEndTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let gt = input.parse()?;
        Ok(HtmlListEndTag { lt, gt })
    }
}

impl HtmlListEndTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Lt) && input.peek2(syn::token::Slash) && input.peek3(syn::token::Gt)
    }
}
