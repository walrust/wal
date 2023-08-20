use gloo::console::__macro::JsValue;
use serde::Serialize;
use web_sys::Node;

use super::{VElement, VText};

#[derive(Serialize)]
pub enum VNode {
    Element{
        velement: VElement,
    },
    Text{
        vtext: VText,
    },
}

impl VNode {
    pub fn render(&self) -> Result<Node, JsValue> {
        match self {
            VNode::Element { velement } => velement.render(),
            VNode::Text { vtext } => vtext.render(),
        }
    }
}
