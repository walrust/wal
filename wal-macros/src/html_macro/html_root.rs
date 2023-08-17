use super::{html_for::HtmlFor, html_forest::HtmlForest};
use syn::parse::{Parse, ParseStream};

pub enum HtmlRoot {
    Empty,
    Expression(syn::Expr),
    For(HtmlFor<syn::Expr>),
    HtmlForest(HtmlForest),
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Empty);
        }

        if HtmlFor::<syn::Expr>::peek(input) {
            return Ok(Self::For(input.parse()?));
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
