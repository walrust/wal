extern crate console_error_panic_hook;
use crate::{
    component::{component_node::AnyComponentNode, Component},
    virtual_dom::Dom,
};

// TODO here maybe we should have a static object that is the root of the application so that Weaks dont get dropped
pub fn start<C: Component + 'static>(root_component: C) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let ancestor = Dom::get_root_element();
    AnyComponentNode::new(root_component, ancestor);
}
