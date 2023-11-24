use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub struct WalClassAttribute {
    pub ident: proc_macro2::Ident,
    pub value: syn::ExprArray,
}

impl Parse for WalClassAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(WalClassAttribute { ident, value })
    }
}

impl ToTokens for WalClassAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens)
    }
}

impl WalClassAttribute {
    pub fn get_space_separated_values(&self) -> String {
        self.value
            .elems
            .iter()
            .map(|elem| elem.into_token_stream().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
