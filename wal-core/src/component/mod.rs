//! This module provides implementations of [Behavior](./behavior/trait.Behavior.html) trait and [Callbacks](./callback/struct.Callback.html) structs.

use crate::virtual_dom::VNode;
use std::any::Any;
use std::hash::Hash;

use self::behavior::{AnyComponentBehavior};

#[doc(hidden)]
pub mod behavior;
#[doc(hidden)]
pub mod callback;

pub(crate) mod node;
pub(crate) mod scheduler;

pub use self::behavior::Behavior;
pub use self::callback::Callback;

/// Trait for defining custom component.
///
/// It adapts a MVC pattern.
/// Model is the fields and state of the component. It is initialized by [new](#tymethod.new) function using [Properties](#associatedtype.Properties).
/// View is the function [view](#tymethod.view) that returns a view of an application.
/// Controller is the function [update](#tymethod.update) that updates the model based on the [Message](#associatedtype.Message).
pub trait Component: Sized {
    /// Type to describe the message that can be sent to the component.
    /// It is used to update the model of the component.
    type Message: 'static;

    /// Type to describe the properties that can be passed to the component.
    /// It is used to initialize the model of the component.
    type Properties: Hash + 'static;

    /// Function that creates a new instance of the component therefore initialize a model using [Properties](#associatedtype.Properties).
    fn new(props: Self::Properties) -> Self;

    /// Function that returns a view of the component.
    /// It uses [Behavior](behavior::Behavior) to create [Callbacks](callback::Callback) that are responsible to send [Messages](#associatedtype.Message) to the component.
    /// Defining the views is done using [rsx](../../wal_rsx/macro.rsx.html) macro.
    /// Using the macro is not mandatory but it is recommended and makes in unnecessary to know the complex structure of [VNode].
    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode;

    /// Function that updates the model of the component based on the [Message](#associatedtype.Message).
    /// It returns a boolean that indicates if the rerender of the component is necessary.
    /// Meaning whether the view of the component should be updated or not.
    fn update(&mut self, message: Self::Message) -> bool;
}

pub(crate) trait AnyComponent {
    fn new(props: Box<dyn Any>) -> Self
    where
        Self: Sized;
    fn view(&self, behavior: &mut AnyComponentBehavior) -> VNode;
    fn update(&mut self, message: Box<dyn Any>) -> bool;
}

impl<C: Component> AnyComponent for C {
    fn new(props: Box<dyn Any>) -> Self {
        let props = *props.downcast::<C::Properties>().expect(
            "Failed to downcast properties in any component to properties of a real component",
        );
        C::new(props)
    }

    fn view(&self, any_component_behavior: &mut AnyComponentBehavior) -> VNode {
        self.view(any_component_behavior)
    }

    fn update(&mut self, message: Box<dyn Any>) -> bool {
        let msg = *message
            .downcast::<C::Message>()
            .expect("Failed to downcast message in any component to message of a real component");
        self.update(msg)
    }
}
