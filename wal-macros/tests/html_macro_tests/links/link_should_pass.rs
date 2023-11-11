use std::collections::HashMap;
use wal::{
    events::EventHandler,
    virtual_dom::{VElement, VNode, VText},
};
use wal_macros::html;

include!("../utils/new_velement_str.rs");

fn main() {
    link_with_to_attr_as_string();
    non_self_closing_link_with_to_attr_as_string();
    link_with_to_attr_as_expr();
    link_with_key_attr();
    link_with_single_child();
    link_with_multiple_children();
}

fn link_with_to_attr_as_string() {
    let html = html! { <Link to="/home" /> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            Vec::new(),
        ))
    );
}

fn non_self_closing_link_with_to_attr_as_string() {
    let html = html! { <Link to="/home"></Link> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            Vec::new(),
        ))
    );
}

fn link_with_to_attr_as_expr() {
    let link = "/home";
    let html = html! { <Link to={link}></Link> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", link), ("data_link", link)]),
            Vec::new(),
            Vec::new(),
        ))
    );
}

fn link_with_key_attr() {
    let html = html! { <Link key="value" to="/home"></Link> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home"), ("key", "value")]),
            Vec::new(),
            Vec::new(),
        ))
    );
}
fn link_with_single_child() {
    let html = html! { <Link to="/home">"Home"</Link> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            vec![VNode::Text(VText::new("Home"))],
        ))
    );
}

fn link_with_multiple_children() {
    let html = html! {
        <Link to="/home">
            "Home"
            <span>"Span"</span>
        </Link>
    };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            vec![
                VNode::Text(VText::new("Home")),
                VNode::Element(new_velement_str(
                    "span",
                    HashMap::new(),
                    Vec::new(),
                    vec![VNode::Text(VText::new("Span"))],
                ))
            ],
        ))
    );
}
