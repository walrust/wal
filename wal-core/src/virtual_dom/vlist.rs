use itertools::{EitherOrBoth, Itertools};
use web_sys::Node;

use super::VNode;

/// A list of nodes of virtual DOM tree. It undergoes virtual DOM manipulations and patching algorithm optimizations.
#[derive(PartialEq, Debug)]
pub struct VList {
    _key: Option<String>, // TODO: add logic for key attribute
    pub(crate) nodes: Vec<VNode>,
}

impl VList {
    /// Creates a [VList] out of [vector](Vec) of [VNodes](VNode). Optional key works as an easy comparison option, if keys match, old [VList] is used.
    pub fn new(nodes: Vec<VNode>, key: Option<String>) -> VList {
        VList { nodes, _key: key }
    }

    /// Creates an empty [VList]. Optional key works as an easy comparison option, if keys match, old [VList] is used.
    pub fn new_empty(key: Option<String>) -> VList {
        VList {
            nodes: Vec::new(),
            _key: key,
        }
    }

    pub(crate) fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        let mut old_virt: Option<VList> = None;

        match last {
            None => {}
            Some(VNode::List(vlist)) => {
                old_virt = Some(vlist);
            }
            Some(VNode::Text(v)) => {
                v.erase();
            }
            Some(VNode::Element(v)) => {
                v.erase();
            }
            Some(VNode::Component(v)) => {
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
    }

    pub(crate) fn erase(&self) {
        for node in self.nodes.iter() {
            node.erase();
        }
    }

    pub(crate) fn set_depth(&mut self, depth: u32) {
        for child in self.nodes.iter_mut() {
            child.set_depth(depth);
        }
    }
}

impl VList {
    fn render(&mut self, last: Option<VList>, ancestor: &Node) {
        for e in self
            .nodes
            .iter_mut()
            .zip_longest(last.map_or_else(Vec::new, |x| x.nodes.into_iter().collect()))
        {
            match e {
                EitherOrBoth::Both(cur, old) => cur.patch(Some(old), ancestor),
                EitherOrBoth::Left(cur) => cur.patch(None, ancestor),
                EitherOrBoth::Right(old) => old.erase(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{dom, VComponent, VElement, VNode, VText},
    };

    use super::VList;
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

        let mut target = VList::new(vec![VText::new(VALID_TEXT).into()], None);
        target.set_depth(0);
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

        let mut target = VList::new(vec![VText::new(VALID_TEXT).into()], None);
        target.set_depth(0);
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

        let mut target = VList::new(vec![VText::new(VALID_TEXT).into()], None);
        target.set_depth(0);
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

        let mut target = VList::new(vec![VText::new(VALID_TEXT).into()], None);
        target.set_depth(0);
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

        let mut target = VList::new(vec![VText::new(VALID_TEXT).into()], None);
        target.set_depth(0);
        target.patch(Some(list), &ancestor);
    }
}
