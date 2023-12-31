use web_sys::Node;

use crate::component::{node::AnyComponentNode, Component};

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

use super::VNode;

pub(crate) type PropertiesHash = u64;
pub(crate) type AnyProps = Option<Box<dyn Any>>;
pub(crate) type ComponentNodeGenerator =
    Box<dyn Fn(AnyProps, &Node) -> Rc<RefCell<AnyComponentNode>> + 'static>;

/// Special VNode type, which represents custom component node.
/// There is no direct translation of [VComponent] to a single DOM node, but it translates to a subtree of DOM nodes.
pub struct VComponent {
    props: AnyProps,
    hash: PropertiesHash,
    generator: ComponentNodeGenerator,
    key: Option<String>,
    depth: Option<u32>,

    pub(crate) comp: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl VComponent {
    /// Creates [VComponent] out of provided properties. Function is generic, therefore type of [Component] ***C*** has to be specified.
    ///
    /// # Examples
    ///
    /// ```
    /// struct ExampleComponent;
    /// impl Component for ExampleComponent {
    ///     type Properties = ();
    ///     ...
    /// }
    /// let props = ();
    /// let vcomp = VComponent::new::<ExampleComponent>(props, None);
    /// ```
    pub fn new<C>(props: C::Properties, key: Option<String>) -> VComponent
    where
        C: Component + 'static,
    {
        let hash = Self::calculate_hash::<C>(&props);
        let generator = Box::new(Self::generator::<C>);
        VComponent {
            props: Some(Box::new(props)),
            generator,
            hash,
            key,
            depth: None,
            comp: None,
        }
    }

    fn calculate_hash<C>(props: &C::Properties) -> PropertiesHash
    where
        C: Component + 'static,
    {
        let mut hasher = DefaultHasher::new();
        props.hash(&mut hasher);
        TypeId::of::<C>().hash(&mut hasher);
        hasher.finish()
    }

    fn generator<C: Component + 'static>(
        props: AnyProps,
        ancestor: &Node,
    ) -> Rc<RefCell<AnyComponentNode>> {
        let props = props
            .unwrap()
            .downcast::<C::Properties>()
            .expect("Trying to unpack others component properties");

        AnyComponentNode::new(C::new(*props), ancestor.clone())
    }

    pub(crate) fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        let mut old_virt: Option<VComponent> = None;

        match last {
            Some(VNode::Component(vcomp)) => {
                old_virt = Some(vcomp);
            }
            Some(VNode::Element(v)) => {
                v.erase();
            }
            Some(VNode::Text(v)) => {
                v.erase();
            }
            None => {}
            Some(VNode::List(v)) => {
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
    }

    pub(crate) fn erase(&self) {
        if let Some(node) = self.comp.as_ref() {
            node.borrow_mut().vdom.as_ref().unwrap().erase();
        }
    }

    pub(crate) fn set_depth(&mut self, depth: u32) {
        self.depth = Some(depth);
    }

    fn render(&mut self, last: Option<VComponent>, ancestor: &Node) {
        match last {
            Some(mut old_vcomp) if self.key.is_some() && old_vcomp.key == self.key => {
                self.comp = old_vcomp.comp.take();
            }
            Some(mut old_vcomp) if old_vcomp.hash == self.hash => {
                self.comp = old_vcomp.comp.take();
            }
            Some(old_vcomp) => {
                let any_component_node_rc = (self.generator)(self.props.take(), ancestor);
                {
                    let mut any_component_node = any_component_node_rc.borrow_mut();
                    any_component_node.depth = self.depth;
                    any_component_node.view();
                    any_component_node.patch(old_vcomp.comp.clone(), ancestor);
                }
                self.comp = Some(any_component_node_rc);
            }
            None => {
                let any_component_node_rc = (self.generator)(self.props.take(), ancestor);
                {
                    let mut any_component_node = any_component_node_rc.borrow_mut();
                    any_component_node.depth = self.depth;
                    any_component_node.view();
                    any_component_node.patch(None, ancestor);
                }
                self.comp = Some(any_component_node_rc);
            }
        }
    }
}

impl fmt::Debug for VComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VComponent")
            .field("props", &self.props)
            .field("hash", &self.hash)
            .field("comp", &self.comp)
            .finish()
    }
}

impl PartialEq for VComponent {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && match (&self.props, &other.props) {
                (Some(self_props), Some(other_props)) => {
                    (*(*self_props)).type_id() == (*(*other_props)).type_id()
                }
                (None, None) => true,
                _ => false,
            }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{dom, VElement, VList, VNode, VText},
    };

    use super::VComponent;
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

    struct Tmp;
    impl Component for Tmp {
        type Message = ();
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            Tmp
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VElement::new(
                "div".into(),
                [(String::from("result"), String::from(VALID_TEXT))].into(),
                vec![],
                None,
                vec![],
            )
            .into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn patch_last_none() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let mut target = VComponent::new::<Tmp>((), None);
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

        let mut target = VComponent::new::<Tmp>((), None);
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

        let mut target = VComponent::new::<Tmp>((), None);
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
            VElement::new(
                "div".into(),
                [(String::from("result"), String::from("I dont love Rust"))].into(),
                vec![],
                None,
                vec![],
            )
            .into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn patch_last_comp_diff_keys() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let mut comp = VNode::Component(VComponent::new::<Comp>((), None));
        comp.set_depth(0);
        comp.patch(None, &ancestor);

        let mut target = VComponent::new::<Tmp>((), None);
        target.set_depth(0);
        target.patch(Some(comp), &ancestor);
    }

    #[wasm_bindgen_test]
    fn patch_last_comp_same_keys() {
        let ancestor = dom::create_element("div");
        dom::set_attribute(&ancestor, "id", function_name!());
        dom::append_child(&dom::get_root_element(), &ancestor);

        let key = Some(String::from("Same_key"));
        let mut comp = VNode::Component(VComponent::new::<Comp>((), key.clone()));
        comp.set_depth(0);
        comp.patch(None, &ancestor);

        let mut target = VComponent::new::<Tmp>((), key);
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

        let mut target = VComponent::new::<Tmp>((), None);
        target.set_depth(0);
        target.patch(Some(list), &ancestor);
    }
}
