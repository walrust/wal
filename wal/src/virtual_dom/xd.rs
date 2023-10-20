#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod virtual_dom {
    pub mod vcomponent {
        use gloo::console::log;
        use web_sys::Node;
        use crate::component::{component_node::AnyComponentNode, Component};
        use std::{
            any::Any, cell::RefCell, collections::hash_map::DefaultHasher, fmt,
            hash::{Hash, Hasher},
            rc::Rc,
        };
        use super::VNode;
        pub(crate) type PropertiesHash = u64;
        pub(crate) type AnyProps = Option<Box<dyn Any>>;
        pub(crate) type ComponentNodeGenerator = Box<
            dyn Fn(AnyProps, &Node) -> Rc<RefCell<AnyComponentNode>> + 'static,
        >;
        pub struct VComponent {
            props: AnyProps,
            hash: PropertiesHash,
            generator: ComponentNodeGenerator,
            pub comp: Option<Rc<RefCell<AnyComponentNode>>>,
        }
        impl VComponent {
            pub fn new<C>(props: C::Properties) -> VComponent
            where
                C: Component + 'static,
            {
                let hash = Self::calculate_hash(&props);
                let generator = Box::new(Self::generator::<C>);
                VComponent {
                    props: Some(Box::new(props)),
                    generator,
                    hash,
                    comp: None,
                }
            }
            fn calculate_hash<T: Hash>(props: &T) -> PropertiesHash {
                let mut hasher = DefaultHasher::new();
                props.hash(&mut hasher);
                hasher.finish()
            }
            fn generator<C: Component + 'static>(
                props: AnyProps,
                ancestor: &Node,
            ) -> Rc<RefCell<AnyComponentNode>> {
                let props = props
                    .unwrap()
                    .downcast::<C::Properties>()
                    .expect("Trying to unpack others component props");
                AnyComponentNode::new(C::new(*props), ancestor.clone())
            }
            pub fn patch(&mut self, last: Option<&VNode>, ancestor: &Node) {
                ::gloo_console::externs::log(
                    ::std::boxed::Box::from([
                        ::gloo_console::__macro::JsValue::from("Patching component"),
                    ]),
                );
                let mut old_virt: Option<&VComponent> = None;
                match last {
                    Some(VNode::Component(vcomp)) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tComparing two components",
                                ),
                            ]),
                        );
                        old_virt = Some(vcomp);
                    }
                    Some(VNode::Element(_)) | Some(VNode::Text(_)) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tNew component over element/text",
                                ),
                            ]),
                        );
                    }
                    None => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCreating the comp for the first time",
                                ),
                            ]),
                        );
                    }
                    Some(VNode::List(_)) => {
                        ::core::panicking::panic("not yet implemented")
                    }
                }
                self.render(old_virt, ancestor);
            }
            fn render(&mut self, last: Option<&VComponent>, ancestor: &Node) {
                match last {
                    Some(old_vcomp) if old_vcomp.hash == self.hash => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\t\tHashes are the same",
                                ),
                            ]),
                        );
                        self.comp = old_vcomp.comp.clone();
                    }
                    Some(old_vcomp) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from("\t\tHashes differ"),
                            ]),
                        );
                        let any_component_node_rc = (self
                            .generator)(self.props.take(), ancestor);
                        {
                            let mut any_component_node = any_component_node_rc
                                .borrow_mut();
                            any_component_node.patch(old_vcomp.comp.clone(), ancestor);
                        }
                        self.comp = Some(any_component_node_rc);
                    }
                    None => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\t\tThere was no component before",
                                ),
                            ]),
                        );
                        let any_component_node_rc = (self
                            .generator)(self.props.take(), ancestor);
                        {
                            let mut any_component_node = any_component_node_rc
                                .borrow_mut();
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
            fn eq(&self, _other: &Self) -> bool {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod velement {
        use gloo::console::log;
        use itertools::{EitherOrBoth, Itertools};
        use std::collections::HashMap;
        use web_sys::{Element, Node};
        use crate::virtual_dom::Dom;
        use super::VNode;
        pub struct VElement {
            pub tag_name: String,
            pub attr: HashMap<String, String>,
            pub children: Vec<VNode>,
            pub dom: Option<Element>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VElement {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "VElement",
                    "tag_name",
                    &self.tag_name,
                    "attr",
                    &self.attr,
                    "children",
                    &self.children,
                    "dom",
                    &&self.dom,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for VElement {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for VElement {
            #[inline]
            fn eq(&self, other: &VElement) -> bool {
                self.tag_name == other.tag_name && self.attr == other.attr
                    && self.children == other.children && self.dom == other.dom
            }
        }
        impl VElement {
            pub fn new(
                tag_name: String,
                attr: HashMap<String, String>,
                children: Vec<VNode>,
            ) -> VElement {
                VElement {
                    tag_name,
                    attr,
                    children,
                    dom: None,
                }
            }
            pub fn patch(&mut self, last: Option<&VNode>, ancestor: &Node) {
                ::gloo_console::externs::log(
                    ::std::boxed::Box::from([
                        ::gloo_console::__macro::JsValue::from("Patching element"),
                    ]),
                );
                let mut old_virt: Option<&VElement> = None;
                match last {
                    None => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCreating the node for the first time",
                                ),
                            ]),
                        );
                        self.dom = None;
                    }
                    Some(VNode::Element(velement)) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCopying existing node",
                                ),
                            ]),
                        );
                        self.dom = velement.dom.clone();
                        old_virt = Some(velement);
                    }
                    Some(VNode::Text(_)) | Some(VNode::Component(_)) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCreating the node for the first time and swapping with existing text/comp node",
                                ),
                            ]),
                        );
                        self.dom = None;
                    }
                    Some(VNode::List(_)) => {
                        ::core::panicking::panic("not yet implemented")
                    }
                }
                self.render(old_virt, ancestor);
                self.handle_children(old_virt);
            }
        }
        impl VElement {
            /// Renders virtual Element into concrete DOM Element object. Diffs on tag name,
            /// attributes and children
            fn render(&mut self, last: Option<&VElement>, ancestor: &Node) {
                match last {
                    Some(last) if last.tag_name == self.tag_name => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\t\tComparing attrs",
                                ),
                            ]),
                        );
                        let target = self
                            .dom
                            .as_mut()
                            .expect(
                                "Target dom object not created before rendering element",
                            );
                        for (key, val) in self.attr.iter() {
                            Dom::set_attribute(target, key, val);
                        }
                        for (key, _val) in last.attr.iter() {
                            if !self.attr.contains_key(key) {
                                Dom::remove_attribute(target, key);
                            }
                        }
                    }
                    _ => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\t\tRendering new node",
                                ),
                            ]),
                        );
                        let el = Dom::create_element(&self.tag_name);
                        for (name, value) in self.attr.iter() {
                            Dom::set_attribute(&el, name, value);
                        }
                        match &self.dom {
                            Some(old_child) => {
                                Dom::replace_child(ancestor, old_child, &el)
                            }
                            None => Dom::append_child(ancestor, &el),
                        };
                        self.dom = Some(el);
                    }
                }
            }
            fn handle_children(&mut self, old_element: Option<&VElement>) {
                let target = self.dom.as_mut().unwrap();
                let old_children = old_element
                    .map_or(Vec::new(), |e| e.children.iter().collect());
                for either_child_or_both in self
                    .children
                    .iter_mut()
                    .zip_longest(old_children)
                {
                    match either_child_or_both {
                        EitherOrBoth::Both(child, old_child) => {
                            child.patch(Some(old_child), target);
                        }
                        EitherOrBoth::Left(child) => {
                            child.patch(None, target);
                        }
                        EitherOrBoth::Right(old_child) => {
                            if let Some(node) = old_child.get_dom() {
                                Dom::remove_child(&target, &node);
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod vlist {
        use super::VNode;
        pub struct VList {
            pub nodes: Vec<VNode>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for VList {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for VList {
            #[inline]
            fn eq(&self, other: &VList) -> bool {
                self.nodes == other.nodes
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VList {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "VList",
                    "nodes",
                    &&self.nodes,
                )
            }
        }
        impl VList {
            pub fn new(nodes: Vec<VNode>) -> VList {
                VList { nodes }
            }
            pub fn new_empty() -> VList {
                VList { nodes: Vec::new() }
            }
        }
    }
    pub mod vnode {
        use web_sys::Node;
        use super::{VComponent, VElement, VList, VText};
        pub enum VNode {
            Element(VElement),
            Text(VText),
            List(VList),
            Component(VComponent),
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for VNode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for VNode {
            #[inline]
            fn eq(&self, other: &VNode) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (VNode::Element(__self_0), VNode::Element(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (VNode::Text(__self_0), VNode::Text(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (VNode::List(__self_0), VNode::List(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (VNode::Component(__self_0), VNode::Component(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VNode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    VNode::Element(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Element",
                            &__self_0,
                        )
                    }
                    VNode::Text(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Text",
                            &__self_0,
                        )
                    }
                    VNode::List(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "List",
                            &__self_0,
                        )
                    }
                    VNode::Component(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Component",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl VNode {
            pub fn patch(&mut self, last: Option<&VNode>, ancestor: &Node) {
                match self {
                    VNode::Element(velement) => velement.patch(last, ancestor),
                    VNode::Text(vtext) => vtext.patch(last, ancestor),
                    VNode::Component(vcomp) => vcomp.patch(last, ancestor),
                    VNode::List(_) => ::core::panicking::panic("not yet implemented"),
                };
            }
            pub fn get_dom(&self) -> Option<Node> {
                match self {
                    VNode::Element(velement) => {
                        velement.dom.as_ref().cloned().map(Into::into)
                    }
                    VNode::Text(vtext) => vtext.dom.as_ref().cloned().map(Into::into),
                    VNode::Component(vcomp) => {
                        vcomp.comp.as_ref().unwrap().borrow().vdom.get_dom()
                    }
                    VNode::List(_) => ::core::panicking::panic("not yet implemented"),
                }
            }
        }
        impl From<VElement> for VNode {
            fn from(velement: VElement) -> Self {
                Self::Element(velement)
            }
        }
        impl From<VComponent> for VNode {
            fn from(vcomp: VComponent) -> Self {
                Self::Component(vcomp)
            }
        }
        impl From<VText> for VNode {
            fn from(vtext: VText) -> Self {
                Self::Text(vtext)
            }
        }
        impl From<VList> for VNode {
            fn from(vlist: VList) -> Self {
                Self::List(vlist)
            }
        }
        impl<T: ToString> From<T> for VNode {
            fn from(t: T) -> Self {
                Self::Text(VText::new(t))
            }
        }
        impl<T: Into<VNode>> FromIterator<T> for VNode {
            fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
                Self::List(VList::new(iter.into_iter().map(Into::into).collect()))
            }
        }
    }
    pub mod vtext {
        use gloo::console::log;
        use web_sys::{Node, Text};
        use super::{Dom, VNode};
        pub struct VText {
            pub text: String,
            pub dom: Option<Text>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for VText {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for VText {
            #[inline]
            fn eq(&self, other: &VText) -> bool {
                self.text == other.text && self.dom == other.dom
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for VText {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "VText",
                    "text",
                    &self.text,
                    "dom",
                    &&self.dom,
                )
            }
        }
        impl VText {
            pub fn new<T: ToString>(text: T) -> VText {
                VText {
                    text: text.to_string(),
                    dom: None,
                }
            }
            pub fn patch(&mut self, last: Option<&VNode>, ancestor: &Node) {
                ::gloo_console::externs::log(
                    ::std::boxed::Box::from([
                        ::gloo_console::__macro::JsValue::from("Patching TextNode"),
                    ]),
                );
                let mut old_virt: Option<&VText> = None;
                match last {
                    None => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCreating the node for the first time",
                                ),
                            ]),
                        );
                        self.dom = None;
                    }
                    Some(VNode::Text(vtext)) => {
                        self.dom = vtext.dom.clone();
                        old_virt = Some(vtext);
                    }
                    Some(VNode::Element(_)) | Some(VNode::Component(_)) => {
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(
                                    "\tCreating the node for the first time and swapping with existing text/comp node",
                                ),
                            ]),
                        );
                        self.dom = None;
                    }
                    Some(VNode::List(_)) => {
                        ::core::panicking::panic("not yet implemented")
                    }
                }
                self.render(old_virt, ancestor);
            }
        }
        impl VText {
            /// Renders virtual text node over concrete DOM Text object. If the last VText
            /// isnt None and text value is the same, function does nothing
            fn render(&mut self, last: Option<&VText>, ancestor: &Node) {
                match last {
                    Some(last) if self.text != last.text => {
                        self.dom
                            .as_ref()
                            .expect("Dom is not created even though it should have been")
                            .set_node_value(Some(self.text.as_str()));
                    }
                    Some(_) => {}
                    None => {
                        let el = Dom::create_text_node(&self.text);
                        match &self.dom {
                            Some(old_child) => {
                                Dom::replace_child(ancestor, &old_child, &el)
                            }
                            None => Dom::append_child(ancestor, &el),
                        };
                        self.dom = Some(el);
                    }
                }
            }
        }
    }
    use gloo::console::log;
    use gloo::utils::{body, document};
    use web_sys::{Element, Node, Text};
    pub use self::vcomponent::VComponent;
    pub use self::velement::VElement;
    pub use self::vlist::VList;
    pub use self::vnode::VNode;
    pub use self::vtext::VText;
    pub struct Dom;
    impl Dom {
        const ROOT_ELEMENT_ID: &'static str = "walrust-root";
        pub fn get_root_element() -> Node {
            Node::from(
                document()
                    .get_element_by_id(Self::ROOT_ELEMENT_ID)
                    .unwrap_or_else(|| {
                        let message = {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "There was no \'{0}\' element, adding default one",
                                    Self::ROOT_ELEMENT_ID,
                                ),
                            );
                            res
                        };
                        ::gloo_console::externs::log(
                            ::std::boxed::Box::from([
                                ::gloo_console::__macro::JsValue::from(message),
                            ]),
                        );
                        let root = document().create_element("div").unwrap();
                        Dom::set_attribute(&root, "id", Self::ROOT_ELEMENT_ID);
                        Dom::append_child(&body(), &root);
                        root
                    }),
            )
        }
        pub fn create_element(local_name: &str) -> Element {
            document().create_element(local_name).expect("Couldnt create new element")
        }
        pub fn create_text_node(data: &String) -> Text {
            document().create_text_node(data)
        }
        pub fn append_child(ancestor: &Node, child: &Node) -> Node {
            ancestor.append_child(child).expect("Couldnt append child to node")
        }
        pub fn replace_child(ancestor: &Node, old_child: &Node, child: &Node) -> Node {
            ancestor
                .replace_child(old_child, child)
                .expect("Couldnt replace child with a new node")
        }
        pub fn remove_child(ancestor: &Node, child: &Node) -> Node {
            ancestor.remove_child(child).expect("Couldnt remove child")
        }
        pub fn set_attribute(el: &Element, name: &str, value: &str) {
            el.set_attribute(name, value).expect("Couldnt set attribute")
        }
        pub fn remove_attribute(el: &Element, name: &str) {
            el.remove_attribute(name).expect("Couldnt remove attribute")
        }
    }
}
pub mod component {
    use crate::virtual_dom::VNode;
    use std::any::Any;
    use std::hash::Hash;
    use self::component_node::{AnyComponentBehavior, ComponentBehavior};
    pub mod callback {
        use std::{hash::Hash, rc::Rc};
        use gloo::console::log;
        pub struct Callback<IN> {
            wrapper: Rc<dyn Fn(IN)>,
        }
        impl<IN> Callback<IN> {
            pub fn new<F>(wrapper: F) -> Self
            where
                F: Fn(IN) + 'static,
            {
                Callback {
                    wrapper: Rc::new(wrapper),
                }
            }
            pub fn emit(&self, input: IN) {
                ::gloo_console::externs::log(
                    ::std::boxed::Box::from([
                        ::gloo_console::__macro::JsValue::from("Emitting callback"),
                    ]),
                );
                (self.wrapper)(input);
            }
        }
        impl<IN> Hash for Callback<IN> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let ptr = self.wrapper.as_ref() as *const dyn Fn(IN);
                ptr.hash(state);
            }
        }
        impl<IN> Clone for Callback<IN> {
            fn clone(&self) -> Self {
                Self {
                    wrapper: self.wrapper.clone(),
                }
            }
        }
    }
    pub mod component_node {
        use crate::virtual_dom::VNode;
        use gloo::console::log;
        use std::{cell::RefCell, fmt, marker::PhantomData, mem, rc::Rc};
        use web_sys::Node;
        use super::{callback::Callback, scheduler::Scheduler, AnyComponent, Component};
        pub struct AnyComponentNode {
            component: Rc<RefCell<Box<dyn AnyComponent>>>,
            depth: u32,
            to_rerender: Rc<RefCell<bool>>,
            behavior: Rc<AnyComponentBehavior>,
            pub vdom: VNode,
            ancestor: Node,
            vdom_observer: Rc<RefCell<VDomObserver>>,
            to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
        }
        impl AnyComponentNode {
            pub fn new_root<C: Component + 'static>(
                component: C,
                ancestor: Node,
            ) -> Rc<RefCell<Self>> {
                let root_rc = Self::new(component, ancestor);
                {
                    let mut root = root_rc.borrow_mut();
                    root.new_root_patch();
                }
                root_rc
            }
            fn new_root_patch(&mut self) {
                self.vdom.patch(None, &self.ancestor);
            }
            pub fn new<C: Component + 'static>(
                component: C,
                ancestor: Node,
            ) -> Rc<RefCell<Self>> {
                let component_box = Box::new(component) as Box<dyn AnyComponent>;
                let component_rc = Rc::new(RefCell::new(component_box));
                let to_rerender_observer_rc = Rc::new(
                    RefCell::new(ToRerenderObserver::new()),
                );
                let behavior_rc = Rc::new(
                    AnyComponentBehavior::new(
                        component_rc.clone(),
                        to_rerender_observer_rc.clone(),
                    ),
                );
                let vdom = component_rc.borrow().view(&behavior_rc);
                let vdom_observer_rc = Rc::new(RefCell::new(VDomObserver::new()));
                let node = Self {
                    component: component_rc,
                    depth: 0,
                    to_rerender: Rc::new(RefCell::new(false)),
                    behavior: behavior_rc,
                    vdom,
                    ancestor,
                    vdom_observer: vdom_observer_rc,
                    to_rerender_observer: to_rerender_observer_rc,
                };
                let node_rc = Rc::new(RefCell::new(node));
                node_rc
                    .borrow()
                    .to_rerender_observer
                    .borrow_mut()
                    .set_observer(node_rc.clone());
                node_rc
                    .borrow()
                    .vdom_observer
                    .borrow_mut()
                    .set_observer(node_rc.clone());
                node_rc
            }
            fn rerender_notify(&mut self) {
                let mut to_rerender = self.to_rerender.borrow_mut();
                if !*to_rerender {
                    *to_rerender = true;
                    Scheduler::add_rerender_message(
                        self.component.clone(),
                        self.behavior.clone(),
                        self.vdom_observer.clone(),
                        self.to_rerender.clone(),
                        self.depth,
                    );
                }
            }
            fn vdom_notify(&mut self, mut new_vdom: VNode) {
                mem::swap(&mut new_vdom, &mut self.vdom);
                self.vdom.patch(Some(&new_vdom), &self.ancestor);
            }
            pub fn patch(
                &mut self,
                last_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
                ancestor: &Node,
            ) {
                if let Some(last_component_node) = last_component_node {
                    let last_component_node = last_component_node.clone();
                    let last_component_node_vdom = &last_component_node.borrow().vdom;
                    self.vdom.patch(Some(last_component_node_vdom), ancestor);
                } else {
                    self.vdom.patch(None, ancestor)
                }
            }
            pub fn get_dom(&self) -> Option<Node> {
                self.vdom.get_dom()
            }
        }
        impl fmt::Debug for AnyComponentNode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                "AnyComponentNode".fmt(f)
            }
        }
        pub struct AnyComponentBehavior {
            component: Rc<RefCell<Box<dyn AnyComponent>>>,
            rerender_observer: Rc<RefCell<ToRerenderObserver>>,
        }
        impl AnyComponentBehavior {
            pub fn new(
                component: Rc<RefCell<Box<dyn AnyComponent>>>,
                rerender_observer: Rc<RefCell<ToRerenderObserver>>,
            ) -> Self {
                Self {
                    component,
                    rerender_observer,
                }
            }
        }
        pub struct ComponentBehavior<C: Component> {
            component: Rc<RefCell<Box<dyn AnyComponent>>>,
            rerender_observer: Rc<RefCell<ToRerenderObserver>>,
            _pd: PhantomData<C>,
        }
        impl<C: Component> ComponentBehavior<C> {
            pub fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
            where
                F: Fn(IN) -> C::Message + 'static,
            {
                let component = self.component.clone();
                let rerender_observer = self.rerender_observer.clone();
                Callback::new(move |data| {
                    let message = wrapper(data);
                    Scheduler::add_update_message(
                        component.clone(),
                        Box::new(message),
                        rerender_observer.clone(),
                    );
                })
            }
        }
        impl<C: Component> From<&AnyComponentBehavior> for ComponentBehavior<C> {
            fn from(value: &AnyComponentBehavior) -> Self {
                Self {
                    component: value.component.clone(),
                    rerender_observer: value.rerender_observer.clone(),
                    _pd: PhantomData,
                }
            }
        }
        pub struct VDomObserver {
            component_node: Option<Rc<RefCell<AnyComponentNode>>>,
        }
        impl VDomObserver {
            fn new() -> Self {
                Self { component_node: None }
            }
            fn set_observer(&mut self, component_node: Rc<RefCell<AnyComponentNode>>) {
                self.component_node = Some(component_node);
            }
            pub fn notify(&self, new_vdom: VNode) {
                if let Some(any_componend_node) = &self.component_node {
                    let mut any_component_node = any_componend_node.borrow_mut();
                    any_component_node.vdom_notify(new_vdom);
                } else {
                    ::gloo_console::externs::log(
                        ::std::boxed::Box::from([
                            ::gloo_console::__macro::JsValue::from(
                                "VDomObserver is not attached to a AnyComponentNode",
                            ),
                        ]),
                    );
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "VDomObserver is not attached to a AnyComponentNode",
                            ),
                        );
                    };
                }
            }
        }
        pub struct ToRerenderObserver {
            any_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
        }
        impl ToRerenderObserver {
            fn new() -> Self {
                Self { any_component_node: None }
            }
            fn set_observer(
                &mut self,
                any_component_node: Rc<RefCell<AnyComponentNode>>,
            ) {
                self.any_component_node = Some(any_component_node);
            }
            pub fn notify(&self) {
                if let Some(any_component_node) = &self.any_component_node {
                    any_component_node.borrow_mut().rerender_notify();
                } else {
                    ::gloo_console::externs::log(
                        ::std::boxed::Box::from([
                            ::gloo_console::__macro::JsValue::from(
                                "RerenderObserver is not attached to AnyComponentNode",
                            ),
                        ]),
                    );
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "RerenderObserver is not attached to AnyComponentNode",
                            ),
                        );
                    };
                }
            }
        }
    }
    pub mod scheduler {
        use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Rc};
        use wasm_bindgen_futures::spawn_local;
        use super::{
            component_node::{AnyComponentBehavior, ToRerenderObserver, VDomObserver},
            AnyComponent,
        };
        enum SchedulerMessage {
            Update(UpdateMessage),
            Rerender(RerenderMessage),
        }
        impl SchedulerMessage {
            fn handle(self) {
                match self {
                    SchedulerMessage::Update(msg) => msg.handle(),
                    SchedulerMessage::Rerender(msg) => msg.handle(),
                }
            }
        }
        struct UpdateMessage {
            component: Rc<RefCell<Box<dyn AnyComponent>>>,
            message: Box<dyn Any>,
            to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
        }
        impl UpdateMessage {
            fn handle(self) {
                let to_rerender = self.component.borrow_mut().update(self.message);
                if to_rerender {
                    self.to_rerender_observer.borrow().notify();
                }
            }
        }
        struct RerenderMessage {
            component: Rc<RefCell<Box<dyn AnyComponent>>>,
            behavior: Rc<AnyComponentBehavior>,
            vdom_observer: Rc<RefCell<VDomObserver>>,
            to_rerender: Rc<RefCell<bool>>,
            depth: u32,
        }
        impl RerenderMessage {
            fn handle(self) {
                let vdom = self.component.borrow().view(self.behavior.as_ref());
                self.vdom_observer.borrow().notify(vdom);
                *self.to_rerender.borrow_mut() = false;
            }
        }
        impl PartialEq for SchedulerMessage {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Update(s_msg), Self::Update(o_msg)) => {
                        Rc::ptr_eq(&s_msg.component, &o_msg.component)
                            && &s_msg.message as *const dyn Any
                                == &o_msg.message as *const dyn Any
                            && Rc::ptr_eq(
                                &s_msg.to_rerender_observer,
                                &o_msg.to_rerender_observer,
                            )
                    }
                    (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                        Rc::ptr_eq(&s_msg.component, &o_msg.component)
                            && Rc::ptr_eq(&s_msg.behavior, &o_msg.behavior)
                            && Rc::ptr_eq(&s_msg.vdom_observer, &o_msg.vdom_observer)
                            && s_msg.depth == o_msg.depth
                    }
                    _ => false,
                }
            }
        }
        impl Eq for SchedulerMessage {}
        impl PartialOrd for SchedulerMessage {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for SchedulerMessage {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                match (self, other) {
                    (Self::Update(_), Self::Rerender(_)) => std::cmp::Ordering::Greater,
                    (Self::Rerender(_), Self::Update(_)) => std::cmp::Ordering::Less,
                    (Self::Update(_), Self::Update(_)) => std::cmp::Ordering::Equal,
                    (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                        s_msg.depth.cmp(&o_msg.depth).reverse()
                    }
                }
            }
        }
        pub const SCHEDULER_INSTANCE: ::std::thread::LocalKey<RefCell<Scheduler>> = {
            #[inline]
            fn __init() -> RefCell<Scheduler> {
                RefCell::new(Scheduler::new())
            }
            #[inline]
            unsafe fn __getit(
                init: ::std::option::Option<
                    &mut ::std::option::Option<RefCell<Scheduler>>,
                >,
            ) -> ::std::option::Option<&'static RefCell<Scheduler>> {
                #[thread_local]
                static __KEY: ::std::thread::local_impl::Key<RefCell<Scheduler>> = ::std::thread::local_impl::Key::<
                    RefCell<Scheduler>,
                >::new();
                #[allow(unused_unsafe)]
                unsafe {
                    __KEY
                        .get(move || {
                            if let ::std::option::Option::Some(init) = init {
                                if let ::std::option::Option::Some(value) = init.take() {
                                    return value;
                                } else if true {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "internal error: entered unreachable code: {0}",
                                                format_args!("missing default value"),
                                            ),
                                        );
                                    };
                                }
                            }
                            __init()
                        })
                }
            }
            unsafe { ::std::thread::LocalKey::new(__getit) }
        };
        pub struct Scheduler {
            messages: BinaryHeap<SchedulerMessage>,
            is_handle_messages_scheduled: bool,
        }
        impl Scheduler {
            fn new() -> Self {
                Self {
                    messages: BinaryHeap::new(),
                    is_handle_messages_scheduled: false,
                }
            }
            fn schedule_handle_messages(&mut self) {
                if !self.is_handle_messages_scheduled {
                    self.is_handle_messages_scheduled = true;
                    spawn_local(async {
                        Scheduler::handle_messages();
                    });
                }
            }
            fn handle_messages() {
                let scheduler_messages: Vec<SchedulerMessage> = SCHEDULER_INSTANCE
                    .with(|scheduler| {
                        let mut scheduler = scheduler.borrow_mut();
                        let messages = scheduler.messages.drain().collect();
                        scheduler.is_handle_messages_scheduled = false;
                        messages
                    });
                for scheduler_message in scheduler_messages {
                    scheduler_message.handle();
                }
            }
            pub fn add_update_message(
                component: Rc<RefCell<Box<dyn AnyComponent>>>,
                message: Box<dyn Any>,
                to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
            ) {
                let message = SchedulerMessage::Update(UpdateMessage {
                    component,
                    message,
                    to_rerender_observer,
                });
                Self::add_message(message);
            }
            pub fn add_rerender_message(
                component: Rc<RefCell<Box<dyn AnyComponent>>>,
                behavior: Rc<AnyComponentBehavior>,
                vdom_observer: Rc<RefCell<VDomObserver>>,
                to_rerender: Rc<RefCell<bool>>,
                depth: u32,
            ) {
                let message = SchedulerMessage::Rerender(RerenderMessage {
                    component,
                    behavior,
                    vdom_observer,
                    to_rerender,
                    depth,
                });
                Self::add_message(message);
            }
            fn add_message(message: SchedulerMessage) {
                SCHEDULER_INSTANCE
                    .with(|scheduler| {
                        let mut scheduler = scheduler.borrow_mut();
                        scheduler.messages.push(message);
                        scheduler.schedule_handle_messages();
                    });
            }
        }
    }
    pub trait Component: Sized {
        type Message: 'static;
        type Properties: Hash + 'static;
        fn new(props: Self::Properties) -> Self;
        fn view(&self, behavior: &mut ComponentBehavior<Self>) -> VNode;
        fn update(&mut self, message: Self::Message) -> bool;
    }
    pub trait AnyComponent {
        fn new(props: Box<dyn Any>) -> Self
        where
            Self: Sized;
        fn view(&self, behavior: &AnyComponentBehavior) -> VNode;
        fn update(&mut self, message: Box<dyn Any>) -> bool;
    }
    impl<C: Component> AnyComponent for C {
        fn new(props: Box<dyn Any>) -> Self {
            let props = *props
                .downcast::<C::Properties>()
                .expect(
                    "Failed to downcast properties in any component to properties of a real component",
                );
            C::new(props)
        }
        fn view(&self, any_component_behavior: &AnyComponentBehavior) -> VNode {
            let mut component_behavior = ComponentBehavior::from(any_component_behavior);
            self.view(&mut component_behavior)
        }
        fn update(&mut self, message: Box<dyn Any>) -> bool {
            let msg = *message
                .downcast::<C::Message>()
                .expect(
                    "Failed to downcast message in any component to message of a real component",
                );
            self.update(msg)
        }
    }
}
pub mod app {
    use crate::{
        component::{component_node::AnyComponentNode, Component},
        virtual_dom::Dom,
    };
    pub fn start<C: Component + 'static>(root_component: C) {
        let ancestor = Dom::get_root_element();
        AnyComponentNode::new_root(root_component, ancestor);
    }
}
