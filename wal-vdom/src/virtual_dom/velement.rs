use gloo::{console::log, utils::document};
use serde::Serialize;
use std::collections::HashMap;
use web_sys::Element;

use super::VNode;

#[derive(Serialize)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<VNode>,
}

impl VElement {
    // TODO: maybe some types for attributes and children
    pub fn new(tag_name: String, attr: HashMap<String, String>, children: Vec<VNode>) -> VElement {
        VElement {
            tag_name,
            attr,
            children,
        }
    }

    // TODO: Implement it
    /// Renders virtual Element into concrete DOM Element object. Diffs on tag name,
    /// attributes and children
    pub fn render(&self, target: &Element, last: Option<VElement>) {
        log!("TODO: implement velement rendering");
        let test = document().create_element("div").unwrap();
        test.set_attribute("test", "test");
        target.replace_with_with_node_1(&test).expect("Why doesnt test work wtf");
    }
}
