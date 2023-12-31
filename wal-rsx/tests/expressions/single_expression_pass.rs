use wal_core::virtual_dom::{VNode, VText};
use wal_rsx::rsx;

include!("../utils/display_struct.rs");

fn main() {
    isolated_expression();
    referance_expression();
    displayable_struct_expression();
    function_returning_value();
    function_returning_rsx();
}

fn isolated_expression() {
    let rsx = rsx! { String::from("Hello world!") };
    assert_eq!(rsx, VNode::Text(VText::new("Hello world!")));
}

fn referance_expression() {
    let val = "Hello world!";
    let rsx = rsx! { val };
    assert_eq!(rsx, VNode::Text(VText::new("Hello world!")));
}

fn displayable_struct_expression() {
    let rsx = rsx! { DisplayStruct };
    assert_eq!(rsx, VNode::Text(VText::new(DisplayStruct)));
}

fn expression_block() {
    let rsx = rsx! { { let s = "Hello world!"; String::from(s) } };
    assert_eq!(rsx, VNode::Text(VText::new("Hello world!")));
}

fn function_returning_value() {
    let node = || 5;
    let rsx = rsx! { node() };
    assert_eq!(rsx, VNode::Text(VText::new(5)));
}

fn function_returning_rsx() {
    let node = || rsx! { "Hello world!" };
    let rsx = rsx! { node() };
    assert_eq!(rsx, VNode::Text(VText::new("Hello world!")));
}
