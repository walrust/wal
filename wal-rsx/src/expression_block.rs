use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub(crate) struct ExpressionBlock(syn::ExprBlock);

impl Parse for ExpressionBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr_block: syn::ExprBlock = input.parse()?;
        if expr_block.block.stmts.is_empty() {
            return Err(syn::Error::new_spanned(
                &expr_block,
                "Expected expressions within braces, found an empty block",
            ));
        }
        Ok(ExpressionBlock(expr_block))
    }
}

impl ToTokens for ExpressionBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr_block = &self.0;
        tokens.extend(quote_spanned! { expr_block.span() =>
            #[allow(unused_braces)]
            #[allow(clippy::useless_conversion)]
            ::wal_core::virtual_dom::VNode::from(#expr_block)
        });
    }
}
