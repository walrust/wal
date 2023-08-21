use super::{html_for::HtmlFor, html_forest::HtmlForest};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub enum HtmlRoot {
    Empty,
    Expression(syn::Expr),
    For(HtmlFor<syn::Expr>),
    Forest(HtmlForest),
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
            Ok(Self::Forest(input.parse()?))
        }
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Empty => {}
            Self::Expression(expr) => {
                expr.to_tokens(tokens);
            }
            Self::For(html_for) => {
                html_for.to_tokens(tokens);
            }
            Self::Forest(html_forest) => {
                html_forest.to_tokens(tokens);
            }
        }
    }
}