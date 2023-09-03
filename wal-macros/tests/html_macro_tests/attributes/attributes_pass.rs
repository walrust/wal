use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VList, VNode};

fn main() {
    single_attribute();
    single_key_attribute();
    multiple_attributes();
    multiple_attributes_self_closing();
    fragment_with_key_attribute();
    single_expression_attribute();
    multiple_expression_and_literal_attributes();
}

fn single_attribute() {
    let html = html! { <div attr="value"></div> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str("div", HashMap::from([("attr", "value")]), Vec::new(),),
        }
    );
}

fn single_key_attribute() {
    let html = html! { <div key="value"></div> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str("div", HashMap::from([("key", "value")]), Vec::new(),),
        }
    );
}

fn multiple_attributes() {
    let html = html! { <div attr1="val1" attr2="val2"></div> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str(
                "div",
                HashMap::from([("attr1", "val1"), ("attr2", "val2")]),
                Vec::new(),
            ),
        }
    );
}

fn multiple_attributes_self_closing() {
    let html = html! { <div attr1="val1" attr2="val2"/> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str(
                "div",
                HashMap::from([("attr1", "val1"), ("attr2", "val2")]),
                Vec::new(),
            ),
        }
    );
}

fn fragment_with_key_attribute() {
    let html = html! { <key="value"></> };
    assert_eq!(
        html,
        VNode::List {
            vlist: VList::new_empty(),
        }
    );
}

fn single_expression_attribute() {
    let html = html! { <div attr={1 + 2}></div> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str("div", HashMap::from([("attr", "3")]), Vec::new(),),
        }
    );
}

fn multiple_expression_and_literal_attributes() {
    let html = html! { <div attr1={1 + 2} attr2="val2" attr3={"val3"}></div> };
    assert_eq!(
        html,
        VNode::Element {
            velement: new_str(
                "div",
                HashMap::from([("attr1", "3"), ("attr2", "val2"), ("attr3", "val3")]),
                Vec::new(),
            ),
        }
    );
}

fn new_str(tag_name: &str, attr: HashMap<&str, &str>, children: Vec<VNode>) -> VElement {
    VElement {
        tag_name: tag_name.to_string(),
        attr: attr
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        children,
    }
}
