use quote::{quote, quote_spanned, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::Attribute;

pub(crate) struct NormalAttribute {
    pub(crate) ident: proc_macro2::Ident,
    pub(crate) value: NormalAttributeValue,
}

impl Parse for NormalAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = proc_macro2::Ident::parse_any(input)?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(NormalAttribute { ident, value })
    }
}

impl Attribute for NormalAttribute {
    type AttributeValue = NormalAttributeValue;

    fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    fn value(&self) -> &Self::AttributeValue {
        &self.value
    }
}

impl NormalAttribute {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(proc_macro2::Ident::peek_any)
    }

    pub(crate) fn get_key_attribute_token_stream(
        key_attribute: Option<&Self>,
    ) -> proc_macro2::TokenStream {
        key_attribute.map_or_else(
            || quote!(None),
            |key_attribute| {
                let key_value = &key_attribute.value;
                quote_spanned!(key_value.error_span() => Some(#key_value.to_string()))
            },
        )
    }
}

#[derive(Clone)]
pub(crate) enum NormalAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
}

impl Parse for NormalAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute_value = if input.peek(syn::Lit) {
            NormalAttributeValue::Literal(input.parse()?)
        } else if let Ok(expr_block) = input.parse::<syn::ExprBlock>() {
            if expr_block.block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &expr_block,
                    "Expected a non-empty expression block",
                ));
            }
            NormalAttributeValue::ExpressionBlock(expr_block)
        } else {
            return Err(input.error("Expected a literal or an expression block"));
        };

        Ok(attribute_value)
    }
}

impl ToTokens for NormalAttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            NormalAttributeValue::Literal(lit) => lit.to_tokens(tokens),
            NormalAttributeValue::ExpressionBlock(expr_block) => expr_block.to_tokens(tokens),
        }
    }
}

impl NormalAttributeValue {
    pub(crate) fn error_span(&self) -> proc_macro2::Span {
        match self {
            NormalAttributeValue::Literal(lit) => lit.span(),
            NormalAttributeValue::ExpressionBlock(expr_block) => expr_block.span(),
        }
    }
}
