use gloo::{console::log, utils::document};
use serde::Serialize;
use web_sys::{Element, Node, Text};

use super::{VElement, VText};

#[derive(Debug, Serialize)]
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
                virt,
            } => VNode::patch_element(concrete, virt, last, ancestor),
            VNode::Text {
                ref mut concrete,
                virt,
            } => VNode::patch_text(concrete, virt, last, ancestor),
        };
    }

    fn patch_element(
        concrete: &mut Option<Element>,
        virt: &mut VElement,
        last: Option<VNode>,
        ancestor: &Node,
    ) {
        log!("Patching element");
        let mut old_virt: Option<VElement> = None;
        match last {
            // First creating the node
            Some(VNode::Element { concrete: None, .. })
            | Some(VNode::Text { concrete: None, .. })
            | None => {
                log!("\tCreating the node from the bottom");
                let new_el = document()
                    .create_element(&virt.tag_name)
                    .expect("Couldnt create new element");
                ancestor
                    .append_child(&new_el)
                    .expect("Couldnt append child to node");
                *concrete = Some(new_el);
            }

            // Just copy
            Some(VNode::Element { concrete: Some(el), virt, }) => {
                log!("\tCopying existing node");
                *concrete = Some(el);
                old_virt = Some(virt);
            }

            // Swap nodes
            Some(VNode::Text { concrete: Some(text), .. }) => {
                log!("\tSwaping existing node");
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
        let target = concrete.as_mut().expect("It shouldnt be none");
        virt.render(target, old_virt.as_ref());
        VNode::handle_children(virt, old_virt, target);
    }

    fn patch_text(
        concrete: &mut Option<Text>,
        virt: &mut VText,
        last: Option<VNode>,
        ancestor: &Node,
    ) {
        let mut old_virt: Option<VText> = None;
        match last {
            // First creating the node
            Some(VNode::Text { concrete: None, virt: _, })
            | Some(VNode::Element { concrete: None, virt: _, })
            | None => {
                let new_el = document().create_text_node(&virt.text);
                ancestor
                    .append_child(&new_el)
                    .expect("Couldnt append child");
                *concrete = Some(new_el);
            }

            // Just copy reference
            Some(VNode::Text { concrete: Some(text), virt, }) => {
                old_virt = Some(virt);
                *concrete = Some(text);
            }

            // Replace node
            Some(VNode::Element { concrete: Some(el), .. }) => {
                let new_el = document().create_text_node(&virt.text);
                ancestor
                    .replace_child(&el, &new_el)
                    .expect("Couldnt append child");
                *concrete = Some(new_el);
            }
        };

        // Render over concrete element
        let target = concrete
            .as_mut()
            .expect("No concrete dom struct cannot be none");

        virt.render(target, old_virt);
    }

    fn handle_children(
        virt: &mut VElement,
        old_virt: Option<VElement>,
        target: &mut Element
        ) {
        let mut children: Vec<Option<&mut VNode>> =
            virt.children.iter_mut().map(|x| Some(x)).collect();
        let mut old_children: Vec<Option<VNode>> = match old_virt {
            Some(el) => el.children.into_iter().map(|x| Some(x)).collect(),
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
                    if let Some(node) = match node {
                        VNode::Element { concrete, .. } => concrete.map(Node::from),
                        VNode::Text { concrete, .. } => concrete.map(Node::from),
                    } {
                        target.remove_child(&node).expect("Couldnt remove child");
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
