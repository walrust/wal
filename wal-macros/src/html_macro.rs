use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

pub enum HtmlTree {
    Empty,
    Block,
    If,
    For,
    List,
    Component,
    Element,
    Value,
}

enum HtmlType {
    Empty,
    Block,
    If,
    For,
    List,
    Component,
    Element,
    Value,
}

impl Parse for HtmlTree {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html_type = HtmlTree::get_html_type(input);
    }
}

impl HtmlTree {
    fn get_html_type(input: ParseStream) -> HtmlType {
        let input = input.fork();

        if input.is_empty() {
            HtmlType::Empty
        } else if input.peek(syn::token::Brace) {
            HtmlType::Block
        } else if input.peek(syn::token::If) {
            HtmlType::If
        } else if input.peek(syn::token::For) {
            HtmlType::For
        } else if input.peek(syn::token::Lt) {
            input.parse::<syn::token::Lt>().unwrap();
            Self::get_html_type_after_lt(&input)
        } else {
            HtmlType::Value
        }
    }

    fn get_html_type_after_lt(input: ParseStream) -> HtmlType {
        if input.peek(syn::token::Gt) {
            HtmlType::List
        } else if input.peek(syn::token::PathSep) {
            HtmlType::Component
        } else if input.peek(proc_macro2::Ident::peek_any) {
            Self::get_html_type_where_after_lt_is_ident(input)
        } else {
            HtmlType::Value
        }
    }

    fn get_html_type_where_after_lt_is_ident(input: ParseStream) -> HtmlType {
        let ident = proc_macro2::Ident::parse_any(&input).unwrap();
        let ident = ident.to_string();

        if input.peek(syn::token::Eq) {
            HtmlType::List
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
