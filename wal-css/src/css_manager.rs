use std::{cell::RefCell, rc::Rc};
use web_sys::{window, Document, Element};

use crate::{css::Css, id_generator::IdGenerator, parser::process_css};

thread_local! {
    static ID_GENERATOR: Rc<RefCell<IdGenerator>> = Rc::new(RefCell::new(IdGenerator::new()));
}

pub struct CssManager {
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
