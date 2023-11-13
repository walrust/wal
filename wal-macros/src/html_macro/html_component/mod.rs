use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use self::html_component_attributes::HtmlComponentAttributes;

pub mod html_component_attributes;

pub struct HtmlComponent {
    lt: syn::token::Lt,
    ty: syn::Type,
    attributes: HtmlComponentAttributes,
    gt: syn::token::Gt,
}

impl Parse for HtmlComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let ty = input.parse()?;
        let attributes = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let gt = input.parse()?;

        Ok(HtmlComponent {
            lt,
            ty,
            attributes,
            gt,
        })
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.ty;
        let props = self.attributes.get_props_token_stream(ty);
        let key = self.attributes.get_key_token_stream();

        tokens.extend(quote_spanned! { self.span() =>
            ::wal::virtual_dom::VNode::Component(
                ::wal::virtual_dom::VComponent::new::<#ty>(#props, #key)
            )
        });
    }
}

impl HtmlComponent {
    fn span(&self) -> proc_macro2::Span {
        self.to_spanned().span()
    }

    fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
