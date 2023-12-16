use itertools::{EitherOrBoth, Itertools};
use std::collections::{HashMap, HashSet};
use web_sys::{Element, Node};

use crate::{events::EventHandler, utils::debug, virtual_dom::dom};

use super::VNode;

#[derive(Debug)]
pub struct VElement {
    pub tag_name: String,
    pub attr: HashMap<String, String>,
    pub event_handlers: Vec<EventHandler>,
    pub key: Option<String>,
    pub children: Vec<VNode>,

    pub dom: Option<Element>,
}

impl VElement {
    // TODO: maybe some types for attributes and children
    // List of attributes - https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
    //                    - https://www.w3schools.com/tags/ref_attributes.asp
    // Maybe some oop approach with defined types for attributes and types for elements?
    pub fn new(
        tag_name: String,
        attr: HashMap<String, String>,
        event_handlers: Vec<EventHandler>,
        key: Option<String>,
        children: Vec<VNode>,
    ) -> VElement {
        VElement {
            tag_name,
            attr,
            event_handlers,
            key,
            children,
            dom: None,
        }
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching element");
        let mut old_virt: Option<VElement> = None;

        match last {
            None => {
                debug::log("\tCreating element for the first time");
                self.dom = None;
            }
            Some(VNode::Element(mut velement)) => {
                debug::log("\tComparing two elements");
                self.dom = velement.dom.take();
                old_virt = Some(velement);
            }
            Some(VNode::Text(v)) => {
                debug::log("\tCreating element for the first time and swapping with existing text");
                self.dom = None;
                v.erase();
            }
            Some(VNode::Component(v)) => {
                debug::log(
                    "\tCreating element for the first time and swapping with existing comp node",
                );
                self.dom = None;
                v.erase();
            }
            Some(VNode::List(v)) => {
                debug::log("\tCreating element for the first time and swapping with list");
                self.dom = None;
                v.erase();
            }
        }

        self.render(old_virt.as_ref(), ancestor);
        self.handle_children(old_virt);
        self.check_if_parents_match(ancestor);
    }

    pub fn erase(&self) {
        if let Some(el) = &self.dom {
            dom::remove_node(el);
        }
    }

    pub fn set_depth(&mut self, depth: u32) {
        debug::log(format!("VElement: Setting depth: {depth}"));
        for child in self.children.iter_mut() {
            child.set_depth(depth);
        }
    }
}

impl VElement {
    /// Renders virtual Element into concrete DOM Element object. Diffs on tag name,
    /// attributes and children
    fn render(&mut self, last: Option<&VElement>, ancestor: &Node) {
        match last {
            // comparison over user-defined key, if match dont do anything
            Some(last) if last.key.is_some() && last.key == self.key => {
                dom::append_child(ancestor, self.dom.as_ref().unwrap());
            }

            Some(last) if last.tag_name == self.tag_name => {
                debug::log("\t\tComparing attrs");
                let target = self
                    .dom
                    .as_mut()
                    .expect("Target dom object not created before rendering element");
                // Compare attributes
                for (key, val) in self.attr.iter() {
                    dom::set_attribute(target, key, val);
                }
                for (key, _val) in last.attr.iter() {
                    if !self.attr.contains_key(key) {
                        dom::remove_attribute(target, key);
                    }
                }

                for event_handler in &mut self.event_handlers {
                    event_handler.attach(target);
                }
            }
            _ => {
                // inverted check, if last == None || last = Some(x) that x.tag_name !=
                // self.tag_name => Swap whole element
                debug::log("\t\tRendering new node");
                let el = dom::create_element(&self.tag_name);

                // add attributes
                for (name, value) in self.attr.iter() {
                    dom::set_attribute(&el, name, value);
                }

                for event_handler in &mut self.event_handlers {
                    event_handler.attach(&el);
                }

                match &self.dom {
                    Some(old_child) => dom::replace_child(ancestor, old_child, &el),
                    None => dom::append_child(ancestor, &el),
                };
                self.dom = Some(el);
            }
        }
    }

    fn handle_children(&mut self, old_element: Option<VElement>) {
        let target = self.dom.as_mut().unwrap();
        let old_children = old_element.map_or(Vec::new(), |e| e.children.into_iter().collect());

        for either_child_or_both in self.children.iter_mut().zip_longest(old_children) {
            match either_child_or_both {
                EitherOrBoth::Both(child, old_child) => {
                    child.patch(Some(old_child), target);
                }
                EitherOrBoth::Left(child) => {
                    child.patch(None, target);
                }
                EitherOrBoth::Right(old_child) => {
                    // child doesnt exist anymore
                    old_child.erase();
                }
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

impl PartialEq for VElement {
    fn eq(&self, other: &Self) -> bool {
        let self_event_handlers: HashSet<_> = self.event_handlers.iter().collect();
        let other_event_handlers: HashSet<_> = other.event_handlers.iter().collect();

        self.tag_name == other.tag_name
            && self.attr == other.attr
            && self.children == other.children
            && self.dom == other.dom
            && self_event_handlers == other_event_handlers
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{dom, VComponent, VList, VNode, VText},
    };

    use super::VElement;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            None,
            vec![],
        );
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

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            None,
            vec![],
        );
        target.patch(Some(text), &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_elem_different_key() {
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

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            Some("Different_key".to_string()),
            vec![],
        );
        target.patch(Some(elem), &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_elem_same_key() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());

        let current = dom::create_element("div");
        dom::set_attribute(&current, "id", "I dont love Rust");
        dom::append_child(&ancestor, &current);

        dom::append_child(&dom::get_root_element(), &ancestor);

        let key = Some("Nice".to_string());

        let elem = VNode::Element(VElement {
            tag_name: "div".into(),
            attr: [("id".into(), "I dont love Rust".into())].into(),
            event_handlers: vec![],
            key: key.clone(),
            children: vec![],
            dom: Some(current),
        });

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            key,
            vec![],
        );
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

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            None,
            vec![],
        );
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
        list.set_depth(0);
        list.patch(None, &ancestor);

        let mut target = VElement::new(
            "div".into(),
            [("id".into(), VALID_TEXT.into())].into(),
            vec![],
            None,
            vec![],
        );
        target.patch(Some(list), &ancestor);
    }
}
