use super::{html_for::HtmlFor, html_forest::HtmlForest};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

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
        if forked_input.parse::<syn::Expr>().is_ok() {
            return Ok(Self::Expression(input.parse()?));
        }

        Ok(Self::Forest(input.parse()?))
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Empty => tokens.extend(quote! {
                ::wal_vdom::virtual_dom::VNode::List {
                    vlist: ::wal_vdom::virtual_dom::VList::new_empty()
                }
            }),
            Self::Expression(expr) => tokens.extend(quote_spanned! {
                expr.span() => ::wal_vdom::virtual_dom::VNode::from(#expr)
            }),
            Self::For(html_for) => html_for.to_tokens(tokens),
            Self::Forest(html_forest) => html_forest.to_tokens(tokens),
        };
    }
}
