use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VList, VNode};

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
        VNode::List {
            vlist: VList::new(vec![
                VNode::Element {
                    velement: VElement::new_str("div", HashMap::new(), Vec::new()),
                },
                VNode::Element {
                    velement: VElement::new_str(
                        "div",
                        HashMap::new(),
                        vec![VNode::Element {
                            velement: VElement::new_str("span", HashMap::new(), Vec::new()),
                        }],
                    ),
                },
                VNode::Element {
                    velement: VElement::new_str("div", HashMap::new(), Vec::new()),
                },
            ]),
        }
    )
}
