use std::collections::HashSet;

use once_cell::sync::Lazy;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use super::Attribute;

pub(crate) type EventAttributeValue = syn::ExprBlock;

pub(crate) struct EventAttribute {
    pub(crate) ident: proc_macro2::Ident,
    pub(crate) value: EventAttributeValue,
}

impl Parse for EventAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;
        let value = input.parse::<syn::ExprBlock>()?;
        if value.block.stmts.is_empty() {
            return Err(syn::Error::new_spanned(
                &value,
                "Expected a non-empty expression block",
            ));
        }

        Ok(EventAttribute { ident, value })
    }
}

impl ToTokens for EventAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens);
    }
}

impl Attribute for EventAttribute {
    type AttributeValue = EventAttributeValue;

    fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    fn value(&self) -> &Self::AttributeValue {
        &self.value
    }
}

pub(crate) trait IsEvent {
    fn is_event(&self) -> bool;
}

impl IsEvent for proc_macro2::Ident {
    fn is_event(&self) -> bool {
        EVENTS.contains(self.to_string().as_str())
    }
}

static EVENTS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
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
