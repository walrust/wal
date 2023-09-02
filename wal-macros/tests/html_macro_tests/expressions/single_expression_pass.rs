use wal_macros::html;
use wal_vdom::virtual_dom::{VNode, VText};

struct TestDisplayStruct {
    field: i32,
}

impl std::fmt::Display for TestDisplayStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestStruct {{ field: {} }}", self.field)
    }
}

fn main() {
    isolated_expression();
    referance_expression();
    displayable_struct_expression();
    function_returning_value();
    function_returning_html();
}

fn isolated_expression() {
    let html = html! { String::from("Hello world!") };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("Hello world!")
        }
    );
}

fn referance_expression() {
    let val = "Hello world!";
    let html = html! { val };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("Hello world!")
        }
    );
}

fn displayable_struct_expression() {
    let html = html! { TestDisplayStruct { field: 15 } };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new(TestDisplayStruct { field: 15 })
        }
    );
}

fn expression_block() {
    let html = html! { { let s = "Hello world!"; String::from(s) } };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("Hello world!")
        }
    );
}

fn function_returning_value() {
    let node = || 5;
    let html = html! { node() };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new(5)
        }
    );
}

fn function_returning_html() {
    let node = || html! { "Hello world!" };
    let html = html! { node() };
    assert_eq!(
        html,
        VNode::Text {
            vtext: VText::new("Hello world!")
        }
    );
}
