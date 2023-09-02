use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VNode};

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
        VNode::Element {
            velement: VElement::new_str(
                "div",
                HashMap::new(),
                vec![
                    VNode::Element {
                        velement: VElement::new_str(
                            "div",
                            HashMap::new(),
                            vec![
                                VNode::Element {
                                    velement: VElement::new_str(
                                        "input",
                                        HashMap::new(),
                                        Vec::new()
                                    ),
                                },
                                VNode::Element {
                                    velement: VElement::new_str("div", HashMap::new(), Vec::new()),
                                },
                            ],
                        ),
                    },
                    VNode::Element {
                        velement: VElement::new_str("div", HashMap::new(), Vec::new()),
                    },
                ],
            ),
        }
    )
}
