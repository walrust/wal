use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Type,
};

use crate::html_macro::{
    html_attribute::{HtmlAttribute, HtmlAttributeValue},
    KEY_ATTR, PROPS_ATTR,
};

pub struct HtmlComponentAttributes {
    pub props: Option<HtmlComponentAttribute>,
    key: Option<HtmlAttribute>,
}

impl Parse for HtmlComponentAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut props = None;
        let mut key = None;

        while HtmlComponentAttribute::peek(input) {
            let attribute = input.parse::<HtmlComponentAttribute>()?;
            Self::process_attribute(&mut props, &mut key, attribute)?;
        }
        Ok(HtmlComponentAttributes { props, key })
    }
}

impl HtmlComponentAttributes {
    fn process_attribute(
        props: &mut Option<HtmlComponentAttribute>,
        key: &mut Option<HtmlAttribute>,
        attribute: HtmlComponentAttribute,
    ) -> syn::Result<()> {
        if attribute.ident == PROPS_ATTR {
            Self::process_props_attribute(props, attribute)
        } else if attribute.ident == KEY_ATTR {
            Self::process_key_attribute(key, attribute)
        } else {
            Err(syn::Error::new(
                attribute.ident.span(),
                format!(
                    "Unsupported attribute `{}`. Custom components supports only `{}` and `{}` attributes",
                    attribute.ident,
                    PROPS_ATTR,
                    KEY_ATTR
                ),
            ))
        }
    }

    fn process_props_attribute(
        props: &mut Option<HtmlComponentAttribute>,
        attribute: HtmlComponentAttribute,
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
        key: &mut Option<HtmlAttribute>,
        attribute: HtmlComponentAttribute,
    ) -> syn::Result<()> {
        if key.is_some() {
            return Err(syn::Error::new(
                attribute.ident.span(),
                format!("Duplicate attribute `{}`", attribute.ident),
            ));
        }
        *key = Some(attribute.into());
        Ok(())
    }
}

impl HtmlComponentAttributes {
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
                HtmlComponentAttributeValue::Literal(lit) => quote_spanned!(props.span() => #lit),
                HtmlComponentAttributeValue::StructExpression(expr_struct) => {
                    quote_spanned!(props.span() => #expr_struct)
                }
                HtmlComponentAttributeValue::ExpressionBlock(expr_block) => {
                    quote_spanned!(props.span() => #[allow(unused_braces)] #expr_block)
                }
            },
        )
    }
}

pub struct HtmlComponentAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlComponentAttributeValue,
}

impl Parse for HtmlComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<proc_macro2::Ident>()?;
        input.parse::<syn::token::Eq>()?;

        let value = if ident == PROPS_ATTR {
            input.parse::<HtmlComponentAttributeValue>()?
        } else if ident == KEY_ATTR {
            input.parse::<HtmlAttributeValue>()?.into()
        } else {
            return Err(syn::Error::new(
                ident.span(),
                format!(
                    "Unsupported attribute `{}`. Custom components supports only `{}` and `{}` attributes",
                    ident,
                    PROPS_ATTR,
                    KEY_ATTR
                ),
            ));
        };

        Ok(HtmlComponentAttribute { ident, value })
    }
}

impl HtmlComponentAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }

    pub fn span(&self) -> proc_macro2::Span {
        self.to_spanned().span()
    }

    fn to_spanned(&self) -> impl ToTokens {
        let ident = &self.ident;
        let value = &self.value;
        quote! { #ident #value }
    }
}

pub enum HtmlComponentAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
    StructExpression(syn::ExprStruct),
}

impl Parse for HtmlComponentAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute_value = if input.peek(syn::Lit) {
            HtmlComponentAttributeValue::Literal(input.parse()?)
        } else if input.fork().parse::<syn::ExprStruct>().is_ok() {
            HtmlComponentAttributeValue::StructExpression(input.parse()?)
        } else if let Ok(expr_block) = input.parse::<syn::ExprBlock>() {
            if expr_block.block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &expr_block,
                    "Expected a non-empty expression block",
                ));
            }
            HtmlComponentAttributeValue::ExpressionBlock(expr_block)
        } else {
            return Err(input
                .error("Expected a literal, a struct literal expression or an expression block"));
        };

        Ok(attribute_value)
    }
}

impl ToTokens for HtmlComponentAttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlComponentAttributeValue::Literal(lit) => lit.to_tokens(tokens),
            HtmlComponentAttributeValue::StructExpression(expr_struct) => {
                expr_struct.to_tokens(tokens)
            }
            HtmlComponentAttributeValue::ExpressionBlock(expr_block) => {
                expr_block.to_tokens(tokens)
            }
        }
    }
}

impl From<HtmlAttributeValue> for HtmlComponentAttributeValue {
    fn from(value: HtmlAttributeValue) -> Self {
        match value {
            HtmlAttributeValue::Literal(lit) => HtmlComponentAttributeValue::Literal(lit),
            HtmlAttributeValue::ExpressionBlock(expr_block) => {
                HtmlComponentAttributeValue::ExpressionBlock(expr_block)
            }
        }
    }
}
