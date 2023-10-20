use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;
use web_sys::{Element, Node};

use crate::{virtual_dom::Dom, utils::{WasmUtils, debug_log}};

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
        debug_log("Patching element");
        let mut old_virt: Option<&VElement> = None;

        match last {
            None => {
                debug_log("\tCreating the node for the first time");
                self.dom = None;
            }
            Some(VNode::Element(velement)) => {
                debug_log("\tCopying existing node");
                self.dom = velement.dom.clone();
                old_virt = Some(velement);
            }
            Some(VNode::Text(_)) | Some(VNode::Component(_)) => {
                debug_log("\tCreating the node for the first time and swapping with existing text/comp node");
                self.dom = None;
            }
            Some(VNode::List(_)) => todo!(),
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
                debug_log("\t\tComparing attrs");
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
                debug_log("\t\tRendering new node");
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

    fn handle_children(&mut self, old_element: Option<&VElement>) {
        let target = self.dom.as_mut().wasm_unwrap();
        let old_children = old_element.map_or(Vec::new(), |e| e.children.iter().collect());

        for either_child_or_both in self.children.iter_mut().zip_longest(old_children) {
            match either_child_or_both {
                EitherOrBoth::Both(child, old_child) => {
                    child.patch(Some(old_child), target);
                }
                EitherOrBoth::Left(child) => {
                    child.patch(None, target);
                }
                EitherOrBoth::Right(old_child) => {
                    // child doesnt exist anymore
                    if let Some(node) = old_child.get_dom() {
                        Dom::remove_child(&target, &node);
                    }
                }
            }
        }
    }
}
