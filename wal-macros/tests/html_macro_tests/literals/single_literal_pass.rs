use wal_macros::html;
use wal_vdom::virtual_dom::{VList, VNode, VText};

fn main() {
    empty();
    empty_string();
    non_empty_string();
    char();
    integer();
    float();
}

fn empty() {
    let html = html! {};
    assert_eq!(
        html,
        VNode::List {
            vlist: VList::new_empty()
        }
    );
}

fn empty_string() {
    let html = html! { "" };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("")
        }
    );
}

fn non_empty_string() {
    let html = html! { "Hello world!" };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("Hello world!")
        }
    );
}

fn char() {
    let html = html! { 'a' };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("a")
        }
    );
}

fn integer() {
    let html = html! { 15 };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("15")
        }
    );
}

fn float() {
    let html = html! { 15.5 };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("15.5")
        }
    );
}
