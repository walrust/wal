use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

pub struct HtmlAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlAttributeValue,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = proc_macro2::Ident::parse_any(&input)?;
        input.parse::<syn::token::Eq>()?;

        let value = if input.peek(syn::Lit) {
            HtmlAttributeValue::Literal(input.parse()?)
        } else {
            HtmlAttributeValue::ExpressionBlock(input.parse()?)
        };

        Ok(HtmlAttribute { ident, value })
    }
}

impl HtmlAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(proc_macro2::Ident::peek_any)
    }

    pub fn to_spanned(&self) -> impl ToTokens {
        let ident = &self.ident;
        let value = self.value.to_spanned();
        quote! { #ident #value }
    }
}

pub enum HtmlAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
}

impl HtmlAttributeValue {
    fn to_spanned(&self) -> impl ToTokens {
        match self {
            HtmlAttributeValue::Literal(lit) => quote! { #lit },
            HtmlAttributeValue::ExpressionBlock(expr_block) => quote! { #expr_block },
        }
    }
}
