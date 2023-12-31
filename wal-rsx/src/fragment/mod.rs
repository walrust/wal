use fragment_closing_tag::FragmentClosingTag;
use fragment_opening_tag::FragmentOpeningTag;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::tree::Tree;

mod fragment_closing_tag;
mod fragment_opening_tag;

pub(crate) struct Fragment {
    opening_tag: FragmentOpeningTag,
    children: Vec<Tree>,
    closing_tag: FragmentClosingTag,
}

impl Parse for Fragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let closing_tag = input.parse::<FragmentClosingTag>()?;
            return Err(syn::Error::new_spanned(
                closing_tag.error_spanned(),
                "This closing fragment does not have a corresponding opening fragment. (hint: try adding `<>`)",
            ));
        }

        let opening_tag = input.parse::<FragmentOpeningTag>()?;
        let children = Self::parse_children(input, &opening_tag)?;
        let closing_tag = input.parse::<FragmentClosingTag>()?;

        Ok(Fragment {
            opening_tag,
            children,
            closing_tag,
        })
    }
}

impl Fragment {
    fn parse_children(
        input: ParseStream,
        opening_tag: &FragmentOpeningTag,
    ) -> syn::Result<Vec<Tree>> {
        let mut children = Vec::new();

        while !FragmentClosingTag::peek(input) {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    opening_tag.error_spanned(),
                    "This opening fragment does not have a coressponding closing fragment. (hint: try adding `</>`)",
                ));
            }

            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl ToTokens for Fragment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        let key = self.opening_tag.get_key_attribute_token_stream();

        tokens.extend(quote_spanned! {self.error_span() =>
            ::wal_core::virtual_dom::VNode::List(
                ::wal_core::virtual_dom::VList::new(
                    ::std::vec![#(#children),*],
                    #key
                )
            )
        });
    }
}

impl Fragment {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let start_error_spanned = self.opening_tag.error_spanned();
        let end_error_spanned = self.closing_tag.error_spanned();
        quote!(#start_error_spanned #end_error_spanned)
    }
}
