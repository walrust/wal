use std::collections::HashMap;
use wal::{
    events::EventHandler,
    virtual_dom::{VElement, VNode},
};
use wal_macros::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    let html = rsx! {
        <div>
            <div>
                <input/>
                <div></div>
            </div>
            <div/>
        </div>
    };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            Vec::new(),
            None,
            vec![
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    vec![
                        VNode::Element(new_velement_str(
                            "input",
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
                        )),
                    ],
                )),
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    Vec::new()
                )),
            ],
        ))
    )
}
