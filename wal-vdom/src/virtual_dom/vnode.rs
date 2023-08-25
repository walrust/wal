use gloo::utils::document;
use serde::Serialize;
use web_sys::{Element, Node, Text};

use super::{VElement, VText};

#[derive(Serialize)]
pub enum VNode {
    Element {
        #[serde(skip_serializing)]
        concrete: Option<Element>,
        virt: VElement,
    },
    Text {
        #[serde(skip_serializing)]
        concrete: Option<Text>,
        virt: VText,
    },
}

impl VNode {
    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        match self {
            VNode::Element {
                ref mut concrete,
                ref mut virt,
            } => {
                let mut old_virt: Option<VElement> = None;
                match last {
                    // First creating the node
                    Some(VNode::Element { concrete: None, .. })
                    | Some(VNode::Text { concrete: None, .. })
                    | None => {
                        let new_el = document()
                            .create_element(&virt.tag_name)
                            .expect("Couldnt create new element");
                        ancestor
                            .append_child(&new_el)
                            .expect("Couldnt append child to node");
                        *concrete = Some(new_el);
                    }

                    // Just copy
                    Some(VNode::Element {
                        concrete: Some(el),
                        virt,
                    }) => {
                        *concrete = Some(el);
                        old_virt = Some(virt);
                    }

                    // Swap nodes
                    Some(VNode::Text {
                        concrete: Some(text),
                        ..
                    }) => {
                        let new_el = document()
                            .create_element(&virt.tag_name)
                            .expect("Couldnt create new element");
                        ancestor
                            .replace_child(&text, &new_el)
                            .expect("Couldnt replace child with a new node");
                        *concrete = Some(new_el);
                    }
                };

                // Render over concrete new element
                let target = concrete.as_ref().expect("It shouldnt be none");
                virt.render(target, old_virt);

                // Handle children
                let mut children = &virt.children;

                match old_virt {
                    // Editing already existing node
                    Some(ref mut old_virt) => {
                        let mut old_children = &old_virt.children;
                        let min_len = children.len().min(old_children.len());
                        let mut i = 0;

                        // patch first min_len children
                        while i < min_len {
                            children[i].patch(Some(old_children[i]), target);
                            i += 1;
                        }

                        if min_len == children.len() {
                            // remove unnecessary children
                            for child in old_children[min_len..].iter() {
                                let node = match child {
                                    VNode::Element { concrete, virt: _ } => {
                                        concrete.map(|x| Node::from(x))
                                    }
                                    VNode::Text { concrete, virt: _ } => {
                                        concrete.map(|x| Node::from(x))
                                    }
                                };
                                if let Some(node) = node {
                                    target
                                        .remove_child(&node)
                                        .expect("Couldnt remove child from node");
                                }
                            }
                        } else {
                            // add new children
                            for child in children[min_len..].iter() {
                                child.patch(None, target);
                            }
                        }
                    }
                    // Create the node and all of the children
                    None => {
                    }
                }
            }
            VNode::Text {
                ref mut concrete,
                ref mut virt,
            } => {
                let mut old_virt: Option<VText> = None;
                match last {
                    // First creating the node
                    Some(VNode::Text {
                        concrete: None,
                        virt: _,
                    })
                    | Some(VNode::Element {
                        concrete: None,
                        virt: _,
                    })
                    | None => {
                        let new_el = document().create_text_node(&virt.text);
                        ancestor
                            .append_child(&new_el)
                            .expect("Couldnt append child");
                        *concrete = Some(new_el);
                    }

                    // Just copy reference
                    Some(VNode::Text {
                        concrete: Some(text),
                        virt,
                    }) => {
                        old_virt = Some(virt);
                        *concrete = Some(text);
                    }

                    // Replace node
                    Some(VNode::Element {
                        concrete: Some(el),
                        virt: _,
                    }) => {
                        let new_el = document().create_text_node(&virt.text);
                        ancestor
                            .replace_child(&el, &new_el)
                            .expect("Couldnt append child");
                        *concrete = Some(new_el);
                    }
                };

                // Render over concrete element
                let target = concrete
                    .as_ref()
                    .expect("No concrete dom struct cannot be none");

                virt.render(target, old_virt);
            }
        };
    }
}
