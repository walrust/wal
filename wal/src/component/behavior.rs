use std::{rc::Rc, cell::RefCell};

use super::{AnyComponent, observer::ToRerenderObserver, Component, callback::Callback, scheduler::Scheduler};

pub struct AnyComponentBehavior {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl AnyComponentBehavior {
    pub fn new(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    ) -> Self {
        Self {
            component,
            rerender_observer,
        }
    }
}

pub trait Behavior<C: Component> {
    fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static;
}

impl<C: Component> Behavior<C> for Rc<AnyComponentBehavior> {
    fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let component = self.component.clone();
        let rerender_observer = self.rerender_observer.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            Scheduler::add_update_message(
                component.clone(),
                Box::new(message),
                rerender_observer.clone(),
            );
        })
    }
}