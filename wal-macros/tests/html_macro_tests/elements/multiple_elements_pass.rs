use std::collections::HashMap;
use wal::virtual_dom::{VElement, VList, VNode};
use wal_macros::html;

fn main() {
    let html = html! {
        <div></div>
        <span/>
        <input/>
    };
    assert_eq!(
        html,
        VNode::List(VList::new(vec![
            VNode::Element(VElement::new("div".to_string(), HashMap::new(), Vec::new())),
            VNode::Element(VElement::new(
                "span".to_string(),
                HashMap::new(),
                Vec::new()
            )),
            VNode::Element(VElement::new(
                "input".to_string(),
                HashMap::new(),
                Vec::new()
            )),
        ]))
    )
}
