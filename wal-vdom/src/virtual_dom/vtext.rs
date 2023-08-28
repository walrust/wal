use gloo::{console::__macro::JsValue, utils::document};
use serde::Serialize;
use web_sys::Node;

#[derive(Serialize)]
pub struct VText {
    pub text: String,
}

impl VText {
    // TODO: consider replacing String with trait ToString
    pub fn new<T: ToString>(text: T) -> VText {
        VText {
            text: text.to_string(),
        }
    }

    pub fn render(&self) -> Result<Node, JsValue> {
        return Ok(document().create_text_node(self.text.as_str()).into());
    }
}
