use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode};
use wal_macros::rsx;

fn main() {
    let html = rsx! { <div></div> };
    assert_eq!(html, get_velement("div"));
    let html = rsx! { <span></span> };
    assert_eq!(html, get_velement("span"));
    let html = rsx! { <div/> };
    assert_eq!(html, get_velement("div"));
    let html = rsx! { <span/> };
    assert_eq!(html, get_velement("span"));
    let html = rsx! { <area/> };
    assert_eq!(html, get_velement("area"));
    let html = rsx! { <base/> };
    assert_eq!(html, get_velement("base"));
    let html = rsx! { <br/> };
    assert_eq!(html, get_velement("br"));
    let html = rsx! { <col/> };
    assert_eq!(html, get_velement("col"));
    let html = rsx! { <embed/> };
    assert_eq!(html, get_velement("embed"));
    let html = rsx! { <hr/> };
    assert_eq!(html, get_velement("hr"));
    let html = rsx! { <img/> };
    assert_eq!(html, get_velement("img"));
    let html = rsx! { <input/> };
    assert_eq!(html, get_velement("input"));
    let html = rsx! { <link/> };
    assert_eq!(html, get_velement("link"));
    let html = rsx! { <meta/> };
    assert_eq!(html, get_velement("meta"));
    let html = rsx! { <param/> };
    assert_eq!(html, get_velement("param"));
    let html = rsx! { <source/> };
    assert_eq!(html, get_velement("source"));
    let html = rsx! { <track/> };
    assert_eq!(html, get_velement("track"));
    let html = rsx! { <wbr/> };
    assert_eq!(html, get_velement("wbr"));
}

fn get_velement(tag: &str) -> VNode {
    VNode::Element(VElement::new(
        tag.to_string(),
        HashMap::new(),
        Vec::new(),
        None,
        Vec::new(),
    ))
}
