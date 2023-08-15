use proc_macro2::Ident;
use syn::parse::Parse;

use crate::html_macro::html_element::html_end_tag::HtmlEndTag;

mod html_end_tag;

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

        unimplemented!();
    }
}
