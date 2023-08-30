pub mod velement;
pub mod vlist;
pub mod vnode;
pub mod vtext;

use gloo::console::__macro::JsValue;
use web_sys::{Element, Node};

pub use self::velement::VElement;
pub use self::vlist::VList;
pub use self::vnode::VNode;
pub use self::vtext::VText;

pub fn mount(node: &Node, target: &Element) -> Result<(), JsValue> {
    target.replace_with_with_node_1(&node)?;
    Ok(())
}
