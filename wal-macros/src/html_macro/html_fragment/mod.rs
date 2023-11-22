use html_fragment_end_tag::HtmlFragmentEndTag;
use html_fragment_start_tag::HtmlFragmentStartTag;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use super::html_tree::HtmlTree;

mod html_fragment_end_tag;
mod html_fragment_start_tag;

pub struct HtmlFragment {
    start_tag: HtmlFragmentStartTag,
    children: Vec<HtmlTree>,
    end_tag: HtmlFragmentEndTag,
}

impl Parse for HtmlFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            let html_end_tag = input.parse::<HtmlFragmentEndTag>()?;
            return Err(syn::Error::new_spanned(
                html_end_tag.to_spanned(),
                "This closing fragment does not have a corresponding opening fragment. (hint: try adding `<>`)",
            ));
        }

        let start_tag = input.parse::<HtmlFragmentStartTag>()?;

        let mut children = Vec::new();
        while !HtmlFragmentEndTag::peek(input) {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    start_tag.to_spanned(),
                    "This opening fragment does not have a coressponding closing fragment. (hint: try adding `</>`)",
                ));
            }

            children.push(input.parse()?);
        }

        let end_tag = input.parse::<HtmlFragmentEndTag>()?;

        Ok(HtmlFragment {
            start_tag,
            children,
            end_tag,
        })
    }
}

impl ToTokens for HtmlFragment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        let key = self.start_tag.get_key_token_stream();

        tokens.extend(quote_spanned! {self.span() =>
            ::wal::virtual_dom::VNode::List(
                ::wal::virtual_dom::VList::new(
                    ::std::vec![#(#children),*],
                    #key
                )
            )
        });
    }
}

impl HtmlFragment {
    fn span(&self) -> proc_macro2::Span {
        self.to_spanned().span()
    }

    fn to_spanned(&self) -> impl ToTokens {
        let start_spanned = self.start_tag.to_spanned();
        let end_spanned = self.end_tag.to_spanned();
        quote!(#start_spanned #end_spanned)
    }
}
