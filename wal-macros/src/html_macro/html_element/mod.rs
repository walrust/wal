use std::collections::HashMap;

use self::html_element_attributes::HtmlElementAttributes;
use super::{html_literal::HtmlLiteral, html_tree::HtmlTree};
use html_element_end_tag::HtmlElementEndTag;
use html_element_start_tag::HtmlElementStartTag;
use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::parse::Parse;

mod html_element_attributes;
mod html_element_end_tag;
mod html_element_start_tag;

pub struct HtmlElement {
    name: Ident,
    attributes: HtmlElementAttributes,
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
            attributes: start_tag.attributes,
            children,
        })
    }
}

impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let attributes: HashMap<String, String> = (&self.attributes).into();
        let attributes: Vec<proc_macro2::TokenStream> = attributes
            .iter()
            .map(|(k, v)| quote_spanned!(self.name.span() => (#k, #v)))
            .collect();
        let children = &self.children;

        tokens.extend(
            quote_spanned!(self.name.span() => ::wal_vdom::virtual_dom::VNode::Element {
                velement: ::wal_vdom::virtual_dom::VElement::new_attrs_as_vecs(
                    #name,
                    ::std::collections::HashMap::from([
                        #(#attributes,)*
                    ]),
                    ::std::vec![#(#children,)*],
                )
            }),
        );
    }
}
