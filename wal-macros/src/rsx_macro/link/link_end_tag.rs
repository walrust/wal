use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use super::LINK_TAG;

pub struct LinkEndTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for LinkEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = input.parse()?;
        let gt = input.parse()?;
        Ok(LinkEndTag { lt, name, gt })
    }
}

impl LinkEndTag {
    pub fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub fn peek(input: ParseStream) -> bool {
        let forked_input = input.fork();
        if forked_input.parse::<syn::token::Lt>().is_err()
            || forked_input.parse::<syn::token::Slash>().is_err()
        {
            return false;
        }

        match forked_input.parse::<proc_macro2::Ident>() {
            Ok(end_tag_name) => end_tag_name == LINK_TAG,
            Err(_) => false,
        }
    }
}
