use std::{cell::RefCell, rc::Rc};
use web_sys::{window, Document, Element};

use crate::{css::Css, id_generator::IdGenerator, parser::process_css};

thread_local! {
    static ID_GENERATOR: Rc<RefCell<IdGenerator>> = Rc::new(RefCell::new(IdGenerator::new()));
}

pub(crate) struct CssManager {
    document: Document,
}

#[allow(dead_code)]
impl CssManager {
    pub fn new() -> Self {
        CssManager {
            document: window().unwrap().document().unwrap(),
        }
    }

    pub fn attach_css(&mut self, css: &str) -> Css {
        // generate new id and prefix for the css stylesheet
        let id = ID_GENERATOR.with(|gen| gen.as_ref().borrow_mut().get_new_id());
        let prefix = generate_prefix(id);

        // parse the css and generate new css with mapping
        let (new_css, mapping) = process_css(css, &prefix);

        // generate new style element
        let style: Element = add_new_style_element(&self.document);
        style.set_text_content(Some(&new_css));

        // return new Css object
        Css::new(id, style, mapping)
    }
}

impl Default for CssManager {
    fn default() -> Self {
        Self::new()
    }
}

fn add_new_style_element(document: &Document) -> Element {
    let style = document.create_element("style").unwrap();
    style.append_child(&document.create_text_node("")).unwrap();
    document.head().unwrap().append_child(&style).unwrap();
    style
}

fn generate_prefix(id: u8) -> String {
    format!("_{}-", id)
}

#[cfg(test)]
mod tests {

    use gloo::utils::document;
    use wasm_bindgen_test::*;
    use web_sys::Node;

    use super::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn manager_creates_properly() {
        CssManager::new();
    }

    #[wasm_bindgen_test]
    fn add_new_style_element_adds_element_properly() {
        let _new_element = add_new_style_element(&document());

        let style_element = document().head().unwrap().last_element_child().unwrap();

        assert_eq!(style_element.local_name(), "style");
        assert_eq!(
            style_element.first_child().unwrap().node_type(),
            Node::TEXT_NODE
        );
    }

    #[wasm_bindgen_test]
    fn manager_attaches_css_properly() {
        let mut manager = CssManager::new();

        let _css = manager.attach_css(".class1 { color: red; }");

        let style_element = manager
            .document
            .head()
            .unwrap()
            .last_element_child()
            .unwrap();

        let css_txt = style_element.first_child().unwrap().text_content().unwrap();

        assert_eq!(css_txt, "._0-class1 { color: red; }");
    }
}
