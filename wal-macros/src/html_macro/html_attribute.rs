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

        let value = if input.peek(syn::Lit) {
            HtmlAttributeValue::Literal(input.parse()?)
        } else {
            let expr_block = input.parse::<syn::ExprBlock>();

            if expr_block.is_err() {
                return Err(input.error("Expected a literal or an expression block"));
            }

            HtmlAttributeValue::ExpressionBlock(expr_block.unwrap())
        };

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

impl ToString for HtmlAttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(lit) => lit.into_token_stream().to_string(),
            Self::ExpressionBlock(expr_block) => expr_block.into_token_stream().to_string(),
        }
    }
}
