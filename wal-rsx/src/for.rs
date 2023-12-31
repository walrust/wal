use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

// This struct is generic because we want to be able to have:
// - For<syn::Expr> as a single element in rsx macro
// - For<syn::ExprBlock> as a part of more complex structure in rsx macro
pub(crate) struct For<Expr: ExprInFor>(Expr);

pub(crate) trait ExprInFor: Spanned {}

impl ExprInFor for syn::Expr {}

impl ExprInFor for syn::ExprBlock {}

impl<Expr: Parse + ExprInFor> Parse for For<Expr> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::token::For>()?;
        Ok(For(input.parse()?))
    }
}

impl For<syn::Expr> {
    pub(crate) fn peek(input: ParseStream) -> bool {
        let forked_input = input.fork();

        forked_input.parse::<syn::token::For>().is_ok()
            && forked_input.parse::<syn::Expr>().is_ok()
            && forked_input.is_empty()
    }
}

impl ToTokens for For<syn::Expr> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.0;
        tokens.extend(quote_spanned!(expr.span() =>
            ::wal_core::virtual_dom::VNode::from_iter(#expr)
        ));
    }
}

impl ToTokens for For<syn::ExprBlock> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.0;
        tokens.extend(quote_spanned!(expr.span() =>
            #[allow(unused_braces)]
            ::wal_core::virtual_dom::VNode::from_iter(#expr)
        ));
    }
}
