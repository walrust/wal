use std::collections::HashSet;

use once_cell::unsync::Lazy;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use super::html_component::html_component_attributes::{
    HtmlComponentAttribute, HtmlComponentAttributeValue,
};

pub struct HtmlAttribute {
    pub ident: proc_macro2::Ident,
    pub value: HtmlAttributeValue,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse()?;

        Ok(HtmlAttribute { ident, value })
    }
}

impl HtmlAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
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
        // Animation Events
        "onanimationcancel",
        "onanimationend",
        "onanimationiteration",
        "onanimationstart",
        // Drag Events
        "ondrag",
        "ondragend",
        "ondragenter",
        "ondragexit",
        "ondragleave",
        "ondragover",
        "ondragstart",
        "ondrop",
        // Focus Events
        "onblur",
        "onfocus",
        "onfocusin",
        "onfocusout",
        // FormData Events
        "onformdata", // web_sys is missing `FormDataEvent`` so it is handled as Unspecialized Event
        // Input Events
        "oninput",
        // Keyboard Events
        "onkeydown",
        "onkeypress",
        "onkeyup",
        // Mouse Events
        "onauxclick",
        "onclick",
        "oncontextmenu",
        "ondblclick",
        "onmousedown",
        "onmouseenter",
        "onmouseleave",
        "onmousemove",
        "onmouseout",
        "onmouseover",
        "onmouseup",
        // Pointer Events
        "ongotpointercapture",
        "onlostpointercapture",
        "onpointercancel",
        "onpointerdown",
        "onpointerenter",
        "onpointerleave",
        "onpointermove",
        "onpointerout",
        "onpointerover",
        "onpointerup",
        // Progress Events
        "onloadend",
        "onloadstart",
        "onprogress",
        // Submit Events
        "onsubmit",
        // Touch Events
        "ontouchcancel",
        "ontouchend",
        "ontouchmove",
        "ontouchstart",
        // Transition Events
        "ontransitioncancel",
        "ontransitionend",
        "ontransitionrun",
        "ontransitionstart",
        // Unspecialized Events
        "onabort",
        "oncancel",
        "oncanplay",
        "oncanplaythrough",
        "onchange",
        "onclose",
        "oncopy",
        "oncuechange",
        "oncut",
        "ondurationchange",
        "onemptied",
        "onended",
        "onerror",
        "oninvalid",
        "onload",
        "onloadeddata",
        "onloadedmetadata",
        "onpaste",
        "onpause",
        "onplay",
        "onplaying",
        "onpointerlockchange",
        "onpointerlockerror",
        "onratechange",
        "onreset",
        "onresize",
        "onscroll",
        "onsecuritypolicyviolation",
        "onseeked",
        "onseeking",
        "onselect",
        "onselectionchange",
        "onselectstart",
        "onshow",
        "onslotchange",
        "onstalled",
        "onsuspend",
        "ontimeupdate",
        "ontoggle",
        "onvolumechange",
        "onwaiting",
        // Wheel Events
        "onwheel",
    ]
    .into()
});
