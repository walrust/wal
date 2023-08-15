use html_element::HtmlElement;
use single_html_value::SingleHtmlValue;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

mod html_element;
mod single_html_value;

pub struct HtmlRoot(HtmlTree);

enum HtmlTree {
    Empty,
    Block,
    If,
    For,
    List,
    Component,
    Element(HtmlElement),
    SingleValue(SingleHtmlValue),
}

enum HtmlType {
    Empty,
    Block,
    If,
    For,
    List,
    Component,
    Element,
    SingleValue,
}

impl Parse for HtmlRoot {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html_tree = input.parse()?;

        if input.is_empty() {
            Ok(Self(html_tree))
        } else {
            let remaining_stream: proc_macro2::TokenStream = input.parse()?;
            Err(syn::Error::new_spanned(
                remaining_stream,
                "Unexpected tokens. Only single html root element is allowed (hint: wrap your html in '<></>')",
            ))
        }
    }
}

impl Parse for HtmlTree {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html_type = HtmlType::get(input);
        let html_tree = match html_type {
            HtmlType::Empty => Self::Empty,
            HtmlType::SingleValue => Self::SingleValue(input.parse()?),
            HtmlType::Element => Self::Element(input.parse()?),
            _ => unimplemented!(),
        };

        Ok(html_tree)
    }
}

impl HtmlType {
    fn get(input: ParseStream) -> HtmlType {
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
            Self::get_after_lt(&input)
        } else {
            HtmlType::SingleValue
        }
    }

    fn get_after_lt(input: ParseStream) -> HtmlType {
        input.parse::<syn::token::Slash>().ok(); // parsing optional slash character for unmatched closing tags

        if input.peek(syn::token::Gt) {
            HtmlType::List
        } else if input.peek(syn::token::PathSep) {
            HtmlType::Component
        } else if input.peek(proc_macro2::Ident::peek_any) {
            Self::get_where_after_lt_is_ident(input)
        } else {
            HtmlType::SingleValue
        }
    }

    fn get_where_after_lt_is_ident(input: ParseStream) -> HtmlType {
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
