use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VList, VNode};

include!("../utils/new_element_str.rs");

fn main() {
    empty();
    with_single_element();
    with_multiple_elements();
    inside_element();
    with_key_attribute();
}

fn empty() {
    let html = html! { <></> };
    assert_eq!(html, VNode::List(VList::new_empty()));
}

fn with_single_element() {
    let html = html! {
        <>
            <div></div>
        </>
    };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            Vec::new()
        ))]),)
    );
}

fn with_multiple_elements() {
    let html = html! {
        <>
            <div></div>
            <div></div>
        </>
    };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
            VNode::Element(new_velement_str("div", HashMap::new(), Vec::new()),),
        ]))
    );
}

fn inside_element() {
    let html = html! {
        <div>
            <></>
        </div>
    };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![VNode::List(VList::new_empty())],
        ))
    );
}

fn with_key_attribute() {
    let html = html! { <key="value"></> };
    assert_eq!(html, VNode::List(VList::new_empty()));
}
