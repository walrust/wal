use super::html_tree::HtmlTree;
use html_element_end_tag::HtmlElementEndTag;
use html_element_start_tag::HtmlElementStartTag;
use proc_macro2::Ident;
use syn::parse::Parse;

mod html_element_end_tag;
mod html_element_start_tag;

pub struct HtmlElement {
    name: Ident,
    //props: ElementProps,
    children: Vec<HtmlTree>,
}

impl Parse for HtmlElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            return match input.parse::<HtmlElementEndTag>() {
                Ok(html_end_tag) => Err(syn::Error::new_spanned(
                    html_end_tag.to_spanned(),
                    format!(
                        "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                        html_end_tag.name
                    )
                )),
                Err(err) => Err(err),
            };
        }

        let start_tag = input.parse::<HtmlElementStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(HtmlElement {
                name: start_tag.name,
                children: Vec::new(),
            });
        }

        if start_tag.is_void() {
            return Err(syn::Error::new_spanned(
                start_tag.to_spanned(),
                format!(
                    "The `<{}>` tag is a void element. Void elements should be self closing (they can not have children). (hint: try `<{0}/>`)",
                    start_tag.name
                )
            ));
        }

        let mut children = Vec::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    start_tag.to_spanned(),
                    format!(
                        "This start tag does not have a coressponding end tag. (hint: try adding `</{}>`)",
                        start_tag.name
                    )
                ));
            }

            if HtmlElementEndTag::peek(&start_tag.name, input) {
                break;
            }

            children.push(input.parse()?);
        }

        input.parse::<HtmlElementEndTag>()?;

        Ok(HtmlElement {
            name: start_tag.name,
            children,
        })
    }
}
