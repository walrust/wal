use super::html_tree::HtmlTree;
use html_end_tag::HtmlEndTag;
use html_start_tag::HtmlStartTag;
use proc_macro2::Ident;
use syn::parse::Parse;

mod html_end_tag;
mod html_start_tag;

pub struct HtmlElement {
    name: Ident,
    //props: ElementProps,
    children: Vec<HtmlTree>,
}

impl Parse for HtmlElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            return match input.parse::<HtmlEndTag>() {
                Ok(html_end_tag) => Err(syn::Error::new_spanned(
                    html_end_tag.to_spanned(),
                    "This closing tag does not have corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let html_start_tag = input.parse::<HtmlStartTag>()?;
        if html_start_tag.is_self_closing() {
            return Ok(HtmlElement {
                name: html_start_tag.name,
                children: Vec::new(),
            });
        }

        if html_start_tag.is_void() {
            return Err(syn::Error::new_spanned(
                html_start_tag.to_spanned(),
                format!(
                    "The '<{}>' tag is a void element. Void elements should be self closing (they can not have children). (hint: try '<{0}/>')",
                    html_start_tag.name
                )
            ));
        }

        let mut children = Vec::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    html_start_tag.to_spanned(),
                    format!(
                        "This start tag does not have a coressponding end tag. (hint: try adding '</{}>')",
                        html_start_tag.name
                    )
                ));
            }

            if HtmlEndTag::is_end_tag_for(&html_start_tag.name, input) {
                break;
            }

            children.push(input.parse()?);
        }

        input.parse::<HtmlEndTag>()?;

        Ok(HtmlElement {
            name: html_start_tag.name,
            children,
        })
    }
}
