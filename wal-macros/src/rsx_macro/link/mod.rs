use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};

use self::{link_end_tag::LinkEndTag, link_start_tag::LinkStartTag};

use super::{attributes::normal_attribute::NormalAttribute, tree::Tree};

mod link_end_tag;
mod link_start_tag;

pub const LINK_TAG: &str = "Link";
const TO_ATTR: &str = "to";

pub struct Link {
    name: proc_macro2::Ident,
    to: NormalAttribute,
    key: Option<NormalAttribute>,
    children: Vec<Tree>,
}

impl Parse for Link {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let end_tag = input.parse::<LinkEndTag>()?;
            return Err(syn::Error::new_spanned(
                end_tag.to_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<LinkStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(Link {
                name: start_tag.name,
                to: start_tag.to,
                key: start_tag.key,
                children: Vec::new(),
            });
        }

        let children = Self::parse_children(&start_tag, input)?;
        input.parse::<LinkEndTag>()?;

        Ok(Link {
            name: start_tag.name,
            to: start_tag.to,
            key: start_tag.key,
            children,
        })
    }
}

impl Link {
    fn parse_children(start_tag: &LinkStartTag, input: ParseStream) -> syn::Result<Vec<Tree>> {
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
        let attributes = self.get_to_attribute_token_stream();
        let key = self.get_key_attribute_token_stream();
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

impl Link {
    fn get_to_attribute_token_stream(&self) -> Vec<proc_macro2::TokenStream> {
        let mut attributes = Vec::new();

        let to_value = &self.to.value;
        attributes
            .push(quote_spanned!(to_value.span() => (::std::string::String::from("href"), #to_value.to_string())));
        attributes
            .push(quote_spanned!(to_value.span() => (::std::string::String::from("data_link"), #to_value.to_string())));

        attributes
    }

    fn get_key_attribute_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_value = &key.value;
            quote_spanned!(key_value.span() => Some(#key_value.to_string()))
        } else {
            quote!(None)
        }
    }
}
