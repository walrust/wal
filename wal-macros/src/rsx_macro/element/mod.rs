use element_end_tag::ElementEndTag;
use element_start_tag::ElementStartTag;
use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};

use self::element_attributes::ElementAttributes;
use super::tree::Tree;

mod element_attribute;
mod element_attributes;
mod element_end_tag;
mod element_start_tag;

pub struct Element {
    name: Ident,
    attributes: ElementAttributes,
    children: Vec<Tree>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let html_end_tag = input.parse::<ElementEndTag>()?;
            return Err(syn::Error::new_spanned(
                html_end_tag.to_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    html_end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<ElementStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(Element {
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
        input.parse::<ElementEndTag>()?;

        Ok(Element {
            name: start_tag.name,
            attributes: start_tag.attributes,
            children,
        })
    }
}

impl Element {
    fn parse_children(start_tag: &ElementStartTag, input: ParseStream) -> syn::Result<Vec<Tree>> {
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

            if ElementEndTag::peek(&start_tag.name, input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let attributes = self.attributes.get_attributes_token_stream();
        let key = self.attributes.get_key_token_stream();
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
                    #key,
                    ::std::vec![#(#children,)*],
                ),
            )
        });
    }
}
