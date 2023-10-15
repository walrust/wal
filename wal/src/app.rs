use crate::{
    component::{component::Component, component_node::AnyComponentNode, scheduler::Scheduler},
    virtual_dom::Dom,
};

pub fn start<C: Component + 'static>(root_component: C) {
    AnyComponentNode::new(root_component, Dom::get_root_element());
    Scheduler::event_loop();
}
