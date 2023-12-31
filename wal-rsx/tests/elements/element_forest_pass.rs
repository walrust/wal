use std::collections::HashMap;
use wal_core::{
    events::EventHandler,
    virtual_dom::{VElement, VList, VNode},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    let rsx = rsx! {
        <div></div>
        <div>
            <span/>
        </div>
        <div/>
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
                    vec![VNode::Element(new_velement_str(
                        "span",
                        HashMap::new(),
                        Vec::new(),
                        None,
                        Vec::new(),
                    ))],
                )),
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new()
                )),
            ],
            None
        ))
    )
}
