use gloo::console::log;
use serde::Serialize;
use web_sys::Text;


#[derive(Serialize)]
pub struct VText {
    pub text: String,
}

impl VText {
    // TODO: consider replacing String with trait ToString
    pub fn new(text: String) -> VText {
        VText{ text }
    }

    // TODO: Implement :))
    /// Renders virtual text node over concrete DOM Text object. If the last VText
    /// isnt None and text value is the same, function does nothing
    pub fn render(&self, target: &Text, last: Option<VText>) {
        web_sys::console::log_1(&format!("TODO: implement vtext rendering").into());
        log!("TODO: implement vtext rendering");
        target.set_node_value(Some("TEST"));
    }
}
