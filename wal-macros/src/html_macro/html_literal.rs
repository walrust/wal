use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};

pub struct HtmlLiteral(syn::Lit);

impl Parse for HtmlLiteral {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit = input.parse()?;

        let error_message = match lit {
            syn::Lit::ByteStr(_) => Some("Byte string literals are not supported"),
            syn::Lit::Verbatim(_) => Some("Raw token literals are not supported"),
            _ => None,
        };

        if let Some(msg) = error_message {
            return Err(syn::Error::new(lit.span(), msg));
        };

        Ok(Self(lit))
    }
}

impl ToTokens for HtmlLiteral {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let literal = &self.0;
        tokens.extend(
            quote_spanned! { literal.span() => ::wal_vdom::virtual_dom::VNode::Text {
                vtext: ::wal_vdom::virtual_dom::VText::new(#literal)
            }},
        );
    }
}
