use gloo::console::log;
use std::collections::HashMap;
use web_sys::{Element, Node};

use crate::virtual_dom::Dom;

use super::VNode;

#[derive(Debug, PartialEq)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<VNode>,

    pub dom: Option<Element>,
}

impl VElement {
    // TODO: maybe some types for attributes and children
    // List of attributes - https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
    //                    - https://www.w3schools.com/tags/ref_attributes.asp
    // Maybe some oop approach with defined types for attributes and types for elements?
    pub fn new(tag_name: String, attr: HashMap<String, String>, children: Vec<VNode>) -> VElement {
        VElement {
            tag_name,
            attr,
            children,
            dom: None,
        }
    }

    pub fn patch(&mut self, last: Option<&VNode>, ancestor: &Node) {
        log!("Patching element");
        let mut old_virt: Option<&VElement> = None;

        match last {
            None => {
                log!("\tCreating the node for the first time");
                self.dom = None;
            }
            Some(VNode::Element { velement }) => {
                log!("\tCopying existing node");
                self.dom = match velement.dom.clone() {
                    Some(v) => Some(v),
                    None => None,
                };
                old_virt = Some(velement);
            }
            Some(VNode::Text { .. }) | Some(VNode::Component { .. }) => {
                log!("\tCreating the node for the first time and swapping with existing text/comp node");
                self.dom = None;
            }
            Some(VNode::List { .. }) => todo!(),
        }

        self.render(old_virt, ancestor);
        self.handle_children(old_virt);
    }
}

impl VElement {
    /// Renders virtual Element into concrete DOM Element object. Diffs on tag name,
    /// attributes and children
    fn render(&mut self, last: Option<&VElement>, ancestor: &Node) {
        match last {
            Some(last) if last.tag_name == self.tag_name => {
                log!("\t\tComparing attrs");
                let target = self
                    .dom
                    .as_mut()
                    .expect("Target dom object not created before rendering element");
                // Compare attributes
                for (key, val) in self.attr.iter() {
                    Dom::set_attribute(target, key, val);
                }
                for (key, _val) in last.attr.iter() {
                    if !self.attr.contains_key(key) {
                        Dom::remove_attribute(target, key);
                    }
                }
            }
            _ => {
                // inverted check, if last == None || last = Some(x) that x.tag_name !=
                // self.tag_name => Swap whole element
                log!("\t\tRendering new node");
                let el = Dom::create_element(&self.tag_name);

                // add attributes
                for (name, value) in self.attr.iter() {
                    Dom::set_attribute(&el, name, value);
                }

                match &self.dom {
                    Some(old_child) => Dom::replace_child(ancestor, old_child, &el),
                    None => Dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }

    fn handle_children(&mut self, old_virt: Option<&VElement>) {
        let target = self.dom.as_mut().unwrap();

        let mut children: Vec<Option<&mut VNode>> =
            self.children.iter_mut().map(|x| Some(x)).collect();
        let mut old_children: Vec<Option<&VNode>> = match old_virt {
            Some(el) => el.children.iter().map(Some).collect(),
            None => Vec::new(),
        };

        // More elegant and rust-style like approach
        let len_diff = children.len() as i64 - old_children.len() as i64;

        if len_diff < 0 {
            let appendix = (0..len_diff.abs()).map(|_| None);
            children.append(&mut appendix.collect());
        } else if len_diff > 0 {
            let appendix = (0..len_diff.abs()).map(|_| None);
            old_children.append(&mut appendix.collect());
        }

        for pair in children.into_iter().zip(old_children) {
            match pair {
                (None, Some(node)) => {
                    // child doesnt exist anymore
                    if let Some(node) = node.get_dom() {
                        Dom::remove_child(&target, &node);
                    }
                }
                (Some(node), old) => {
                    //patch child
                    node.patch(old, target);
                }
                (None, None) => {
                    log!("Impossible redundant loop");
                    panic!("Impossible redundant loop");
                }
            }
        }
    }
}
