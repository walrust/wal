use std::{collections::HashMap, ops::Index};
use web_sys::Element;

/// Css struct allows to reference the stylesheet selectors from appended stylesheet
/// inside the [rsx macro](../../wal_rsx/macro.rsx.html).
///
/// In order to referecne a selector from the stylesheet use the indexing operator with
/// the name of the selector as an argument.
///
/// # Example usage
/// ```
/// use wal_css::css:Css;
/// use wal_css::css_stylesheet;
///
/// thread_local! {
///     static CSS: Css = css_stylesheet!("path-to-css-file");
/// }
/// // ...
/// CSS.with(|css| {
///     rsx! {
///         <div class={format!("{} {}", css["class1"], css["class2"])} />
///     }
/// })
/// ```
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Css;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn get_document() -> web_sys::Document {
        web_sys::window().unwrap().document().unwrap()
    }

    #[wasm_bindgen_test]
    fn indexing_operator_yields_result_if_entry_present() {
        let mut selector_map = HashMap::new();
        selector_map.insert("class1".to_owned(), "_1-class1".to_owned());

        let element = get_document().create_element("style").unwrap();

        let css = Css::new(1, element, selector_map);

        assert_eq!(css["class1"], "_1-class1");
    }

    #[wasm_bindgen_test]
    #[should_panic]
    fn indexing_operator_panicst_if_entry_not_present() {
        let mut selector_map = HashMap::new();
        selector_map.insert("class1".to_owned(), "_1-class1".to_owned());

        let element = get_document().create_element("style").unwrap();

        let css = Css::new(1, element, selector_map);

        let _x = &css["clas_not_included"];
    }
}
