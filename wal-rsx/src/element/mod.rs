use element_closing_tag::ElementClosingTag;
use element_opening_tag::ElementOpeningTag;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::tree::Tree;

mod element_attribute;
mod element_attributes;
mod element_closing_tag;
mod element_opening_tag;

pub(crate) struct Element {
    opening_tag: ElementOpeningTag,
    children: Vec<Tree>,
    closing_tag: Option<ElementClosingTag>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let closing_tag = input.parse::<ElementClosingTag>()?;
            return Err(syn::Error::new_spanned(
                closing_tag.error_spanned(),
                format!(
                    "This closing tag does not have a corresponding opening tag. (hint: try adding `<{}>`)",
                    closing_tag.name
                )
            ));
        }

        let opening_tag = input.parse::<ElementOpeningTag>()?;
        if opening_tag.is_self_closing() {
            return Ok(Element {
                opening_tag,
                children: Vec::new(),
                closing_tag: None,
            });
        }

        if opening_tag.is_void() {
            return Err(syn::Error::new_spanned(
                opening_tag.error_spanned(),
                format!(
                    "The `<{}>` tag is a void element. Void elements should be self closing (they can not have children). (hint: try `<{0}/>`)",
                    opening_tag.name
                )
            ));
        }

        let children = Self::parse_children(input, &opening_tag)?;
        let closing_tag = input.parse::<ElementClosingTag>()?;

        Ok(Element {
            opening_tag,
            children,
            closing_tag: Some(closing_tag),
        })
    }
}

impl Element {
    fn parse_children(
        input: ParseStream,
        opening_tag: &ElementOpeningTag,
    ) -> syn::Result<Vec<Tree>> {
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

            if ElementClosingTag::peek(&opening_tag.name, input) {
                break;
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.opening_tag.name.to_string();
        let attributes = self.opening_tag.attributes.get_attributes_token_stream();
        let key = self.opening_tag.attributes.get_key_attribute_token_stream();
        let event_handlers = self
            .opening_tag
            .attributes
            .get_event_handlers_token_stream();
        let children = &self.children;

        tokens.extend(quote_spanned! { self.error_span() =>
            ::wal_core::virtual_dom::VNode::Element(
                ::wal_core::virtual_dom::VElement::new(
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
        let start_error_spanned = self.opening_tag.error_spanned();
        let end_error_spanned = self.closing_tag.as_ref().map(ElementClosingTag::error_spanned);
        if end_error_spanned.is_some() {
            quote!(#start_error_spanned #end_error_spanned)
        } else {
            quote!(#start_error_spanned)
        }
    }
}
