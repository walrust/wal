use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::html_macro::html_attribute::{HtmlAttribute, HtmlAttributeValue};

pub struct HtmlComponentAttributes {
    pub props: Option<HtmlComponentAttribute>,
    _key: Option<HtmlAttribute>,
}

impl Parse for HtmlComponentAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut props = None;
        let mut key = None;

        while HtmlComponentAttribute::peek(input) {
            let attribute = input.parse::<HtmlComponentAttribute>()?;
            if attribute.ident == "props" {
                if props.is_some() {
                    return Err(syn::Error::new(
                        attribute.ident.span(),
                        format!("Duplicate attribute `{}`", attribute.ident),
                    ));
                }
                props = Some(attribute);
            } else if attribute.ident == "key" {
                if key.is_some() {
                    return Err(syn::Error::new(
                        attribute.ident.span(),
                        format!("Duplicate attribute `{}`", attribute.ident),
                    ));
                }
                key = Some(HtmlAttribute {
                    ident: attribute.ident,
                    value: attribute.value.into(),
                });
            } else {
                return Err(syn::Error::new(
                    attribute.ident.span(),
                    format!(
                        "Unsupported attribute `{}`. Custom components supports only `props` and `key` attributes",
                        attribute.ident
                    ),
                ));
            }
        }
        Ok(HtmlComponentAttributes { props, _key: key })
    }
}

pub struct HtmlComponentAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlComponentAttributeValue,
}

impl Parse for HtmlComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = proc_macro2::Ident::parse_any(&input)?;
        input.parse::<syn::token::Eq>()?;

        let value = if ident == "props" {
            input.parse::<HtmlComponentAttributeValue>()?
        } else if ident == "key" {
            input.parse::<HtmlAttributeValue>()?.into()
        } else {
            return Err(syn::Error::new(
                ident.span(),
                format!(
                    "Unsupported attribute `{}`. Custom components supports only `props` and `key` attributes",
                    ident
                ),
            ));
        };

        Ok(HtmlComponentAttribute { ident, value })
    }
}

impl HtmlComponentAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(proc_macro2::Ident::peek_any)
    }

    pub fn to_spanned(&self) -> impl ToTokens {
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
