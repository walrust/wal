use web_sys::{Node, Text};

use crate::utils::debug;

use super::{dom, VNode};

/// Node in virtual DOM tree representing [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text). It undergoes virtual DOM manipulations and patching algorithm optimizations.
#[derive(PartialEq, Debug)]
pub struct VText {
    pub(crate) text: String,
    pub(crate) dom: Option<Text>,
}

impl VText {
    /// Constructor for [VText] out of any object implementing trait [ToString].
    ///
    /// # Example
    ///
    /// ```
    /// let vtext1 = VText::new(321);
    /// let vtext2 = VText::new("321");
    /// assert_eq!(vtext1, vtext2);
    /// ```
    pub fn new<T: ToString>(text: T) -> VText {
        VText {
            text: text.to_string(),
            dom: None,
        }
    }

    pub(crate) fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching TextNode");
        let mut old_virt: Option<VText> = None;

        match last {
            None => {
                debug::log("\tCreating text for the first time");
                self.dom = None;
            }
            Some(VNode::Text(vtext)) => {
                self.dom = vtext.dom.clone();
                old_virt = Some(vtext);
            }
            Some(VNode::Element(v)) => {
                debug::log("\tCreating text for the first time and swapping with existing element");
                self.dom = None;
                v.erase();
            }
            Some(VNode::Component(v)) => {
                debug::log(
                    "\tCreating text for the first time and swapping with existing comp node",
                );
                self.dom = None;
                v.erase();
            }
            Some(VNode::List(v)) => {
                debug::log("\tCreating text for the first time and swapping with list");
                self.dom = None;
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
        self.check_if_parents_match(ancestor);
    }

    pub(crate) fn erase(&self) {
        if let Some(text) = &self.dom {
            dom::remove_node(text);
        }
    }
}

impl VText {
    fn render(&mut self, last: Option<VText>, ancestor: &Node) {
        match last {
            // Different value => just change node value
            Some(last) if self.text != last.text => {
                self.dom
                    .as_ref()
                    .expect("Dom is not created even though it should have been")
                    .set_node_value(Some(self.text.as_str()));
            }
            // Same thing => do nothing
            Some(_) => (),
            None => {
                let el = dom::create_text_node(&self.text);

                match &self.dom {
                    Some(old_child) => dom::replace_child(ancestor, old_child, &el),
                    None => dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }

    fn check_if_parents_match(&mut self, ancestor: &Node) {
        // Corner case when parent is changed but child cannot be reassigned earlier
        let parent_node = self.dom.as_ref().unwrap().parent_node().unwrap();
        if !parent_node.eq(ancestor) {
            let dom_ref = self.dom.as_ref().unwrap();
            dom::remove_child(&parent_node, dom_ref);
            dom::append_child(ancestor, dom_ref);
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{dom, VComponent, VElement, VList, VNode},
    };

    use super::VText;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn new_from_str() {
        let target = VText::new("I love Rust");
        assert_eq!(target.text, String::from("I love Rust"));
    }

    macro_rules! function_name {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f").unwrap()
        }};
    }

    const VALID_TEXT: &str = "";

    #[wasm_bindgen_test]
    fn patch_last_none() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let mut target = VText::new(VALID_TEXT);
        target.patch(None, &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_text() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());

        let current = dom::create_text_node("I dont love Rust");
        dom::append_child(&ancestor, &current);

        dom::append_child(&dom::get_root_element(), &ancestor);

        let text = VNode::Text(VText {
            text: "I dont love Rust".into(),
            dom: Some(current),
        });

        let mut target = VText::new(VALID_TEXT);
        target.patch(Some(text), &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_elem() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());

        let current = dom::create_element("div");
        dom::set_attribute(&current, "id", "I dont love Rust");
        dom::append_child(&ancestor, &current);

        dom::append_child(&dom::get_root_element(), &ancestor);

        let elem = VNode::Element(VElement {
            tag_name: "div".into(),
            attr: [("id".into(), "I dont love Rust".into())].into(),
            event_handlers: vec![],
            key: None,
            children: vec![],
            dom: Some(current),
        });

        let mut target = VText::new(VALID_TEXT);
        target.patch(Some(elem), &ancestor);
    }

    struct Comp;
    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            Comp
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new("I dont love Rust").into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn patch_last_comp() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let mut comp = VNode::Component(VComponent::new::<Comp>((), None));
        comp.set_depth(0);
        comp.patch(None, &ancestor);

        let mut target = VText::new(VALID_TEXT);
        target.patch(Some(comp), &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_list() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let mut list = VNode::List(VList::new(
            vec![VText::new("I dont love Rust").into()],
            None,
        ));
        list.patch(None, &ancestor);

        let mut target = VText::new(VALID_TEXT);
        target.patch(Some(list), &ancestor);
    }
}
