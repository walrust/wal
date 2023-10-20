use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

pub struct HtmlAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlAttributeValue,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = proc_macro2::Ident::parse_any(&input)?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(HtmlAttribute { ident, value })
    }
}

impl HtmlAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(proc_macro2::Ident::peek_any)
    }
}

pub enum HtmlAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
}

impl Parse for HtmlAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute_value = if input.peek(syn::Lit) {
            HtmlAttributeValue::Literal(input.parse()?)
        } else if let Ok(expr_block) = input.parse::<syn::ExprBlock>() {
            if expr_block.block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &expr_block,
                    "Expected a non-empty expression block",
                ));
            }
            HtmlAttributeValue::ExpressionBlock(expr_block)
        } else {
            return Err(input.error("Expected a literal or an expression block"));
        };

        Ok(attribute_value)
    }
}

impl ToTokens for HtmlAttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlAttributeValue::Literal(lit) => lit.to_tokens(tokens),
            HtmlAttributeValue::ExpressionBlock(expr_block) => expr_block.to_tokens(tokens),
        }
    }
}
