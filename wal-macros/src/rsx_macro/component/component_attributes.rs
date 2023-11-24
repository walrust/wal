use quote::{quote, quote_spanned};
use syn::{parse::Parse, spanned::Spanned, Type};

use crate::rsx_macro::attributes::{
    normal_attribute::NormalAttribute,
    props_attribute::{PropsAttribute, PropsAttributeValue},
};

use super::component_attribute::ComponentAttribute;

pub struct ComponentAttributes {
    pub props: Option<PropsAttribute>,
    key: Option<NormalAttribute>,
}

impl Parse for ComponentAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut props = None;
        let mut key = None;

        while ComponentAttribute::peek(input) {
            let attribute = input.parse::<ComponentAttribute>()?;
            Self::process_attribute(&mut props, &mut key, attribute)?;
        }
        Ok(ComponentAttributes { props, key })
    }
}

impl ComponentAttributes {
    fn process_attribute(
        props: &mut Option<PropsAttribute>,
        key: &mut Option<NormalAttribute>,
        attribute: ComponentAttribute,
    ) -> syn::Result<()> {
        match attribute {
            ComponentAttribute::Props(props_attribute) => {
                Self::process_props_attribute(props, props_attribute)
            }
            ComponentAttribute::Key(key_attribute) => {
                Self::process_key_attribute(key, key_attribute)
            }
        }
    }

    fn process_props_attribute(
        props: &mut Option<PropsAttribute>,
        attribute: PropsAttribute,
    ) -> syn::Result<()> {
        if props.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *props = Some(attribute);
        Ok(())
    }

    fn process_key_attribute(
        key: &mut Option<NormalAttribute>,
        attribute: NormalAttribute,
    ) -> syn::Result<()> {
        if key.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *key = Some(attribute);
        Ok(())
    }
}

impl ComponentAttributes {
    pub(crate) fn get_key_token_stream(&self) -> proc_macro2::TokenStream {
        if let Some(key) = &self.key {
            let key_val = &key.value;
            quote_spanned!(key.ident.span() => Some(#key_val.to_string()))
        } else {
            quote!(None)
        }
    }

    pub(crate) fn get_props_token_stream(&self, component_type: &Type) -> proc_macro2::TokenStream {
        let props_type = quote_spanned!(component_type.span() => <#component_type as ::wal::component::Component>::Properties);

        self.props.as_ref().map_or_else(
            || quote_spanned!(component_type.span() => <#props_type as ::std::default::Default>::default()),
            |props| match &props.value {
                PropsAttributeValue::Literal(lit) => quote_spanned!(props.span() => #lit),
                PropsAttributeValue::StructExpression(expr_struct) => {
                    quote_spanned!(props.span() => #expr_struct)
                }
                PropsAttributeValue::ExpressionBlock(expr_block) => {
                    quote_spanned!(props.span() => #[allow(unused_braces)] #expr_block)
                }
            },
        )
    }
}
