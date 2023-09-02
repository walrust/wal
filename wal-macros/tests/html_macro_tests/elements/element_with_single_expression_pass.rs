use std::collections::HashMap;
use wal_macros::html;
use wal_vdom::virtual_dom::{VElement, VNode, VText};

struct TestDisplayStruct {
    field: i32,
}

impl std::fmt::Display for TestDisplayStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestStruct {{ field: {} }}", self.field)
    }
}

fn main() {
    element_with_isolated_expression();
    element_with_referance_expression();
    element_with_struct_expression();
    element_with_function_returning_value();
    element_with_function_returning_html();
}

fn element_with_isolated_expression() {
    let html = html! { <div> { String::from("Hello world!") } </div> };
    assert_eq!(html, get_div_with("Hello world!"));
}

fn element_with_referance_expression() {
    let val = "Hello world!";
    let html = html! { <div> { val } </div> };
    assert_eq!(html, get_div_with("Hello world!"));
}

fn element_with_struct_expression() {
    let html = html! { <div> { TestDisplayStruct { field: 15 } } </div> };
    assert_eq!(html, get_div_with(TestDisplayStruct { field: 15 }));
}

fn element_with_function_returning_value() {
    let node = || 5;
    let html = html! { <div> { node() } </div> };
    assert_eq!(html, get_div_with(5));
}

fn element_with_function_returning_html() {
    let node = || html! { "Hello world!" };
    let html = html! { <div> { node() } </div> };
    assert_eq!(html, get_div_with("Hello world!"));
}

fn get_div_with<T: ToString>(t: T) -> VNode {
    VNode::Element {
        velement: VElement::new(
            "div".to_string(),
            HashMap::new(),
            vec![VNode::Text {
                vtext: VText::new(t.to_string()),
            }],
        ),
    }
}
