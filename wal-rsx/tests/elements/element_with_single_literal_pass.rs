use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode, VText};
use wal_rsx::rsx;

fn main() {
    element_with_empty_string();
    element_with_string();
    element_with_char();
    element_with_integer();
    element_with_float();
}

fn element_with_empty_string() {
    let rsx = rsx! { <div> "" </div> };
    assert_eq!(rsx, get_div_with(""));
}

fn element_with_string() {
    let rsx = rsx! { <div> "Hello world!" </div> };
    assert_eq!(rsx, get_div_with("Hello world!"));
}

fn element_with_char() {
    let rsx = rsx! { <div> 'a' </div> };
    assert_eq!(rsx, get_div_with('a'));
}

fn element_with_integer() {
    let rsx = rsx! { <div> 15 </div> };
    assert_eq!(rsx, get_div_with(15));
}

fn element_with_float() {
    let rsx = rsx! { <div> 15.5 </div> };
    assert_eq!(rsx, get_div_with(15.5));
}

fn get_div_with<T: ToString>(t: T) -> VNode {
    VNode::Element(VElement::new(
        "div".to_string(),
        HashMap::new(),
        Vec::new(),
        None,
        vec![VNode::Text(VText::new(t.to_string()))],
    ))
}
