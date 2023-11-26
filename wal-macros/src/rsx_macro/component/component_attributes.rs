use quote::quote_spanned;
use syn::{parse::Parse, spanned::Spanned, Type};

use crate::rsx_macro::attributes::{
    normal_attribute::NormalAttribute,
    process_specialized_attribute,
    props_attribute::{PropsAttribute, PropsAttributeValue},
};

use super::component_attribute::ComponentAttribute;

pub(crate) struct ComponentAttributes {
    props: Option<PropsAttribute>,
    key: Option<NormalAttribute>,
}

impl Parse for ComponentAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut props = None;
        let mut key = None;

        while ComponentAttribute::peek(input) {
            let incoming_attribute = input.parse::<ComponentAttribute>()?;
            Self::process_attribute(&mut props, &mut key, incoming_attribute)?;
        }
        Ok(ComponentAttributes { props, key })
    }
}

impl ComponentAttributes {
    fn process_attribute(
        props: &mut Option<PropsAttribute>,
        key: &mut Option<NormalAttribute>,
        incoming_attribute: ComponentAttribute,
    ) -> syn::Result<()> {
        match incoming_attribute {
            ComponentAttribute::Props(incoming_props_attribute) => {
                process_specialized_attribute(props, incoming_props_attribute)
            }
            ComponentAttribute::Key(incoming_key_attribute) => {
                process_specialized_attribute(key, incoming_key_attribute)
            }
        }
    }
}

impl ComponentAttributes {
    pub(crate) fn get_key_attribute_token_stream(&self) -> proc_macro2::TokenStream {
        NormalAttribute::get_key_attribute_token_stream(self.key.as_ref())
    }

    pub(crate) fn get_props_attribute_token_stream(
        &self,
        component_type: &Type,
    ) -> proc_macro2::TokenStream {
        let props_type = quote_spanned!(component_type.span() => <#component_type as ::wal::component::Component>::Properties);

        self.props.as_ref().map_or_else(
            || quote_spanned!(component_type.span() => <#props_type as ::std::default::Default>::default()),
            |props| match &props.value {
                PropsAttributeValue::Literal(lit) => quote_spanned!(lit.span() => #lit),
                PropsAttributeValue::StructExpression(expr_struct) => {
                    quote_spanned!(expr_struct.span() => #expr_struct)
                }
                PropsAttributeValue::ExpressionBlock(expr_block) => {
                    quote_spanned!(expr_block.span() => #[allow(unused_braces)] #expr_block)
                }
            },
        )
    }
}
