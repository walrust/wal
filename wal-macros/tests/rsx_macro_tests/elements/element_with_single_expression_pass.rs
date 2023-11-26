use std::collections::HashMap;
use wal::virtual_dom::{VElement, VNode, VText};
use wal_macros::rsx;

include!("../utils/display_struct.rs");

fn main() {
    element_with_isolated_expression();
    element_with_referance_expression();
    element_with_struct_expression();
    element_with_function_returning_value();
    element_with_function_returning_rsx();
}

fn element_with_isolated_expression() {
    let rsx = rsx! { <div> { String::from("Hello world!") } </div> };
    assert_eq!(rsx, get_div_with("Hello world!"));
}

fn element_with_referance_expression() {
    let val = "Hello world!";
    let rsx = rsx! { <div> { val } </div> };
    assert_eq!(rsx, get_div_with("Hello world!"));
}

fn element_with_struct_expression() {
    let rsx = rsx! { <div> { DisplayStruct } </div> };
    assert_eq!(rsx, get_div_with(DisplayStruct));
}

fn element_with_function_returning_value() {
    let node = || 5;
    let rsx = rsx! { <div> { node() } </div> };
    assert_eq!(rsx, get_div_with(5));
}

fn element_with_function_returning_rsx() {
    let node = || rsx! { "Hello world!" };
    let rsx = rsx! { <div> { node() } </div> };
    assert_eq!(rsx, get_div_with("Hello world!"));
}

fn get_div_with<T: ToString>(t: T) -> VNode {
    VNode::Element(VElement::new(
        "div".to_string(),
        HashMap::new(),
        Vec::new(),
        None,
        vec![VNode::Text(VText::new(t.to_string()))],
    ))
}
