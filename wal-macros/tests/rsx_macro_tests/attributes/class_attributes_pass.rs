use std::collections::HashMap;
use wal::{
    events::EventHandler,
    virtual_dom::{VElement, VNode},
};
use wal_macros::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    class_attribute();
    wal_class_attribute_with_single_value();
    wal_class_attribute_with_multiple_values();
    class_attribute_and_wal_class_attribute_with_single_value();
    class_attribute_and_wal_class_attribute_with_multiple_values();
    class_attributes_with_other_attributes();
}

fn class_attribute() {
    let rsx = rsx! { <div class="class1"></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([("class", "class1")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn wal_class_attribute_with_single_value() {
    let rsx = rsx! { <div wal_class=[String::from("class1")]></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([("class", "class1")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn wal_class_attribute_with_multiple_values() {
    let rsx = rsx! { <div wal_class=[String::from("class1"), "class2", 1]></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([("class", "class1 class2 1")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn class_attribute_and_wal_class_attribute_with_single_value() {
    let rsx = rsx! { <div class="class1" wal_class=[String::from("class2")]></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([("class", "class1 class2")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn class_attribute_and_wal_class_attribute_with_multiple_values() {
    let rsx = rsx! { <div class="class1" wal_class=[String::from("class2"), "class3", 1]></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([("class", "class1 class2 class3 1")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn class_attributes_with_other_attributes() {
    let rsx = rsx! { <div class="class1" attr1="val1" wal_class=[String::from("class2"), "class3", 1] attr2="val2"></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::from([
                ("attr1", "val1"),
                ("attr2", "val2"),
                ("class", "class1 class2 class3 1"),
            ]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}