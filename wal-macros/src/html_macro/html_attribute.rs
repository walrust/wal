use std::collections::HashSet;

use once_cell::unsync::Lazy;
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use super::html_component::html_component_attributes::{
    HtmlComponentAttribute, HtmlComponentAttributeValue,
};

pub struct HtmlAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlAttributeValue,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = proc_macro2::Ident::parse_any(&input)?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(HtmlAttribute { ident, value })
    }
}

impl HtmlAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(proc_macro2::Ident::peek_any)
    }

    pub fn is_event(&self) -> bool {
        EVENTS.contains(&self.ident.to_string().as_str())
    }
}

impl From<HtmlComponentAttribute> for HtmlAttribute {
    fn from(attribute: HtmlComponentAttribute) -> Self {
        HtmlAttribute {
            ident: attribute.ident,
            value: attribute.value.into(),
        }
    }
}

pub enum HtmlAttributeValue {
    Literal(syn::Lit),
    ExpressionBlock(syn::ExprBlock),
}

impl Parse for HtmlAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute_value = if input.peek(syn::Lit) {
            HtmlAttributeValue::Literal(input.parse()?)
        } else if let Ok(expr_block) = input.parse::<syn::ExprBlock>() {
            if expr_block.block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &expr_block,
                    "Expected a non-empty expression block",
                ));
            }
            HtmlAttributeValue::ExpressionBlock(expr_block)
        } else {
            return Err(input.error("Expected a literal or an expression block"));
        };

        Ok(attribute_value)
    }
}

impl ToTokens for HtmlAttributeValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlAttributeValue::Literal(lit) => lit.to_tokens(tokens),
            HtmlAttributeValue::ExpressionBlock(expr_block) => expr_block.to_tokens(tokens),
        }
    }
}

impl From<HtmlComponentAttributeValue> for HtmlAttributeValue {
    fn from(value: HtmlComponentAttributeValue) -> Self {
        match value {
            HtmlComponentAttributeValue::Literal(lit) => HtmlAttributeValue::Literal(lit),
            HtmlComponentAttributeValue::ExpressionBlock(expr_block) => {
                HtmlAttributeValue::ExpressionBlock(expr_block)
            }
            _ => panic!(
                "Unsupported conversion from HtmlComponentAttributeValue to HtmlAttributeValue - should never happen"
            ),
        }
    }
}

// Events from https://www.w3schools.com/tags/ref_eventattributes.asp
const EVENTS: Lazy<HashSet<&str>> = Lazy::new(|| {
    [
        // Window Event Attributes
        "onafterprint",
        "onbeforeprint",
        "onbeforeunload",
        "onerror",
        "onhashchange",
        "onload",
        "onmessage",
        "onoffline",
        "ononline",
        "onpagehide",
        "onpageshow",
        "onpopstate",
        "onresize",
        "onstorage",
        "onunload",
        // Form Events
        "onblur",
        "onchange",
        "oncontextmenu",
        "onfocus",
        "oninput",
        "oninvalid",
        "onreset",
        "onsearch",
        "onselect",
        "onsubmit",
        // Keyboard Events
        "onkeydown",
        "onkeypress",
        "onkeyup",
        // Mouse Events
        "onclick",
        "ondblclick",
        "onmousedown",
        "onmousemove",
        "onmouseout",
        "onmouseover",
        "onmouseup",
        "onmousewheel",
        "onscroll",
        "onwheel",
        // Drag Events
        "ondrag",
        "ondragend",
        "ondragenter",
        "ondragleave",
        "ondragover",
        "ondragstart",
        "ondrop",
        // Clipboard Events
        "oncopy",
        "oncut",
        "onpaste",
        // Media Events
        "onabort",
        "oncanplay",
        "oncanplaythrough",
        "oncuechange",
        "ondurationchange",
        "onemptied",
        "onended",
        "onerror",
        "onloadeddata",
        "onloadedmetadata",
        "onloadstart",
        "onpause",
        "onplay",
        "onplaying",
        "onprogress",
        "onratechange",
        "onseeked",
        "onseeking",
        "onstalled",
        "onsuspend",
        "ontimeupdate",
        "onvolumechange",
        "onwaiting",
        // Misc Events
        "ontoggle",
    ]
    .into()
});
