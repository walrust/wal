pub mod vnode;
pub mod velement;
pub mod vtext;

use gloo::console::__macro::JsValue;
use web_sys::{Node, Element};

pub use self::vnode::VNode;
pub use self::velement::VElement;
pub use self::vtext::VText;



pub fn mount(node: &Node, target: &Element) -> Result<(), JsValue> {
    target.replace_with_with_node_1(&node)?;
    Ok(())
}