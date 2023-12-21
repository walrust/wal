use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub(crate) struct FragmentClosingTag {
    lt: syn::token::Lt,
    gt: syn::token::Gt,
}

impl Parse for FragmentClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let gt = input.parse()?;
        Ok(FragmentClosingTag { lt, gt })
    }
}

impl FragmentClosingTag {
    pub(crate) fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Lt) && input.peek2(syn::token::Slash) && input.peek3(syn::token::Gt)
    }
}
