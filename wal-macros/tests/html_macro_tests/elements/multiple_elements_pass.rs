use std::collections::HashMap;
use wal::virtual_dom::{VElement, VList, VNode};
use wal_macros::html;

fn main() {
    let html = html! {
        <div></div>
        <span/>
        <input/>
    };
    assert_eq!(
        html,
        VNode::List(VList::new(
            vec![
                get_velement("div"),
                get_velement("span"),
                get_velement("input"),
            ],
            None
        ))
    )
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
