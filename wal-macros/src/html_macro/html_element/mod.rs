use html_element_end_tag::HtmlElementEndTag;
use html_element_start_tag::HtmlElementStartTag;
use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};

use self::html_element_attributes::HtmlElementAttributes;
use super::html_tree::HtmlTree;

mod html_element_attributes;
mod html_element_end_tag;
mod html_element_start_tag;

pub struct HtmlElement {
    name: Ident,
    attributes: HtmlElementAttributes,
    children: Vec<HtmlTree>,
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let html_end_tag = input.parse::<HtmlElementEndTag>()?;
            return Err(syn::Error::new_spanned(
                html_end_tag.to_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    html_end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<HtmlElementStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(HtmlElement {
                name: start_tag.name,
                attributes: start_tag.attributes,
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

        let children = Self::parse_children(&start_tag, input)?;
        input.parse::<HtmlElementEndTag>()?;

        Ok(HtmlElement {
            name: start_tag.name,
            attributes: start_tag.attributes,
            children,
        })
    }
}

impl HtmlElement {
    fn parse_children(
        start_tag: &HtmlElementStartTag,
        input: ParseStream,
    ) -> syn::Result<Vec<HtmlTree>> {
        let mut children = Vec::new();

        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                start_tag.to_spanned(),
                format!(
                    "This start tag does not have a corresponding end tag. (hint: try adding `</{}>`)",
                    start_tag.name
                ),
            ));
            }

            if HtmlElementEndTag::peek(&start_tag.name, input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let attributes = self.attributes.get_attributes_token_stream();
        let event_handlers = self.attributes.get_event_handlers_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.name.span() =>
            ::wal::virtual_dom::VNode::Element(
                ::wal::virtual_dom::VElement::new(
                    ::std::string::String::from(#name),
                    ::std::collections::HashMap::from([
                        #(#attributes,)*
                    ]),
                    ::std::vec![#(#event_handlers,)*],
                    ::std::vec![#(#children,)*],
                ),
            )
        });
    }
}
