use element_end_tag::ElementEndTag;
use element_start_tag::ElementStartTag;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::tree::Tree;

mod element_attribute;
mod element_attributes;
mod element_end_tag;
mod element_start_tag;

pub(crate) struct Element {
    start_tag: ElementStartTag,
    children: Vec<Tree>,
    end_tag: Option<ElementEndTag>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let end_tag = input.parse::<ElementEndTag>()?;
            return Err(syn::Error::new_spanned(
                end_tag.error_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    end_tag.name
                )
            ));
        }

        let start_tag = input.parse::<ElementStartTag>()?;
        if start_tag.is_self_closing() {
            return Ok(Element {
                start_tag,
                children: Vec::new(),
                end_tag: None,
            });
        }

        if start_tag.is_void() {
            return Err(syn::Error::new_spanned(
                start_tag.error_spanned(),
                format!(
                    "The `<{}>` tag is a void element. Void elements should be self closing (they can not have children). (hint: try `<{0}/>`)",
                    start_tag.name
                )
            ));
        }

        let children = Self::parse_children(input, &start_tag)?;
        let end_tag = input.parse::<ElementEndTag>()?;

        Ok(Element {
            start_tag,
            children,
            end_tag: Some(end_tag),
        })
    }
}

impl Element {
    fn parse_children(input: ParseStream, start_tag: &ElementStartTag) -> syn::Result<Vec<Tree>> {
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
        let name = &self.start_tag.name.to_string();
        let attributes = self.start_tag.attributes.get_attributes_token_stream();
        let key = self.start_tag.attributes.get_key_attribute_token_stream();
        let event_handlers = self.start_tag.attributes.get_event_handlers_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.error_span() =>
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

impl Element {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let start_error_spanned = self.start_tag.error_spanned();
        let end_error_spanned = self.end_tag.as_ref().map(ElementEndTag::error_spanned);
        if end_error_spanned.is_some() {
            quote!(#start_error_spanned #end_error_spanned)
        } else {
            quote!(#start_error_spanned)
        }
    }
}
