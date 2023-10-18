use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VList, VNode};

include!("../utils/new_element_str.rs");

fn main() {
    let html = html! {
        <div></div>
        <div>
            <span/>
        </div>
        <div/>
    };

    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
            VNode::Element(new_velement_str(
                "div",
                HashMap::new(),
                vec![VNode::Element {
                    velement: new_velement_str("span", HashMap::new(), Vec::new()),
                }],
            )),
            VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
        ]))
    )
}
