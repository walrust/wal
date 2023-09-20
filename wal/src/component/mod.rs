pub mod callback;
pub mod any_props;

use std::hash::Hash;
use crate::virtual_dom::VNode;
use callback::Callback;


pub trait Component {
   type Message;
   type Properties: Hash;

    fn new(props: Self::Properties) -> Self;
    fn view(&self) -> VNode;
    fn update(&mut self, message: Self::Message) -> bool;

    fn create_callback<IN, F>(wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> Self::Message + 'static,
    {
        Callback::new(move |data| {
            let _message = wrapper(data);
            todo!("send message to a queue, which will be processed later - meaning an update method will be called")
        })
    }
}


#[cfg(test)]
mod tests {
}
