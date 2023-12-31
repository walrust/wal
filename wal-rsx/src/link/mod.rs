use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use self::{link_closing_tag::LinkClosingTag, link_opening_tag::LinkOpeningTag};

use super::tree::Tree;

mod link_closing_tag;
mod link_opening_tag;

pub(crate) const LINK_TAG: &str = "Link";
const TO_ATTR: &str = "to";

pub(crate) struct Link {
    opening_tag: LinkOpeningTag,
    children: Vec<Tree>,
    closing_tag: Option<LinkClosingTag>,
}

impl Parse for Link {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let closing_tag = input.parse::<LinkClosingTag>()?;
            return Err(syn::Error::new_spanned(
                closing_tag.error_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    closing_tag.name
                )
            ));
        }

        let opening_tag = input.parse::<LinkOpeningTag>()?;
        if opening_tag.is_self_closing() {
            return Ok(Link {
                opening_tag,
                children: Vec::new(),
                closing_tag: None,
            });
        }

        let children = Self::parse_children(&opening_tag, input)?;
        let closing_tag = input.parse()?;

        Ok(Link {
            opening_tag,
            children,
            closing_tag: Some(closing_tag),
        })
    }
}

impl Link {
    fn parse_children(opening_tag: &LinkOpeningTag, input: ParseStream) -> syn::Result<Vec<Tree>> {
        let mut children = Vec::new();

        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    opening_tag.error_spanned(),
                    format!(
                        "This opening tag does not have a corresponding closing tag. (hint: try adding `</{}>`)",
                        opening_tag.name
                    ),
                ));
            }

            if LinkClosingTag::peek(input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for Link {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attributes = self.opening_tag.get_to_attribute_token_stream();
        let key = self.opening_tag.get_key_attribute_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.error_span() =>
            ::wal_core::virtual_dom::VNode::Element(
                ::wal_core::virtual_dom::VElement::new(
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
        let start_error_spanned = self.opening_tag.error_spanned();
        let end_error_spanned = self.closing_tag.as_ref().map(LinkClosingTag::error_spanned);
        if end_error_spanned.is_some() {
            quote!(#start_error_spanned #end_error_spanned)
        } else {
            quote!(#start_error_spanned)
        }
    }
}
