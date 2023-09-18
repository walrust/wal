use gloo::utils::document;
use serde::Serialize;
use web_sys::Text;

#[derive(Serialize, PartialEq, Debug)]
pub struct VText {
    pub text: String,
}

impl VText {
    pub fn new<T: ToString> (text: T) -> VText {
        VText{ text: text.to_string() }
    }

    /// Renders virtual text node over concrete DOM Text object. If the last VText
    /// isnt None and text value is the same, function does nothing
    pub fn render(&self, target: &mut Text, last: Option<VText>) {
        match last {
            // Different value => just change node value
            Some(last) if self.text != last.text => {
                target.set_node_value(Some(self.text.as_str()));
            },
            // Same thing => do nothing
            Some(_) => (),
            None => {
                let new_el = document()
                    .create_text_node(self.text.as_str());

                target.replace_with_with_node_1(&new_el).expect("Couldnt replace whole node");
                *target = new_el;
            },
        }
    }
}
