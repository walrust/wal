use super::html_tree::HtmlTree;
use html_list_end_tag::HtmlListEndTag;
use html_list_start_tag::HtmlListStartTag;
use syn::parse::Parse;

mod html_list_end_tag;
mod html_list_start_tag;

pub struct HtmlList(Vec<HtmlTree>);

impl Parse for HtmlList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            return match input.parse::<HtmlListEndTag>() {
                Ok(html_end_tag) => Err(syn::Error::new_spanned(
                    html_end_tag.to_spanned(),
                    "This closing fragment does not have a corresponding opening fragment. (hint: try adding '<>')",
                )),
                Err(err) => Err(err),
            };
        }

        let start_tag = input.parse::<HtmlListStartTag>()?;

        let mut children = Vec::new();
        while !HtmlListEndTag::peek(input) {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    start_tag.to_spanned(),
                    "This opening fragment does not have a coressponding closing fragment. (hint: try adding '</>')",
                ));
            }

            children.push(input.parse()?);
        }

        input.parse::<HtmlListEndTag>()?;

        Ok(HtmlList(children))
    }
}
