use std::collections::HashMap;

use gloo::{utils::document, console::__macro::JsValue};
use serde::Serialize;
use web_sys::{Element, Node};

#[derive(Serialize)]
pub struct VNode<'a> {
    pub tag_name: &'a str,
    pub attr: HashMap<&'a str, &'a str>,
    pub children: Vec<VNode<'a>>,
}

impl<'a> VNode<'a> {
    pub fn new(
        tag_name: &'a str,
        attr: HashMap<&'a str, &'a str>,
        children: Vec<VNode<'a>>,
    ) -> VNode<'a> {
        VNode {
            tag_name,
            attr,
            children,
        }
    }
}

pub fn render(vnode: VNode) -> Result<Node, JsValue> {
    let el = document()
        .create_element(vnode.tag_name)?;

    // set attributes
    for (key, val) in vnode.attr.iter() {
        el.set_attribute(key, val)?;
    }
    // appending children
    for v_child in vnode.children {
        let child = render(v_child)?;
        el.append_child(&child)?;
    }

    Ok(Node::from(el))
}

pub fn mount(node: Node, target: Element) -> Result<Node, JsValue> {
    target.replace_with_with_node_1(&node)?;
    Ok(node) 
}
