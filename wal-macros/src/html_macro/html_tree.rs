use super::{
    html_element::HtmlElement, html_for::HtmlFor, html_fragment::HtmlFragment,
    html_if::HtmlIfExpression, html_literal::HtmlLiteral,
};
use quote::{quote_spanned, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub enum HtmlTree {
    If(HtmlIfExpression),
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
            HtmlType::Component => unimplemented!(), // TODO: Component parsing needed
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
            Self::_Component => unimplemented!(),     // TODO: Component parsing needed
            Self::Element(html_element) => html_element.to_tokens(tokens),
            Self::Literal(html_literal) => html_literal.to_tokens(tokens),
            Self::ExpressionBlock(expr_block) => tokens.extend(
                quote_spanned!(expr_block.span() => ::wal_vdom::virtual_dom::VNode::from(#expr_block)),
            ),
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
