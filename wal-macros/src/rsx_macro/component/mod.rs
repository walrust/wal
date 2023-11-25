use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use self::component_attributes::ComponentAttributes;

pub mod component_attribute;
pub mod component_attributes;

pub struct Component {
    lt: syn::token::Lt,
    ty: syn::Type,
    attributes: ComponentAttributes,
    gt: syn::token::Gt,
}

impl Parse for Component {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let ty = input.parse()?;
        let attributes = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let gt = input.parse()?;

        Ok(Component {
            lt,
            ty,
            attributes,
            gt,
        })
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.ty;
        let props = self.attributes.get_props_token_stream(ty);
        let key = self.attributes.get_key_token_stream();

        tokens.extend(quote_spanned! { self.error_span() =>
            ::wal::virtual_dom::VNode::Component(
                ::wal::virtual_dom::VComponent::new::<#ty>(#props, #key)
            )
        });
    }
}

impl Component {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote! { #lt #gt }
    }
}
