use std::collections::HashMap;
use wal::{
    events::EventHandler,
    virtual_dom::{VElement, VNode, VText},
};
use wal_macros::rsx;

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
    let rsx = rsx! { <Link to="/home" /> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn non_self_closing_link_with_to_attr_as_string() {
    let rsx = rsx! { <Link to="/home"></Link> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn link_with_to_attr_as_expr() {
    let link = "/home";
    let rsx = rsx! { <Link to={link}></Link> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", link), ("data_link", link)]),
            Vec::new(),
            None,
            Vec::new(),
        ))
    );
}

fn link_with_key_attr() {
    let rsx = rsx! { <Link key="value" to="/home"></Link> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            Some("value"),
            Vec::new(),
        ))
    );
}

fn link_with_single_child() {
    let rsx = rsx! { <Link to="/home">"Home"</Link> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            None,
            vec![VNode::Text(VText::new("Home"))],
        ))
    );
}

fn link_with_multiple_children() {
    let rsx = rsx! {
        <Link to="/home">
            "Home"
            <span>"Span"</span>
        </Link>
    };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "a",
            HashMap::from([("href", "/home"), ("data_link", "/home")]),
            Vec::new(),
            None,
            vec![
                VNode::Text(VText::new("Home")),
                VNode::Element(new_velement_str(
                    "span",
                    HashMap::new(),
                    Vec::new(),
                    None,
                    vec![VNode::Text(VText::new("Span"))],
                ))
            ],
        ))
    );
}
