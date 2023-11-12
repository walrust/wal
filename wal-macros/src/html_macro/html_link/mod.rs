use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};

use self::{html_link_end_tag::HtmlLinkEndTag, html_link_start_tag::HtmlLinkStartTag};

use super::{html_attribute::HtmlAttribute, html_tree::HtmlTree};

mod html_link_end_tag;
mod html_link_start_tag;

pub struct HtmlLink {
    name: Ident,
    to: HtmlAttribute,
    key: Option<HtmlAttribute>,
    children: Vec<HtmlTree>,
}

impl Parse for HtmlLink {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let html_link_end_tag = input.parse::<HtmlLinkEndTag>()?;
            return Err(syn::Error::new_spanned(
                html_link_end_tag.to_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    html_link_end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<HtmlLinkStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(HtmlLink {
                name: start_tag.name,
                to: start_tag.to,
                key: start_tag.key,
                children: Vec::new(),
            });
        }

        let children = Self::parse_children(&start_tag, input)?;
        input.parse::<HtmlLinkEndTag>()?;

        Ok(HtmlLink {
            name: start_tag.name,
            to: start_tag.to,
            key: start_tag.key,
            children,
        })
    }
}

impl HtmlLink {
    fn parse_children(
        start_tag: &HtmlLinkStartTag,
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

            if HtmlLinkEndTag::peek(input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for HtmlLink {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attributes = self.get_attributes_token_stream();
        let key = self.get_key_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.name.span() =>
            ::wal::virtual_dom::VNode::Element(
                ::wal::virtual_dom::VElement::new(
                    ::std::string::String::from("a"),
                    ::std::collections::HashMap::from([
                        #(#attributes,)*
                    ]),
                    ::std::vec::Vec::new(),
                    #key,
                    ::std::vec![#(#children,)*],
                ),
            )
        });
    }
}

impl HtmlLink {
    fn get_attributes_token_stream(&self) -> Vec<TokenStream> {
        let mut attributes = Vec::new();

        let to_ident = &self.to.ident;
        let to_val = &self.to.value;
        attributes
            .push(quote_spanned!(to_ident.span() => (::std::string::String::from("href"), #to_val.to_string())));
        attributes
            .push(quote_spanned!(to_ident.span() => (::std::string::String::from("data_link"), #to_val.to_string())));

        attributes
    }

    fn get_key_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_val = &key.value;
            quote_spanned!(key.ident.span() => Some(#key_val.to_string()))
        } else {
            quote!(None)
        }
    }
}
