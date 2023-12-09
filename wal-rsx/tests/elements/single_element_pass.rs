use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode};
use wal_rsx::rsx;

fn main() {
    let rsx = rsx! { <div></div> };
    assert_eq!(rsx, get_velement("div"));
    let rsx = rsx! { <span></span> };
    assert_eq!(rsx, get_velement("span"));
    let rsx = rsx! { <div/> };
    assert_eq!(rsx, get_velement("div"));
    let rsx = rsx! { <span/> };
    assert_eq!(rsx, get_velement("span"));
    let rsx = rsx! { <area/> };
    assert_eq!(rsx, get_velement("area"));
    let rsx = rsx! { <base/> };
    assert_eq!(rsx, get_velement("base"));
    let rsx = rsx! { <br/> };
    assert_eq!(rsx, get_velement("br"));
    let rsx = rsx! { <col/> };
    assert_eq!(rsx, get_velement("col"));
    let rsx = rsx! { <embed/> };
    assert_eq!(rsx, get_velement("embed"));
    let rsx = rsx! { <hr/> };
    assert_eq!(rsx, get_velement("hr"));
    let rsx = rsx! { <img/> };
    assert_eq!(rsx, get_velement("img"));
    let rsx = rsx! { <input/> };
    assert_eq!(rsx, get_velement("input"));
    let rsx = rsx! { <link/> };
    assert_eq!(rsx, get_velement("link"));
    let rsx = rsx! { <meta/> };
    assert_eq!(rsx, get_velement("meta"));
    let rsx = rsx! { <param/> };
    assert_eq!(rsx, get_velement("param"));
    let rsx = rsx! { <source/> };
    assert_eq!(rsx, get_velement("source"));
    let rsx = rsx! { <track/> };
    assert_eq!(rsx, get_velement("track"));
    let rsx = rsx! { <wbr/> };
    assert_eq!(rsx, get_velement("wbr"));
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
