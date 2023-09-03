use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VList, VNode};

include!("../utils/new_element_str.rs");

fn main() {
    empty();
    with_single_element();
    with_multiple_elements();
    inside_element();
    with_key_attribute();
    with_vec_expression();

    html! {
        <>
            <div></div>
            <div></div>
        </>
    };
    html! {
        <div>
            <></>
        </div>
    }
    html! { <key="value"></> };

    let children = vec![1, 2];
    html! { <> { children } </> };
}

fn empty() {
    let html = html! { <></> };
    assert_eq!(
        html,
        VNode::List {
            vlist: VList::new_empty()
        }
    );
}

fn with_single_element() {
    let html = html! {
        <>
            <div></div>
        </>
    };
    assert_eq!(
        html,
        VNode::List {
            vlist: VList::new(vec![VNode::Element {
                velement: new_velement_str("div", HashMap::new(), Vec::new()),
            }]),
        }
    );
}
