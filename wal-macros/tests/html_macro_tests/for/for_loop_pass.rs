use std::collections::HashMap;
use wal::virtual_dom::{VElement, VList, VNode, VText};
use wal_macros::html;

include!("../utils/new_velement_str.rs");

fn main() {
    for_empty_iter();
    for_empty_vec();

    for_non_empty_vec();
    for_non_empty_iter();

    for_with_mapped_elements();
    for_with_mapped_elements_to_html();

    for_as_first_child_of_element();
    for_as_a_not_first_child_of_element();
}

fn for_empty_iter() {
    let html = html! { for empty_iter() };
    assert_eq!(html, VNode::List(VList::new_empty()));
}

fn for_empty_vec() {
    let html = html! { for empty_vec() };
    assert_eq!(html, VNode::List(VList::new_empty()));
}

fn for_non_empty_vec() {
    let html = html! { for vec![0, 1, 2] };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Text(VText::new("0")),
            VNode::Text(VText::new("1")),
            VNode::Text(VText::new("2")),
        ]))
    );
}

fn for_non_empty_iter() {
    let html = html! { for 0..3 };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Text(VText::new("0")),
            VNode::Text(VText::new("1")),
            VNode::Text(VText::new("2")),
        ]))
    );
}

fn for_with_mapped_elements() {
    let html = html! { for std::iter::Iterator::map(0..2, |num| {num + 1}) };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Text(VText::new("1")),
            VNode::Text(VText::new("2")),
        ]))
    );
}

fn for_with_mapped_elements_to_html() {
    let html = html! { for std::iter::Iterator::map(0..2, |num| { html! { <div>{ num }</div> } }) };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Element(new_velement_str(
                "div",
                HashMap::new(),
                vec![VNode::Text(VText::new("0"))]
            )),
            VNode::Element(new_velement_str(
                "div",
                HashMap::new(),
                vec![VNode::Text(VText::new("1"))]
            )),
        ]))
    );
}

fn for_as_first_child_of_element() {
    let html = html! {
        <div>
            for { empty_vec() }
        </div>
    };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![VNode::List(VList::new_empty())],
        )),
    );
}

fn for_as_a_not_first_child_of_element() {
    let html = html! {
        <>
            <div/>
            for { empty_vec() }
        </>
    };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Element(new_velement_str("div", HashMap::new(), vec![])),
            VNode::List(VList::new_empty()),
        ]))
    );
}

fn empty_iter() -> impl std::iter::Iterator<Item = i32> {
    std::iter::empty()
}

fn empty_vec() -> std::vec::Vec<i32> {
    std::vec::Vec::new()
}
