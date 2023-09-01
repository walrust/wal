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
        if self.0.len() == 1 {
            tokens.extend(self.0[0].to_token_stream());
        } else {
            let html_trees = &self.0;

            tokens.extend(quote! {
                ::wal_vdom::virtual_dom::VNode::List {
                    vlist: ::wal_vdom::virtual_dom::VList::new(vec![#(#html_trees),*])
                }
            });
        }
    }
}
