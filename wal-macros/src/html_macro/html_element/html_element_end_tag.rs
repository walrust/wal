use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

pub struct HtmlElementEndTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for HtmlElementEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = proc_macro2::Ident::parse_any(&input)?;
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
        let input = input.fork();
        if input.parse::<syn::token::Lt>().is_err() {
            return false;
        }
        if input.parse::<syn::token::Slash>().is_err() {
            return false;
        }

        match proc_macro2::Ident::parse_any(&input) {
            Ok(end_tag_name) => end_tag_name == *start_tag_name,
            Err(_) => return false,
        }
    }
}
