use gloo::console::log;
use web_sys::{Node, Text};

use super::{Dom, VNode};

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

    pub fn patch(&mut self, mut last: Option<&VNode>, ancestor: &Node) {
        log!("Patching TextNode");
        let mut old_virt: Option<&VText> = None;
        
        match last {
            None => {
                log!("\tCreating the node for the first time");
                self.dom = None;
            }
            Some(VNode::Text { ref vtext }) => {
                self.dom = vtext.dom.clone();
                old_virt = Some(vtext);
            }
            Some(VNode::Element { .. }) | Some(VNode::Component { .. }) => {
                log!("\tCreating the node for the first time and swapping with existing text/comp node");
                self.dom = None;
            }
            Some(VNode::List { .. }) => todo!(),
        }

        self.render(old_virt, ancestor);
    }
}

impl VText {
    /// Renders virtual text node over concrete DOM Text object. If the last VText
    /// isnt None and text value is the same, function does nothing
    fn render(&mut self, last: Option<&VText>, ancestor: &Node) {
        match last {
            // Different value => just change node value
            Some(last) if self.text != last.text => {
                self.dom
                    .as_ref()
                    .expect("Dom isnt craeted even tho it should have been")
                    .set_node_value(Some(self.text.as_str()));
            }
            // Same thing => do nothing
            Some(_) => (),
            None => {
                let el = Dom::create_text_node(&self.text);

                match &self.dom {
                    Some(old_child) => Dom::replace_child(ancestor, &old_child, &el),
                    None => Dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }
}
