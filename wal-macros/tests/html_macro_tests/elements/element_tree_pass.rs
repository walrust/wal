use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode};
use wal_macros::html;

include!("../utils/new_velement_str.rs");

fn main() {
    let html = html! {
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
            vec![
                VNode::Element(new_velement_str(
                    "div",
                    HashMap::new(),
                    vec![
                        VNode::Element(new_velement_str("input", HashMap::new(), Vec::new())),
                        VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
                    ],
                )),
                VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
            ],
        ))
    )
}
