use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::{forest::Forest, r#for::For};

pub enum Root {
    Empty,
    Expression(syn::Expr),
    For(For<syn::Expr>),
    Forest(Forest),
}

impl Parse for Root {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Empty);
        }

        if For::<syn::Expr>::peek(input) {
            return Ok(Self::For(input.parse()?));
        }

        let forked_input = input.fork();
        if let Ok(expr) = forked_input.parse::<syn::Expr>() {
            if forked_input.is_empty() && !matches!(expr, syn::Expr::Lit(_) | syn::Expr::If(_)) {
                return Ok(Self::Expression(input.parse()?));
            }
        }

        Ok(Self::Forest(input.parse()?))
    }
}

impl ToTokens for Root {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Empty => tokens.extend(quote! {
                ::wal::virtual_dom::VNode::List (
                    ::wal::virtual_dom::VList::new_empty(None)
                )
            }),
            Self::Expression(expr) => tokens
                .extend(quote_spanned! { expr.span() => ::wal::virtual_dom::VNode::from(#expr) }),
            Self::For(rsx_for) => rsx_for.to_tokens(tokens),
            Self::Forest(forest) => forest.to_tokens(tokens),
        };
    }
}
