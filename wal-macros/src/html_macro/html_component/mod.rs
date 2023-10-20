use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use self::html_component_attributes::HtmlComponentAttributes;

use super::html_attribute::HtmlAttributeValue;

mod html_component_attributes;

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
        let props_type =
            quote_spanned!(self.ty.span() => <#ty as ::wal::component::Component>::Properties);
        let props = if let Some(props) = &self.attributes.props {
            match &props.value {
                HtmlAttributeValue::Literal(lit) => {
                    quote_spanned!(props.to_spanned().span() => <#lit as #props_type>)
                }
                HtmlAttributeValue::ExpressionBlock(expr_block) => {
                    quote_spanned!(props.to_spanned().span() => <#expr_block as #props_type>)
                }
            }
        } else {
            quote_spanned!(self.ty.span() =>  <#props_type as ::std::default::Default>::default())
        };
        tokens.extend(quote_spanned! { self.to_spanned().span() =>
            ::wal::virtual_dom::VNode::Component(
                ::wal::virtual_dom::VComponent::new::<#ty>(#props)
            )
        });
    }
}

impl HtmlComponent {
    fn to_spanned(&self) -> impl ToTokens {
        let lt = &self.lt;
        let gt = &self.gt;
        quote!( #lt #gt )
    }
}
