use super::html_tree::HtmlTree;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub struct HtmlForest(Vec<HtmlTree>);

impl Parse for HtmlForest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut html_forest = Vec::new();

        while !input.is_empty() {
            html_forest.push(input.parse()?);
        }

        Ok(HtmlForest(html_forest))
    }
}

impl ToTokens for HtmlForest {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let html_trees_tokens: Vec<proc_macro2::TokenStream> = self
            .0
            .iter()
            .map(|html_tree| html_tree.to_token_stream())
            .collect();

        if html_trees_tokens.len() == 1 {
            tokens.extend(html_trees_tokens[0].clone());
        } else {
            unimplemented!(); // TODO: here we should wrap the html_trees_tokens in a VList
        }

        // TODO: we could rewrite it using quote only
    }
}
