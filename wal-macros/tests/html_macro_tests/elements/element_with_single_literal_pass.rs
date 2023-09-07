use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VNode, VText};

fn main() {
    element_with_empty_string();
    element_with_string();
    element_with_char();
    element_with_integer();
    element_with_float();
}

fn element_with_empty_string() {
    let html = html! { <div> "" </div> };
    assert_eq!(html, get_div_with(""));
}

fn element_with_string() {
    let html = html! { <div> "Hello world!" </div> };
    assert_eq!(html, get_div_with("Hello world!"));
}

fn element_with_char() {
    let html = html! { <div> 'a' </div> };
    assert_eq!(html, get_div_with('a'));
}

fn element_with_integer() {
    let html = html! { <div> 15 </div> };
    assert_eq!(html, get_div_with(15));
}

fn element_with_float() {
    let html = html! { <div> 15.5 </div> };
    assert_eq!(html, get_div_with(15.5));
}

fn get_div_with<T: ToString>(t: T) -> VNode {
    VNode::Element {
        velement: VElement::new(
            "div".to_string(),
            HashMap::new(),
            vec![VNode::Text {
                vtext: VText::new(t.to_string()),
            }],
        ),
    }
}
