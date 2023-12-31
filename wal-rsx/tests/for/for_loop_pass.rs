use std::collections::HashMap;
use wal_core::{
    events::EventHandler,
    virtual_dom::{VElement, VList, VNode, VText},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    for_empty_iter();
    for_empty_vec();

    for_non_empty_vec();
    for_non_empty_iter();

    for_with_mapped_elements();
    for_with_mapped_elements_to_rsx();

    for_as_first_child_of_element();
    for_as_a_not_first_child_of_element();
}

fn for_empty_iter() {
    let rsx = rsx! { for empty_iter() };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn for_empty_vec() {
    let rsx = rsx! { for empty_vec() };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn for_non_empty_vec() {
    let rsx = rsx! { for vec![0, 1, 2] };
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

fn for_non_empty_iter() {
    let rsx = rsx! { for 0..3 };
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

fn for_with_mapped_elements() {
    let rsx = rsx! { for std::iter::Iterator::map(0..2, |num| {num + 1}) };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![VNode::Text(VText::new("1")), VNode::Text(VText::new("2")),],
            None
        ))
    );
}

fn for_with_mapped_elements_to_rsx() {
    let rsx = rsx! { for std::iter::Iterator::map(0..2, |num| { rsx! { <div>{ num }</div> } }) };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    vec![VNode::Text(VText::new("0"))]
                )),
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    vec![VNode::Text(VText::new("1"))]
                )),
            ],
            None
        ))
    );
}

fn for_as_first_child_of_element() {
    let rsx = rsx! {
        <div>
            for { empty_vec() }
        </div>
    };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            Vec::new(),
            None,
            vec![VNode::List(VList::new_empty(None))],
        )),
    );
}

fn for_as_a_not_first_child_of_element() {
    let rsx = rsx! {
        <>
            <div/>
            for { empty_vec() }
        </>
    };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new()
                )),
                VNode::List(VList::new_empty(None)),
            ],
            None
        ))
    );
}

fn empty_iter() -> impl std::iter::Iterator<Item = i32> {
    std::iter::empty()
}

fn empty_vec() -> std::vec::Vec<i32> {
    std::vec::Vec::new()
}
