use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use self::{link_end_tag::LinkEndTag, link_start_tag::LinkStartTag};

use super::tree::Tree;

mod link_end_tag;
mod link_start_tag;

pub(crate) const LINK_TAG: &str = "Link";
const TO_ATTR: &str = "to";

pub(crate) struct Link {
    start_tag: LinkStartTag,
    children: Vec<Tree>,
    end_tag: Option<LinkEndTag>,
}

impl Parse for Link {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let end_tag = input.parse::<LinkEndTag>()?;
            return Err(syn::Error::new_spanned(
                end_tag.error_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<LinkStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(Link {
                start_tag,
                children: Vec::new(),
                end_tag: None,
            });
        }

        let children = Self::parse_children(&start_tag, input)?;
        let end_tag = input.parse()?;

        Ok(Link {
            start_tag,
            children,
            end_tag: Some(end_tag),
        })
    }
}

impl Link {
    fn parse_children(start_tag: &LinkStartTag, input: ParseStream) -> syn::Result<Vec<Tree>> {
        let mut children = Vec::new();

        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    start_tag.error_spanned(),
                    format!(
                        "This start tag does not have a corresponding end tag. (hint: try adding `</{}>`)",
                        start_tag.name
                    ),
                ));
            }

            if LinkEndTag::peek(input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for Link {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attributes = self.start_tag.get_to_attribute_token_stream();
        let key = self.start_tag.get_key_attribute_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.error_span() =>
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

impl Link {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let start_error_spanned = self.start_tag.error_spanned();
        let end_error_spanned = self.end_tag.as_ref().map(LinkEndTag::error_spanned);
        if end_error_spanned.is_some() {
            quote!(#start_error_spanned #end_error_spanned)
        } else {
            quote!(#start_error_spanned)
        }
    }
}
