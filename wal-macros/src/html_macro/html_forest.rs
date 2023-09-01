use super::html_tree::HtmlTree;
use quote::{quote, ToTokens};
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
        match self.0.as_slice() {
            [single_tree] => single_tree.to_tokens(tokens),
            trees => tokens.extend(quote! {
                ::wal_vdom::virtual_dom::VNode::List {
                    vlist: ::wal_vdom::virtual_dom::VList::new(vec![#(#trees),*])
                }
            }),
        }
    }
}
