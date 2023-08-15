use html_end_tag::HtmlEndTag;
use html_start_tag::HtmlStartTag;
use proc_macro2::Ident;
use syn::parse::Parse;

mod html_end_tag;
mod html_start_tag;

pub struct HtmlElement {
    name: Ident,
    //props: ElementProps,
    //children: Vec<HtmlTree>
}

impl Parse for HtmlElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            return match input.parse::<HtmlEndTag>() {
                Ok(html_end_tag) => Err(syn::Error::new(
                    html_end_tag.span(),
                    "This closing tag does not have corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let html_start_tag = input.parse::<HtmlStartTag>()?;
        if html_start_tag.is_self_closing() {
            return Ok(HtmlElement {
                name: html_start_tag.name,
            });
        }

        if html_start_tag.is_void() {
            return Err(syn::Error::new(
                html_start_tag.span(),
                format!(
                    "The tag {} is a void element. It should be self closing (it can not have children). (hint: try <{0}/>)",
                    html_start_tag.name
                )
            ));
        }

        // here parse children

        let html_end_tag = input.parse::<HtmlEndTag>()?;

        // after parsing children this code might be unnecessary
        if html_start_tag.name != html_end_tag.name {
            return Err(syn::Error::new(
                html_start_tag
                    .span()
                    .join(html_end_tag.span())
                    .unwrap_or(html_start_tag.span()),
                format!(
                    "Start tag {} and end tag {} do not match. (hint: try Expected </{1}>",
                    html_start_tag.name, html_end_tag.name,
                ),
            ));
        }

        Ok(HtmlElement {
            name: html_start_tag.name,
        })
    }
}
