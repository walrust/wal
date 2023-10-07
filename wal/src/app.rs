use crate::component::{
    component::Component, context_node::AnyComponentNode, scheduler::Scheduler,
};

pub struct App {
    root_component_node: AnyComponentNode,
}

impl App {
    pub fn new<C: Component + 'static>(root_component: C) -> Self {
        Self {
            root_component_node: AnyComponentNode::new(root_component),
        }
    }
}

pub fn start<C: Component + 'static>(root_component: C) {
    let app = App::new(root_component);
    todo!("create dom");
    Scheduler::event_loop();
}
