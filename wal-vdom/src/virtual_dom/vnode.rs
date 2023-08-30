use gloo::console::__macro::JsValue;
use serde::Serialize;
use web_sys::Node;

use super::{VElement, VList, VText};

#[derive(Serialize)]
pub enum VNode {
    Element { velement: VElement },
    Text { vtext: VText },
    List { vlist: VList },
}

impl VNode {
    pub fn render(&self) -> Result<Node, JsValue> {
        match self {
            VNode::Element { velement } => velement.render(),
            VNode::Text { vtext } => vtext.render(),
            VNode::List { .. } => unimplemented!(),
        }
    }
}

impl From<VElement> for VNode {
    fn from(velement: VElement) -> Self {
        Self::Element { velement }
    }
}

impl From<VText> for VNode {
    fn from(vtext: VText) -> Self {
        Self::Text { vtext }
    }
}

impl From<VList> for VNode {
    fn from(vlist: VList) -> Self {
        Self::List { vlist }
    }
}

impl<T: ToString> From<T> for VNode {
    fn from(t: T) -> Self {
        Self::Text {
            vtext: VText::new(t),
        }
    }
}
