use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use super::tree::Tree;

pub(crate) struct Forest(Vec<Tree>);

impl Parse for Forest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut forest = Vec::new();

        while !input.is_empty() {
            forest.push(input.parse()?);
        }

        Ok(Forest(forest))
    }
}

impl ToTokens for Forest {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0.as_slice() {
            [single_tree] => single_tree.to_tokens(tokens),
            trees => tokens.extend(quote! {
                ::wal_core::virtual_dom::VNode::List(
                    ::wal_core::virtual_dom::VList::new(vec![#(#trees),*], None)
                )
            }),
        }
    }
}
