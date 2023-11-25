use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub struct NormalAttribute {
    pub ident: proc_macro2::Ident,
    pub value: NormalAttributeValue,
}

impl Parse for NormalAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(NormalAttribute { ident, value })
    }
}

impl NormalAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }
}

pub enum NormalAttributeValue {
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
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            NormalAttributeValue::Literal(lit) => lit.span(),
            NormalAttributeValue::ExpressionBlock(expr_block) => expr_block.span(),
        }
    }
}
