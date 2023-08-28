use super::{
    html_element::HtmlElement, html_for::HtmlFor, html_fragment::HtmlFragment, html_if::HtmlIf,
    html_literal::HtmlLiteral,
};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

pub enum HtmlTree {
    If(HtmlIf),
    For(HtmlFor<syn::ExprBlock>),
    Fragment(HtmlFragment),
    _Component,
    Element(HtmlElement),
    Literal(HtmlLiteral),
    ExpressionBlock(syn::ExprBlock),
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_type = HtmlType::get(input);

        let html_tree = match html_type {
            HtmlType::If => Self::If(input.parse()?),
            HtmlType::For => Self::For(input.parse()?),
            HtmlType::Fragment => Self::Fragment(input.parse()?),
            HtmlType::Element => Self::Element(input.parse()?),
            HtmlType::Literal => Self::Literal(input.parse()?),
            HtmlType::ExpressionBlock => Self::ExpressionBlock(input.parse()?),
            _ => unimplemented!(),
        };

        Ok(html_tree)
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::If(html_if) => unimplemented!(),
            Self::For(html_for) => unimplemented!(),
            Self::Fragment(html_fragment) => unimplemented!(),
            Self::_Component => unimplemented!(),
            Self::Element(html_element) => unimplemented!(),
            Self::Literal(html_literal) => unimplemented!(),
            Self::ExpressionBlock(expr_block) => unimplemented!(),
        }
    }
}

pub enum HtmlType {
    If,
    For,
    Fragment,
    Component,
    Element,
    Literal,
    ExpressionBlock,
}

impl HtmlType {
    fn get(input: ParseStream) -> Self {
        let input = input.fork();

        if input.peek(syn::token::If) {
            Self::If
        } else if input.peek(syn::token::For) {
            Self::For
        } else if input.peek(syn::token::Brace) {
            Self::ExpressionBlock
        } else if input.peek(syn::token::Lt) {
            input.parse::<syn::token::Lt>().unwrap();
            input.parse::<syn::token::Slash>().ok(); // parsing optional slash character for unmatched closing tags
            Self::get_after_lt(&input)
        } else {
            Self::Literal
        }
    }

    fn get_after_lt(input: ParseStream) -> HtmlType {
        if input.peek(syn::token::Gt) {
            HtmlType::Fragment
        } else if input.peek(syn::token::PathSep) {
            HtmlType::Component
        } else if input.peek(proc_macro2::Ident::peek_any) {
            Self::get_where_after_lt_is_ident(input)
        } else {
            HtmlType::Element
        }
    }

    fn get_where_after_lt_is_ident(input: ParseStream) -> HtmlType {
        let ident = proc_macro2::Ident::parse_any(&input).unwrap();
        let ident = ident.to_string();

        if input.peek(syn::token::Eq) {
            HtmlType::Fragment
        } else if ident
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_uppercase())
            || input.peek(syn::token::PathSep)
        {
            HtmlType::Component
        } else {
            HtmlType::Element
        }
    }
}
