use std::hash::Hash;
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

// We want to guarantee uniqueness of attributes. Attributes are considered the same if their idents are the same.
impl PartialEq for HtmlAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for HtmlAttribute {}

impl Hash for HtmlAttribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

pub enum HtmlAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
}
