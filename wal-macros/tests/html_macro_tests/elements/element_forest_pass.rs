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
                    velement: new_str("div", HashMap::new(), Vec::new()),
                },
                VNode::Element {
                    velement: new_str(
                        "div",
                        HashMap::new(),
                        vec![VNode::Element {
                            velement: new_str("span", HashMap::new(), Vec::new()),
                        }],
                    ),
                },
                VNode::Element {
                    velement: new_str("div", HashMap::new(), Vec::new()),
                },
            ]),
        }
    )
}

fn new_str(tag_name: &str, attr: HashMap<&str, &str>, children: Vec<VNode>) -> VElement {
    VElement::new(
        tag_name.to_string(),
        attr.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        children,
    )
}
