use fragment_end_tag::FragmentEndTag;
use fragment_start_tag::FragmentStartTag;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use super::tree::Tree;

mod fragment_end_tag;
mod fragment_start_tag;

pub struct Fragment {
    start_tag: FragmentStartTag,
    children: Vec<Tree>,
    end_tag: FragmentEndTag,
}

impl Parse for Fragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let end_tag = input.parse::<FragmentEndTag>()?;
            return Err(syn::Error::new_spanned(
                end_tag.error_spanned(),
                "This closing fragment does not have a corresponding opening fragment. (hint: try adding `<>`)",
            ));
        }

        let start_tag = input.parse::<FragmentStartTag>()?;

        let mut children = Vec::new();
        while !FragmentEndTag::peek(input) {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    start_tag.error_spanned(),
                    "This opening fragment does not have a coressponding closing fragment. (hint: try adding `</>`)",
                ));
            }

            children.push(input.parse()?);
        }

        let end_tag = input.parse::<FragmentEndTag>()?;

        Ok(Fragment {
            start_tag,
            children,
            end_tag,
        })
    }
}

impl ToTokens for Fragment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        let key = self.start_tag.get_key_token_stream();

        tokens.extend(quote_spanned! {self.error_span() =>
            ::wal::virtual_dom::VNode::List(
                ::wal::virtual_dom::VList::new(
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
        let start_spanned = self.start_tag.error_spanned();
        let end_spanned = self.end_tag.error_spanned();
        quote!(#start_spanned #end_spanned)
    }
}
