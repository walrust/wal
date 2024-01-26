#[doc(hidden)]
pub mod vcomponent;
#[doc(hidden)]
pub mod velement;
#[doc(hidden)]
pub mod vlist;
#[doc(hidden)]
pub mod vnode;
#[doc(hidden)]
pub mod vtext;

pub use self::vcomponent::VComponent;
pub use self::velement::VElement;
pub use self::vlist::VList;
pub use self::vnode::VNode;
pub use self::vtext::VText;

pub(crate) mod dom {
    use std::borrow::Cow;

    use gloo::events::EventListener;
    use gloo::utils::{body, document};
    use web_sys::{Element, Event, Node, Text};

    pub const ROOT_ELEMENT_ID: &str = "walrust-root";

    pub fn get_root_element() -> Node {
        Node::from(
            document()
                .get_element_by_id(ROOT_ELEMENT_ID)
                .unwrap_or_else(|| {
                    let root = document().create_element("div").unwrap();
                    set_attribute(&root, "id", ROOT_ELEMENT_ID);
                    append_child(&body(), &root);
                    root
                }),
        )
    }

    pub fn create_element(local_name: &str) -> Element {
        document()
            .create_element(local_name)
            .expect("Couldnt create new element")
    }

    pub fn create_text_node(data: &str) -> Text {
        document().create_text_node(data)
    }

    pub fn remove_node(node: &Node) {
        let ancestor = node.parent_node().expect("Node does not have a parent");
        self::remove_child(&ancestor, node);
    }

    pub fn append_child(ancestor: &Node, child: &Node) -> Node {
        ancestor
            .append_child(child)
            .expect("Couldnt append child to node")
    }

    pub fn replace_child(ancestor: &Node, old_child: &Node, child: &Node) -> Node {
        ancestor
            .replace_child(child, old_child)
            .expect("Couldnt replace child with a new node")
    }

    pub fn remove_child(ancestor: &Node, child: &Node) -> Node {
        ancestor.remove_child(child).expect("Couldnt remove child")
    }

    pub fn set_attribute(el: &Element, name: &str, value: &str) {
        el.set_attribute(name, value)
            .expect("Couldnt set attribute")
    }

    pub fn remove_attribute(el: &Element, name: &str) {
        el.remove_attribute(name).expect("Couldnt remove attribute")
    }

    pub fn create_event_listener<F>(
        element: &Element,
        event_type: Cow<'static, str>,
        callback: F,
    ) -> EventListener
    where
        F: FnMut(&Event) + 'static,
    {
        EventListener::new(element, event_type, callback)
    }
}
