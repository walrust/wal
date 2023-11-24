use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use super::tree::Tree;

pub struct Forest(Vec<Tree>);

impl Parse for Forest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut html_forest = Vec::new();

        while !input.is_empty() {
            html_forest.push(input.parse()?);
        }

        Ok(Forest(html_forest))
    }
}

impl ToTokens for Forest {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0.as_slice() {
            [single_tree] => single_tree.to_tokens(tokens),
            trees => tokens.extend(quote! {
                ::wal::virtual_dom::VNode::List(
                    ::wal::virtual_dom::VList::new(vec![#(#trees),*], None)
                )
            }),
        }
    }
}
