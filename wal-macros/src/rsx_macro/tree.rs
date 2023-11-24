use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use super::{
    component::Component, element::Element, expression_block::ExpressionBlock, fragment::Fragment,
    link::Link, literal::Literal, r#for::For, r#if::IfExpression, LINK_TAG,
};

pub enum Tree {
    If(IfExpression),
    For(For<syn::ExprBlock>),
    Fragment(Fragment),
    Component(Component),
    Element(Element),
    Literal(Literal),
    ExpressionBlock(ExpressionBlock),
    Link(Link),
}

impl Parse for Tree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_tree = if input.peek(syn::token::If) {
            Self::If(input.parse()?)
        } else if input.peek(syn::token::For) {
            Self::For(input.parse()?)
        } else if input.peek(syn::token::Brace) {
            Self::ExpressionBlock(input.parse()?)
        } else if input.peek(syn::token::Lt) {
            let forked_input = input.fork();
            forked_input.parse::<syn::token::Lt>().unwrap();
            forked_input.parse::<syn::token::Slash>().ok(); // parsing optional slash character for an unmatched closing tag
            Self::parse_after_lt(input, &forked_input)?
        } else if input.peek(syn::Lit) {
            Self::Literal(input.parse()?)
        } else {
            return Err(input.error("Invalid syntax encountered"));
        };

        Ok(html_tree)
    }
}

impl Tree {
    fn parse_after_lt(input: ParseStream, forked_input: ParseStream) -> syn::Result<Self> {
        let html_tree = if forked_input.peek(syn::token::Gt) {
            Self::Fragment(input.parse()?)
        } else if forked_input.peek(syn::token::PathSep) {
            Self::Component(input.parse()?)
        } else if forked_input.peek(syn::Ident) {
            Self::parse_where_after_lt_is_ident(input, forked_input)?
        } else {
            Self::Element(input.parse()?)
        };

        Ok(html_tree)
    }

    fn parse_where_after_lt_is_ident(
        input: ParseStream,
        forked_input: ParseStream,
    ) -> syn::Result<Self> {
        let ident = forked_input.parse::<proc_macro2::Ident>()?.to_string();

        let html_tree = if forked_input.peek(syn::token::Eq) {
            Self::Fragment(input.parse()?)
        } else if ident
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_uppercase())
        {
            if ident == LINK_TAG {
                Self::Link(input.parse()?)
            } else {
                Self::Component(input.parse()?)
            }
        } else if forked_input.peek(syn::token::PathSep) {
            Self::Component(input.parse()?)
        } else {
            Self::Element(input.parse()?)
        };

        Ok(html_tree)
    }
}

impl ToTokens for Tree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::If(html_if) => html_if.to_tokens(tokens),
            Self::For(html_for) => html_for.to_tokens(tokens),
            Self::Fragment(html_fragment) => html_fragment.to_tokens(tokens),
            Self::Component(html_component) => html_component.to_tokens(tokens),
            Self::Element(html_element) => html_element.to_tokens(tokens),
            Self::Literal(html_literal) => html_literal.to_tokens(tokens),
            Self::ExpressionBlock(html_expr_block) => html_expr_block.to_tokens(tokens),
            Self::Link(html_link) => html_link.to_tokens(tokens),
        }
    }
}
