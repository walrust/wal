use super::html_tree::HtmlTree;
use syn::parse::{Parse, ParseStream};

pub enum HtmlRoot {
    Empty,
    Expression(syn::Expr),
    HtmlTree(HtmlTree),
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Empty);
        }

        let forked_input = input.fork();
        let expr = forked_input.parse::<syn::Expr>();
        let html_root = if expr.is_ok() {
            Self::Expression(input.parse()?)
        } else {
            Self::HtmlTree(input.parse()?)
        };

        if input.is_empty() {
            Ok(html_root)
        } else {
            let remaining_stream: proc_macro2::TokenStream = input.parse()?;
            Err(syn::Error::new_spanned(
                remaining_stream,
                "Unexpected tokens. Only single html root element is allowed (hint: wrap your html in '<></>')",
            ))
        }
    }
}
