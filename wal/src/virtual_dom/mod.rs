pub mod vcomponent;
pub mod velement;
pub mod vlist;
pub mod vnode;
pub mod vtext;

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
    pub fn get_root_element() -> Node {
        Node::from(
            document()
                .get_element_by_id("walrust-root")
                .unwrap_or_else(|| {
                    log!("** There was no 'walrust-root' element, adding default one");
                    let root = document().create_element("div").unwrap();
                    Dom::set_attribute(&root, "id", "walrust-root");
                    Dom::append_child(&body(), &root);
                    root
                }),
        )
    }
    pub fn create_element(local_name: &str) -> Element {
        document()
            .create_element(local_name)
            .expect("Couldnt create new element")
    }
    pub fn create_text_node(data: &String) -> Text {
        document().create_text_node(data.as_str())
    }
    pub fn append_child(ancestor: &Node, child: &Node) -> Node {
        ancestor
            .append_child(child)
            .expect("Couldnt append child to node")
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
        el.set_attribute(name, value)
            .expect("Couldnt set attribute")
    }
    pub fn remove_attribute(el: &Element, name: &str) {
        el.remove_attribute(name).expect("Couldnt remove attribute")
    }
}
