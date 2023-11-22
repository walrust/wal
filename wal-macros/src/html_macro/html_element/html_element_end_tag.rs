use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub struct HtmlElementEndTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for HtmlElementEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = input.parse()?;
        let gt = input.parse()?;
        Ok(HtmlElementEndTag { lt, name, gt })
    }
}

impl HtmlElementEndTag {
    pub fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub fn peek(start_tag_name: &proc_macro2::Ident, input: ParseStream) -> bool {
        let forked_input = input.fork();
        if forked_input.parse::<syn::token::Lt>().is_err()
            || forked_input.parse::<syn::token::Slash>().is_err()
        {
            return false;
        }

        match forked_input.parse::<proc_macro2::Ident>() {
            Ok(end_tag_name) => end_tag_name == *start_tag_name,
            Err(_) => false,
        }
    }
}
