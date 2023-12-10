use wal::virtual_dom::{VList, VNode, VText};
use wal_rsx::rsx;

fn main() {
    empty();
    empty_string();
    non_empty_string();
    char();
    integer();
    float();
}

fn empty() {
    let rsx = rsx! {};
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn empty_string() {
    let rsx = rsx! { "" };
    assert_eq!(rsx, VNode::Text(VText::new("")));
}

fn non_empty_string() {
    let rsx = rsx! { "Hello world!" };
    assert_eq!(rsx, VNode::Text(VText::new("Hello world!")));
}

fn char() {
    let rsx = rsx! { 'a' };
    assert_eq!(rsx, VNode::Text(VText::new("a")));
}

fn integer() {
    let rsx = rsx! { 15 };
    assert_eq!(rsx, VNode::Text(VText::new("15")));
}

fn float() {
    let rsx = rsx! { 15.5 };
    assert_eq!(rsx, VNode::Text(VText::new("15.5")));
}
