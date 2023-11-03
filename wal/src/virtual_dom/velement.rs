use itertools::{EitherOrBoth, Itertools};
use std::collections::{HashMap, HashSet};
use web_sys::{Element, Node};

use crate::{events::EventHandler, utils::debug, virtual_dom::Dom};

use super::VNode;

#[derive(Debug)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub event_handlers: Vec<EventHandler>,
    pub children: Vec<VNode>,

    pub dom: Option<Element>,
}

impl VElement {
    // TODO: maybe some types for attributes and children
    // List of attributes - https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
    //                    - https://www.w3schools.com/tags/ref_attributes.asp
    // Maybe some oop approach with defined types for attributes and types for elements?
    pub fn new(
        tag_name: String,
        attr: HashMap<String, String>,
        event_handlers: Vec<EventHandler>,
        children: Vec<VNode>,
    ) -> VElement {
        VElement {
            tag_name,
            attr,
            event_handlers,
            children,
            dom: None,
        }
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching element");
        let mut old_virt: Option<VElement> = None;

        match last {
            None => {
                debug::log("\tCreating element for the first time");
                self.dom = None;
            }
            Some(VNode::Element(mut velement)) => {
                debug::log("\tComparing two elements");
                self.dom = velement.dom.take();
                old_virt = Some(velement);
            }
            Some(VNode::Text(v)) => {
                debug::log("\tCreating element for the first time and swapping with existing text");
                self.dom = None;
                v.erase();
            }
            Some(VNode::Component(v)) => {
                debug::log(
                    "\tCreating element for the first time and swapping with existing comp node",
                );
                self.dom = None;
                v.erase();
            }
            Some(VNode::List(v)) => {
                debug::log("\tCreating element for the first time and swapping with list");
                self.dom = None;
                v.erase();
            }
        }

        self.render(old_virt.as_ref(), ancestor);
        self.handle_children(old_virt);
    }

    pub fn erase(&self) {
        if let Some(el) = &self.dom {
            Dom::remove_node(el);
        }
    }
}

impl VElement {
    /// Renders virtual Element into concrete DOM Element object. Diffs on tag name,
    /// attributes and children
    fn render(&mut self, last: Option<&VElement>, ancestor: &Node) {
        match last {
            Some(last) if last.tag_name == self.tag_name => {
                debug::log("\t\tComparing attrs");
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

                for event_handler in &mut self.event_handlers {
                    event_handler.attach(target);
                }
            }
            _ => {
                // inverted check, if last == None || last = Some(x) that x.tag_name !=
                // self.tag_name => Swap whole element
                debug::log("\t\tRendering new node");
                let el = Dom::create_element(&self.tag_name);

                // add attributes
                for (name, value) in self.attr.iter() {
                    Dom::set_attribute(&el, name, value);
                }

                for event_handler in &mut self.event_handlers {
                    event_handler.attach(&el);
                }

                match &self.dom {
                    Some(old_child) => Dom::replace_child(ancestor, old_child, &el),
                    None => Dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }

    fn handle_children(&mut self, old_element: Option<VElement>) {
        let target = self.dom.as_mut().unwrap();
        let old_children = old_element.map_or(Vec::new(), |e| e.children.into_iter().collect());

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
                    old_child.erase();
                }
            }
        }
    }
}

impl PartialEq for VElement {
    fn eq(&self, other: &Self) -> bool {
        let self_event_handlers: HashSet<_> = self.event_handlers.iter().collect();
        let other_event_handlers: HashSet<_> = other.event_handlers.iter().collect();

        self.tag_name == other.tag_name
            && self.attr == other.attr
            && self.children == other.children
            && self.dom == other.dom
            && self_event_handlers == other_event_handlers
    }
}
