use wal::virtual_dom::{VNode, VText};
use wal_macros::rsx;

include!("../utils/display_struct.rs");

fn main() {
    isolated_expression();
    referance_expression();
    displayable_struct_expression();
    function_returning_value();
    function_returning_html();
}

fn isolated_expression() {
    let html = rsx! { String::from("Hello world!") };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn referance_expression() {
    let val = "Hello world!";
    let html = rsx! { val };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn displayable_struct_expression() {
    let html = rsx! { DisplayStruct };
    assert_eq!(html, VNode::Text(VText::new(DisplayStruct)));
}

fn expression_block() {
    let html = rsx! { { let s = "Hello world!"; String::from(s) } };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn function_returning_value() {
    let node = || 5;
    let html = rsx! { node() };
    assert_eq!(html, VNode::Text(VText::new(5)));
}

fn function_returning_html() {
    let node = || rsx! { "Hello world!" };
    let html = rsx! { node() };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}
