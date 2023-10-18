use wal_macros::html;
use wal_vdom::virtual_dom::{VList, VNode, VText};

fn main() {
    if_let_true();
    if_let_false();
    if_let_true_using_value_from_let();
    if_let_true_else();
    if_let_false_else();
    if_let_true_else_if_let();
    if_let_false_else_if_let_true();
    if_let_else_if_lets();
    if_let_else_if_lets_else();
    if_let_false_else_if_lets_false_else();
}

fn if_let_true() {
    let html = html! {
        if let Some(_val) = Some("hello") { "hello" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello")));
}

fn if_let_false() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
    };
    assert_eq!(html, VNode::List(VList::new_empty()));
}

fn if_let_true_using_value_from_let() {
    let html = html! {
        if let Some(val) = Some("hello") { val }
    };
    assert_eq!(html, VNode::Text(VText::new("hello")));
}

fn if_let_true_else() {
    let html = html! {
        if let Some(_val) = Some("hello") { "hello" }
        else { "hello2" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello")));
}

fn if_let_false_else() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
        else { "hello2" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello2")));
}

fn if_let_true_else_if_let() {
    let html = html! {
        if let Some(_val) = Some("hello") { "hello" }
        else if let Some(..) = None::<i32> { "hello2" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello")));
}

fn if_let_false_else_if_let_true() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
        else if let Some(_val) = Some("hello2") { "hello2" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello2")));
}

fn if_let_else_if_lets() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
        else if let Some(_val) = Some("hello2") { "hello2" }
        else if let Some(..) = None::<i32> { "hello3" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello2")));
}

fn if_let_else_if_lets_else() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
        else if let Some(_val) = Some("hello2") { "hello2" }
        else if let Some(..) = None::<i32> { "hello3" }
        else { "hello4" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello2")));
}

fn if_let_false_else_if_lets_false_else() {
    let html = html! {
        if let Some(_val) = None::<i32> { "hello" }
        else if let Some(_val) = None::<i32> { "hello2" }
        else if let Some(..) = None::<i32> { "hello3" }
        else { "hello4" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello4")));
}
