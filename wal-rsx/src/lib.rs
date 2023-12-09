use proc_macro::TokenStream;
use quote::ToTokens;
use root::Root;
use syn::parse_macro_input;

mod attributes;
mod component;
mod element;
mod expression_block;
mod r#for;
mod forest;
mod fragment;
mod r#if;
mod link;
mod literal;
mod root;
mod tree;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Root);
    TokenStream::from(root.into_token_stream())
}
