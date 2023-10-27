use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{
    callback::Callback, component_node::AnyComponentNode, scheduler::Scheduler, Component,
};

pub struct AnyComponentBehavior {
    any_component_node: Weak<RefCell<AnyComponentNode>>,
}

impl AnyComponentBehavior {
    pub(crate) fn new() -> Self {
        Self {
            any_component_node: Weak::new(),
        }
    }

    pub(crate) fn set_any_component_node(
        &mut self,
        any_component_node: Rc<RefCell<AnyComponentNode>>,
    ) {
        self.any_component_node = Rc::downgrade(&any_component_node);
    }
}

pub trait Behavior<C: Component> {
    fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static;
}

impl<C: Component> Behavior<C> for AnyComponentBehavior {
    fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let any_component_node = self.any_component_node.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            Scheduler::add_update_message(Box::new(message), any_component_node.clone());
        })
    }
}
