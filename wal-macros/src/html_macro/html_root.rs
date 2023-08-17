use super::html_forest::HtmlForest;
use syn::parse::{Parse, ParseStream};

pub enum HtmlRoot {
    Empty,
    Expression(syn::Expr),
    HtmlForest(HtmlForest),
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Empty);
        }

        let forked_input = input.fork();
        let expr = forked_input.parse::<syn::Expr>();
        if expr.is_ok() {
            Ok(Self::Expression(input.parse()?))
        } else {
            Ok(Self::HtmlForest(input.parse()?))
        }
    }
}
