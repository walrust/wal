use super::{AnyStore, Store};
use crate::component::Component;
use std::any::Any;

pub type Subscriptions = Option<Vec<Box<dyn AnySubscription>>>;

pub struct Subscription<T: Copy, C: Component> {
    store: Store<T>,
    messeage_generator: dyn Fn(T) -> C::Message,
}

pub trait AnySubscription {
    fn get_store(&self) -> Box<dyn AnyStore>;
    fn get_messeage(&self, payload: Box<dyn Any>) -> Box<dyn Any>;
}

impl<T, C> AnySubscription for Subscription<T, C>
where
    T: Copy + 'static,
    C: Component,
{
    fn get_store(&self) -> Box<dyn AnyStore> {
        Box::new(self.store)
    }

    fn get_messeage(&self, payload: Box<dyn Any>) -> Box<dyn Any> {
        let payload = *payload
            .downcast::<T>()
            .expect("Failed to downcast payload in AnySubscription to payload of a real messeage");
        Box::new((self.messeage_generator)(payload))
    }
}
