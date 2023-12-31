use std::collections::HashMap;
use wal_core::virtual_dom::{VElement, VList, VNode, VText};
use wal_rsx::rsx;

fn main() {
    if_with_empty();
    if_with_for();
    if_with_multiple_element_forest();
}

fn if_with_empty() {
    let rsx = rsx! {
        if true {
        }
    };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn if_with_for() {
    let rsx = rsx! {
        if true {
            for 0..3
        }
    };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![
                VNode::Text(VText::new("0")),
                VNode::Text(VText::new("1")),
                VNode::Text(VText::new("2")),
            ],
            None
        ))
    );
}

fn if_with_multiple_element_forest() {
    let rsx = rsx! {
        if true {
            <div></div>
            <div></div>
        }
    };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![
                VNode::Element(VElement::new(
                    "div".to_string(),
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new(),
                )),
                VNode::Element(VElement::new(
                    "div".to_string(),
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new(),
                )),
            ],
            None
        ))
    );
}
