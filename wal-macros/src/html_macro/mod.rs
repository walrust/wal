use std::collections::HashSet;

use once_cell::unsync::Lazy;

mod html_attribute;
mod html_component;
mod html_element;
mod html_expression_block;
mod html_for;
mod html_forest;
mod html_fragment;
mod html_if;
mod html_literal;
pub mod html_root;
mod html_tree;

const KEY_STR: &str = "key";
const PROPS_STR: &str = "props";

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
