use crate::{
    component::{component::Component, component_node::AnyComponentNode, scheduler::Scheduler},
    virtual_dom::Dom,
};

pub fn start<C: Component + 'static>(root_component: C) {
    let ancestor = Dom::get_root_element();
    AnyComponentNode::new_root(root_component, ancestor);
    Scheduler::event_loop();
}
