use wal::virtual_dom::{VList, VNode, VText};
use wal_macros::rsx;

fn main() {
    empty();
    empty_string();
    non_empty_string();
    char();
    integer();
    float();
}

fn empty() {
    let html = rsx! {};
    assert_eq!(html, VNode::List(VList::new_empty(None)));
}

fn empty_string() {
    let html = rsx! { "" };
    assert_eq!(html, VNode::Text(VText::new("")));
}

fn non_empty_string() {
    let html = rsx! { "Hello world!" };
    assert_eq!(html, VNode::Text(VText::new("Hello world!")));
}

fn char() {
    let html = rsx! { 'a' };
    assert_eq!(html, VNode::Text(VText::new("a")));
}

fn integer() {
    let html = rsx! { 15 };
    assert_eq!(html, VNode::Text(VText::new("15")));
}

fn float() {
    let html = rsx! { 15.5 };
    assert_eq!(html, VNode::Text(VText::new("15.5")));
}
