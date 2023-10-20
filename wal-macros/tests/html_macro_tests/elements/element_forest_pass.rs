use std::collections::HashMap;
use wal::virtual_dom::{VElement, VList, VNode};
use wal_macros::html;

include!("../utils/new_velement_str.rs");

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
                vec![VNode::Element(new_velement_str(
                    "span",
                    HashMap::new(),
                    Vec::new(),
                ))],
            )),
            VNode::Element(new_velement_str("div", HashMap::new(), Vec::new())),
        ]))
    )
}
