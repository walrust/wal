use web_sys::{Node, Text};

use crate::utils::debug;

use super::{dom, VNode};

#[derive(PartialEq, Debug)]
pub struct VText {
    pub text: String,
    pub dom: Option<Text>,
}

impl VText {
    pub fn new<T: ToString>(text: T) -> VText {
        VText {
            text: text.to_string(),
            dom: None,
        }
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching TextNode");
        let mut old_virt: Option<VText> = None;

        match last {
            None => {
                debug::log("\tCreating text for the first time");
                self.dom = None;
            }
            Some(VNode::Text(vtext)) => {
                self.dom = vtext.dom.clone();
                old_virt = Some(vtext);
            }
            Some(VNode::Element(v)) => {
                debug::log("\tCreating text for the first time and swapping with existing element");
                self.dom = None;
                v.erase();
            }
            Some(VNode::Component(v)) => {
                debug::log(
                    "\tCreating text for the first time and swapping with existing comp node",
                );
                self.dom = None;
                v.erase();
            }
            Some(VNode::List(v)) => {
                debug::log("\tCreating text for the first time and swapping with list");
                self.dom = None;
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
    }

    pub fn erase(&self) {
        if let Some(text) = &self.dom {
            dom::remove_node(text);
        }
    }
}

impl VText {
    /// Renders virtual text node over concrete DOM Text object. If the last VText
    /// isnt None and text value is the same, function does nothing
    fn render(&mut self, last: Option<VText>, ancestor: &Node) {
        match last {
            // Different value => just change node value
            Some(last) if self.text != last.text => {
                self.dom
                    .as_ref()
                    .expect("Dom is not created even though it should have been")
                    .set_node_value(Some(self.text.as_str()));
            }
            // Same thing => do nothing
            Some(_) => (),
            None => {
                let el = dom::create_text_node(&self.text);

                match &self.dom {
                    Some(old_child) => dom::replace_child(ancestor, old_child, &el),
                    None => dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }
}
