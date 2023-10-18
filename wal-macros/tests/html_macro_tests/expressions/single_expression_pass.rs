use wal_macros::html;
use wal_vdom::virtual_dom::{VNode, VText};

include!("../utils/display_struct.rs");

fn main() {
    isolated_expression();
    referance_expression();
    displayable_struct_expression();
    function_returning_value();
    function_returning_html();
}

fn isolated_expression() {
    let html = html! { String::from("Hello world!") };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn referance_expression() {
    let val = "Hello world!";
    let html = html! { val };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn displayable_struct_expression() {
    let html = html! { DisplayStruct };
    assert_eq!(html, VNode::Text(VText::new(DisplayStruct)));
}

fn expression_block() {
    let html = html! { { let s = "Hello world!"; String::from(s) } };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn function_returning_value() {
    let node = || 5;
    let html = html! { node() };
    assert_eq!(html, VNode::Text(VText::new(5)));
}

fn function_returning_html() {
    let node = || html! { "Hello world!" };
    let html = html! { node() };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}
