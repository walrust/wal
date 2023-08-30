use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

// This struct is generic because we want to be able to have:
// - HtmlFor<syn::Expr> as a single element in html macro
// - HtmlFor<syn::ExprBlock> as a part of more complex structure in html macro
pub struct HtmlFor<Expr: ExprInFor>(Expr);

pub trait ExprInFor {}

impl ExprInFor for syn::Expr {}

impl ExprInFor for syn::ExprBlock {}

impl<Expr: Parse + ExprInFor> Parse for HtmlFor<Expr> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::token::For>()?;

        Ok(HtmlFor(input.parse()?))
    }
}

impl HtmlFor<syn::Expr> {
    pub fn peek(input: ParseStream) -> bool {
        let input = input.fork();
        if input.parse::<syn::token::For>().is_err() {
            return false;
        };
        if input.parse::<syn::Expr>().is_err() {
            return false;
        };

        input.is_empty()
    }
}

impl<Expr: ExprInFor> ToTokens for HtmlFor<Expr> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let expr = &self.0;
        // tokens.extend(quote!(::wal_vdom::virtual_dom::VNode::For {
        //     vfor: ::wal_vdom::virtual_dom::VFor::new(#expr)
        // }));
    }
}
