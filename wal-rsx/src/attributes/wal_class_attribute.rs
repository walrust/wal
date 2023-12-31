use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::Attribute;

pub(crate) type WalClassAttributeValue = syn::ExprArray;

pub(crate) struct WalClassAttribute {
    pub(crate) ident: proc_macro2::Ident,
    pub(crate) value: WalClassAttributeValue,
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
        self.value.to_tokens(tokens);
    }
}

impl Attribute for WalClassAttribute {
    type AttributeValue = WalClassAttributeValue;

    fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    fn value(&self) -> &Self::AttributeValue {
        &self.value
    }
}

impl WalClassAttribute {
    pub(crate) fn get_values_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        self.value
            .elems
            .iter()
            .map(|elem| quote_spanned!(elem.span() => #elem.to_string()))
            .collect()
    }
}
