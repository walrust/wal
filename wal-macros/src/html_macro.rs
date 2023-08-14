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

impl Parse for HtmlTree {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let html_type = HtmlTree::get_html_type(input);
        let html_tree = match html_type {
            HtmlType::Empty => Self::Empty,
            HtmlType::SingleValue => Self::SingleValue(input.parse()?),
            _ => unimplemented!(),
        };

        if input.is_empty() {
            Ok(html_tree)
        } else {
            let remaining_stream: proc_macro2::TokenStream = input.parse()?;
            Err(syn::Error::new_spanned(
                remaining_stream,
                "Unexpected tokens. Only single html root element is allowed (hint: wrap your html in '<></>')",
            ))
        }
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
            HtmlType::SingleValue
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
            HtmlType::SingleValue
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

pub enum SingleHtmlValue {
    Lit(syn::Lit),
    Expr(syn::Expr),
}

impl Parse for SingleHtmlValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            let lit = input.parse()?;

            let error_message = match lit {
                syn::Lit::ByteStr(_) => Some("Byte string literals are not supported"),
                syn::Lit::Byte(_) => Some("Byte literals are not supported"),
                syn::Lit::Verbatim(_) => Some("Unsupported literal format encountered"),
                _ => None,
            };

            if let Some(msg) = error_message {
                return Err(syn::Error::new_spanned(lit, msg));
            };

            Ok(Self::Lit(lit))
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}
