use super::html_tree::HtmlTree;
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
