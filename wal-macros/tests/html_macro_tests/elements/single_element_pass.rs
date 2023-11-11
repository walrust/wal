use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode};
use wal_macros::html;

fn main() {
    let html = html! { <div></div> };
    assert_eq!(html, get_velement("div"));
    let html = html! { <span></span> };
    assert_eq!(html, get_velement("span"));
    let html = html! { <div/> };
    assert_eq!(html, get_velement("div"));
    let html = html! { <span/> };
    assert_eq!(html, get_velement("span"));
    let html = html! { <area/> };
    assert_eq!(html, get_velement("area"));
    let html = html! { <base/> };
    assert_eq!(html, get_velement("base"));
    let html = html! { <br/> };
    assert_eq!(html, get_velement("br"));
    let html = html! { <col/> };
    assert_eq!(html, get_velement("col"));
    let html = html! { <embed/> };
    assert_eq!(html, get_velement("embed"));
    let html = html! { <hr/> };
    assert_eq!(html, get_velement("hr"));
    let html = html! { <img/> };
    assert_eq!(html, get_velement("img"));
    let html = html! { <input/> };
    assert_eq!(html, get_velement("input"));
    let html = html! { <link/> };
    assert_eq!(html, get_velement("link"));
    let html = html! { <meta/> };
    assert_eq!(html, get_velement("meta"));
    let html = html! { <param/> };
    assert_eq!(html, get_velement("param"));
    let html = html! { <source/> };
    assert_eq!(html, get_velement("source"));
    let html = html! { <track/> };
    assert_eq!(html, get_velement("track"));
    let html = html! { <wbr/> };
    assert_eq!(html, get_velement("wbr"));
}

fn get_velement(tag: &str) -> VNode {
    VNode::Element(VElement::new(
        tag.to_string(),
        HashMap::new(),
        Vec::new(),
        Vec::new(),
    ))
}
