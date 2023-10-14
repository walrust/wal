use crate::virtual_dom::VNode;
use std::any::Any;
use std::hash::Hash;

use super::component_node::{AnyComponentBehavior, ComponentBehavior};

pub trait Component: Sized {
    type Message: 'static;
    type Properties: Hash + 'static;

    fn new(props: Self::Properties) -> Self;
    fn view(&self, behavior: &mut ComponentBehavior<Self>) -> VNode;
    fn update(&mut self, message: Self::Message) -> bool;
}

pub trait AnyComponent {
    fn new(props: Box<dyn Any>) -> Self
    where
        Self: Sized;
    fn view(&self, behavior: &AnyComponentBehavior) -> VNode;
    fn update(&mut self, message: Box<dyn Any>) -> bool;
}

impl<C: Component> AnyComponent for C {
    fn new(props: Box<dyn Any>) -> Self {
        let props = *props.downcast::<C::Properties>().expect(
            "Failed to downcast properties in any component to properties of a real component",
        );
        C::new(props)
    }

    fn view(&self, any_component_behavior: &AnyComponentBehavior) -> VNode {
        let mut component_behavior = ComponentBehavior::from(any_component_behavior);

        self.view(&mut component_behavior)
    }

    fn update(&mut self, message: Box<dyn Any>) -> bool {
        let msg = *message
            .downcast::<C::Message>()
            .expect("Failed to downcast message in any component to message of a real component");
        self.update(msg)
    }
}
