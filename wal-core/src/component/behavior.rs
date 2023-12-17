use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{callback::Callback, node::AnyComponentNode, scheduler::Scheduler, Component};

pub(crate) struct AnyComponentBehavior {
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

/// Behavior is a trait that is used to create [callbacks](Callback) that are responsible to send [Messages](Component::Message) to the [Component](Component).
pub trait Behavior<C: Component> {
    /// Creates a [callback](Callback) that is responsible to send [Messages](Component::Message) to the [Component](Component).
    /// Created [callback](Callback) will send the message to the [Component](Component) that the [Behavior](Behavior) is defined for.
    /// If the motivation is to send the message to the parent [Component](Component) [new](Callback::new) function should be used.
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
