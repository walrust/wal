use std::collections::HashMap;
use wal::{
    events::EventHandler,
    virtual_dom::{VElement, VNode},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    let rsx = rsx! {
        <div>
            <div>
                <input/>
                <div></div>
            </div>
            <div/>
        </div>
    };
    assert_eq!(
        rsx,
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
