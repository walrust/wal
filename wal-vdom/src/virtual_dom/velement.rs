use gloo::{console::log, utils::document};
use serde::Serialize;
use std::collections::HashMap;
use web_sys::Element;

use super::VNode;

#[derive(Debug, Serialize)]
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
    pub fn render(&self, target: &mut Element, last: Option<&VElement>) {
        match last {
            Some(last) if last.tag_name == self.tag_name => {
                log!("\t\tComparing attrs");
                // Compare attributes
                for (key, val) in self.attr.iter() {
                    target.set_attribute(key, val).expect("Couldnt set attribute");
                }
                for (key, _val) in last.attr.iter() {
                    if !self.attr.contains_key(key) {
                        target.remove_attribute(key).expect("Couldnt remove attribute");
                    }
                }
            },
            _ => {
                // inverted check, if last == None || last = Some(x) that x.tag_name ==
                // self.tag_name => Swap whole element
                log!("\t\tRendering new node");
                let new_el = document()
                    .create_element(&self.tag_name)
                    .expect("Couldnt create new element");

                // add attributes
                for (key, val) in self.attr.iter() {
                    new_el.set_attribute(key, val).expect("Couldnt set attribute");
                }

                target.replace_with_with_node_1(&new_el).expect("Couldnt replace whole node");
                *target = new_el;
            },
        }
    }
}
