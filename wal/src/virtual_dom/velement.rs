use gloo::{console::log, events::EventListener};
use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;
use web_sys::{Element, Node};

use crate::{events::EventHandler, virtual_dom::Dom};

use super::VNode;

#[derive(Debug)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<VNode>,
    pub event_handlers: Vec<(Box<dyn EventHandler>, Option<EventListener>)>,

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
        children: Vec<VNode>,
        event_handlers: Vec<Box<dyn EventHandler>>,
    ) -> VElement {
        VElement {
            tag_name,
            attr,
            children,
            event_handlers: event_handlers
                .into_iter()
                .map(|event_handler| (event_handler, None))
                .collect(),
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
            Some(VNode::Element(velement)) => {
                log!("\tCopying existing node");
                self.dom = velement.dom.clone();
                old_virt = Some(velement);
            }
            Some(VNode::Text(_)) | Some(VNode::Component(_)) => {
                log!("\tCreating the node for the first time and swapping with existing text/comp node");
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
                for event_handler in &mut self.event_handlers {
                    event_handler.1 = Some(Dom::create_event_listener(target, &event_handler.0));
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

                for event_handler in &mut self.event_handlers {
                    event_handler.1 = Some(Dom::create_event_listener(&el, &event_handler.0));
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
        let target = self.dom.as_mut().unwrap();
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

impl PartialEq for VElement {
    fn eq(&self, other: &Self) -> bool {
        self.tag_name == other.tag_name
            && self.attr == other.attr
            && self.children == other.children
            && self.dom == other.dom
    }
}
