use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub struct PropsAttribute {
    pub ident: proc_macro2::Ident,
    pub value: PropsAttributeValue,
}

impl Parse for PropsAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(PropsAttribute { ident, value })
    }
}

pub enum PropsAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
    StructExpression(syn::ExprStruct),
}

impl Parse for PropsAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute_value = if input.peek(syn::Lit) {
            PropsAttributeValue::Literal(input.parse()?)
        } else if input.fork().parse::<syn::ExprStruct>().is_ok() {
            PropsAttributeValue::StructExpression(input.parse()?)
        } else if let Ok(expr_block) = input.parse::<syn::ExprBlock>() {
            if expr_block.block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &expr_block,
                    "Expected a non-empty expression block",
                ));
            }
            PropsAttributeValue::ExpressionBlock(expr_block)
        } else {
            return Err(input
                .error("Expected a literal, a struct literal expression or an expression block"));
        };

        Ok(attribute_value)
    }
}

impl ToTokens for PropsAttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            PropsAttributeValue::Literal(lit) => lit.to_tokens(tokens),
            PropsAttributeValue::StructExpression(expr_struct) => expr_struct.to_tokens(tokens),
            PropsAttributeValue::ExpressionBlock(expr_block) => expr_block.to_tokens(tokens),
        }
    }
}
