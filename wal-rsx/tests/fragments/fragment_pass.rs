use std::collections::HashMap;
use wal_core::{
    events::EventHandler,
    virtual_dom::{VElement, VList, VNode},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    empty();
    with_single_element();
    with_multiple_elements();
    inside_element();
    with_key_attribute();
}

fn empty() {
    let rsx = rsx! { <></> };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn with_single_element() {
    let rsx = rsx! {
        <>
            <div></div>
        </>
    };
    assert_eq!(
        rsx,
        VNode::List(VList::new(
            vec![VNode::Element(new_velement_str(
                "div",
                HashMap::new(),
                Vec::new(),
                None,
                Vec::new()
            ))],
            None
        ),)
    );
}

fn with_multiple_elements() {
    let rsx = rsx! {
        <>
            <div></div>
            <div></div>
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
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new()
                ),),
            ],
            None
        ))
    );
}

fn inside_element() {
    let rsx = rsx! {
        <div>
            <></>
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
        ))
    );
}

fn with_key_attribute() {
    let rsx = rsx! { <key="value"></> };
    assert_eq!(
        rsx,
        VNode::List(VList::new_empty(Some("value".to_string())))
    );
}
