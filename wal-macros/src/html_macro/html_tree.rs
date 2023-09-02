use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use super::{
    html_element::HtmlElement, html_expression_block::HtmlExpressionBlock, html_for::HtmlFor,
    html_fragment::HtmlFragment, html_if::HtmlIfExpression, html_literal::HtmlLiteral,
};

pub enum HtmlTree {
    If(HtmlIfExpression),
    For(HtmlFor<syn::ExprBlock>),
    Fragment(HtmlFragment),
    _Component, // TODO: Implement component
    Element(HtmlElement),
    Literal(HtmlLiteral),
    ExpressionBlock(HtmlExpressionBlock),
}

impl Parse for HtmlTree {
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
        } else {
            Self::Literal(input.parse()?)
        };

        Ok(html_tree)
    }
}

impl HtmlTree {
    fn parse_after_lt(input: ParseStream, forked_input: ParseStream) -> syn::Result<Self> {
        let html_tree = if forked_input.peek(syn::token::Gt) {
            Self::Fragment(input.parse()?)
        } else if forked_input.peek(syn::token::PathSep) {
            Self::_Component // TODO: Implement parsing for component
        } else if forked_input.peek(proc_macro2::Ident::peek_any) {
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
        let ident = proc_macro2::Ident::parse_any(forked_input)?.to_string();

        let html_tree = if forked_input.peek(syn::token::Eq) {
            Self::Fragment(input.parse()?)
        } else if ident
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_uppercase())
            || forked_input.peek(syn::token::PathSep)
        {
            Self::_Component // TODO: Implement parsing for component
        } else {
            Self::Element(input.parse()?)
        };

        Ok(html_tree)
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::If(html_if) => html_if.to_tokens(tokens),
            Self::For(html_for) => html_for.to_tokens(tokens),
            Self::Fragment(html_fragment) => html_fragment.to_tokens(tokens),
            Self::_Component => unimplemented!(), // TODO: implement totokens for component
            Self::Element(html_element) => html_element.to_tokens(tokens),
            Self::Literal(html_literal) => html_literal.to_tokens(tokens),
            Self::ExpressionBlock(html_expr_block) => html_expr_block.to_tokens(tokens),
        }
    }
}
