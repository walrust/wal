use crate::{component::{
    component::Component, component_node::AnyComponentNode, scheduler::Scheduler,
}, virtual_dom::Dom};

pub struct App {
    root_component_node: AnyComponentNode,
}

impl App {
    pub fn new<C: Component + 'static>(root_component: C) -> Self {
        Self {
            root_component_node: AnyComponentNode::new(root_component, Dom::get_root_element()),
        }
    }
}

pub fn start<C: Component + 'static>(root_component: C) {
    let app = App::new(root_component);
    Scheduler::event_loop();
}
