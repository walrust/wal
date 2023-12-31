use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub(crate) struct ElementClosingTag {
    lt: syn::token::Lt,
    pub(crate) name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for ElementClosingTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = input.parse()?;
        let gt = input.parse()?;
        Ok(ElementClosingTag { lt, name, gt })
    }
}

impl ElementClosingTag {
    pub(crate) fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }

    pub(crate) fn peek(opening_tag_name: &proc_macro2::Ident, input: ParseStream) -> bool {
        let forked_input = input.fork();
        if forked_input.parse::<syn::token::Lt>().is_err()
            || forked_input.parse::<syn::token::Slash>().is_err()
        {
            return false;
        }

        match forked_input.parse::<proc_macro2::Ident>() {
            Ok(closing_tag_name) => closing_tag_name == *opening_tag_name,
            Err(_) => false,
        }
    }
}
