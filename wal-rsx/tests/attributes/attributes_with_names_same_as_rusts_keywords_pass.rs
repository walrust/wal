use std::collections::HashMap;
use wal_core::{
    events::EventHandler,
    virtual_dom::{VElement, VNode},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    let rsx = rsx! {
        <input type="checkbox" />
    };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "input",
            HashMap::from([("type", "checkbox")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}
