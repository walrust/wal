extern crate console_error_panic_hook;
use std::{cell::RefCell, rc::Rc};

use crate::{
    component::{component_node::AnyComponentNode, Component},
    virtual_dom::Dom,
};

thread_local! {
    pub static ROOT_INSTANCE: RefCell<Option<Rc<RefCell<AnyComponentNode>>>> = RefCell::new(None);
}

pub fn start<C: Component + 'static>(root_component: C) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let ancestor = Dom::get_root_element();
    let root = AnyComponentNode::new(root_component, ancestor);

    ROOT_INSTANCE.with(move |root_instance| {
        *root_instance.borrow_mut() = Some(root);
    })
}
