use std::{collections::HashMap, ops::Index};
use web_sys::Element;

pub struct Css {
    stylesheet_id: u8,
    element: Element,
    selector_map: HashMap<String, String>,
}
#[allow(dead_code)]
impl Css {
    pub fn new(stylesheet_id: u8, element: Element, selector_map: HashMap<String, String>) -> Self {
        Css {
            stylesheet_id,
            element,
            selector_map,
        }
    }

    pub fn get_id(&self) -> u8 {
        self.stylesheet_id
    }

    pub fn get_inner_css(&self) -> String {
        self.element.text_content().unwrap()
    }
}

// Indexing operator for accessing prepended selectors by original selector names
impl Index<&str> for Css {
    type Output = String;
    fn index(&self, index: &str) -> &Self::Output {
        self.selector_map.get(index).unwrap_or_else(|| {
            panic!(
                "CSS selector {} is not defined in the given stylesheet",
                index
            )
        })
    }
}
