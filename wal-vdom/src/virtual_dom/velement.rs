use gloo::{console::__macro::JsValue, utils::document};
use serde::Serialize;
use std::collections::HashMap;
use web_sys::Node;

use super::VNode;

#[derive(Serialize)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<VNode>,
}

impl VElement {
    pub fn new(tag_name: String, attr: HashMap<String, String>, children: Vec<VNode>) -> VElement {
        VElement {
            tag_name,
            attr,
            children,
        }
    }

    pub fn new_attrs_as_vecs(
        tag_name: &str,
        attr_keys: Vec<&str>,
        attr_values: Vec<&str>,
        children: Vec<VNode>,
    ) -> VElement {
        let tag_name = tag_name.to_string();
        let attr = attr_keys
            .into_iter()
            .map(|key| key.to_string())
            .zip(attr_values.into_iter().map(|val| val.to_string()))
            .collect();
        Self::new(tag_name, attr, children)
    }

    pub fn render(&self) -> Result<Node, JsValue> {
        let el = document().create_element(self.tag_name.as_str())?;

        // set attributes
        for (key, val) in self.attr.iter() {
            el.set_attribute(key, val)?;
        }
        // appending children
        for v_child in self.children.iter() {
            let child = v_child.render()?;
            el.append_child(&child)?;
        }

        Ok(el.into())
    }
}
