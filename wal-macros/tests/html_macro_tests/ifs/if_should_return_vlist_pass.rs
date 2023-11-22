use std::collections::HashMap;
use wal::virtual_dom::{VElement, VList, VNode, VText};
use wal_macros::html;

include!("../utils/wrap_in_list.rs");

fn main() {
    if_with_empty();
    if_with_literal();
    if_with_expression();
    if_with_for();
    if_with_single_element_forest();
    if_with_multiple_element_forest();
}

fn if_with_empty() {
    let html = html! {
        if true {
        }
    };
    assert_eq!(html, VNode::List(VList::new_empty(None)));
}

fn if_with_literal() {
    let html = html! {
        if true {
            "hello"
        }
    };
    assert_eq!(html, wrap_in_list(VNode::Text(VText::new("hello"))));
}

fn if_with_expression() {
    let html = html! {
        if true {
            1 + 1
        }
    };
    assert_eq!(html, wrap_in_list(VNode::Text(VText::new("2"))));
}

fn if_with_for() {
    let html = html! {
        if true {
            for 0..3
        }
    };
    assert_eq!(
        html,
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

fn if_with_single_element_forest() {
    let html = html! {
        if true {
            <div></div>
        }
    };
    assert_eq!(
        html,
        wrap_in_list(VNode::Element(VElement::new(
            "div".to_string(),
            HashMap::new(),
            Vec::new(),
            None,
            Vec::new(),
        )))
    );
}

fn if_with_multiple_element_forest() {
    let html = html! {
        if true {
            <div></div>
            <div></div>
        }
    };
    assert_eq!(
        html,
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
