use super::{
    callback::Callback,
    component::{Component, DynamicComponent},
    messeage_queue::MessageQueue,
};

pub struct ContextNode<'a> {
    component: Box<dyn DynamicComponent>, //Component<'a>,
    message_queue: MessageQueue,
    children: Vec<Box<dyn DynamicComponent>>,
}

impl<'a> ContextNode<'a> {
    pub fn new(component: Component) -> ContextNode<'a> {
        ContextNode {
            component,
            message_queue: MessageQueue::new(component),
            children: Vec::new(),
        }
    }

    pub fn create_callback<IN, F, C: Component>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        Callback::new(move |data| {
            let message = wrapper(data);
            self.message_queue.add_messeage(message);
        })
    }
}
