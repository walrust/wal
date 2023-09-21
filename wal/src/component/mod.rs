pub mod any_props;
pub mod callback;
pub mod messeage_queue;

use crate::virtual_dom::VNode;
use callback::Callback;
use std::hash::Hash;

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
    use wal_macros::html;

    use crate::virtual_dom::VNode;

    use super::{callback::Callback, Component};
    struct MyChild;
    impl Component for MyChild {
        type Message = ();
        type Properties = MyChildProps;
        fn new(props: Self::Properties) -> Self {
            todo!()
        }
        fn view(&self) -> VNode {
            todo!()
        }
        fn update(&mut self, message: Self::Message) -> bool {
            todo!()
        }
    }

    #[derive(Hash)]
    struct MyProps;
    #[derive(Hash)]
    struct MyChildProps {
        onclick1: Callback<i64>,
        onclick2: Callback<i64>,
    }

    enum MyMessage {
        ChangeMyMother(i64),
        ChangeMyFather(i64),
    }

    struct MyComponent;
    impl Component for MyComponent {
        type Message = MyMessage;
        type Properties = MyProps;
        fn new(_props: Self::Properties) -> Self {
            todo!()
        }

        fn view(&self) -> VNode {
            let callback1: Callback<_> = Self::create_callback(MyMessage::ChangeMyMother);
            let callback2: Callback<_> =
                Self::create_callback(|_x: i64| MyMessage::ChangeMyFather(_x));

            html! {
               <MyChild props={ MyChildProps { onclick1: callback1, onclick2: callback2}} />
            }

            todo!()
        }

        fn update(&mut self, message: Self::Message) -> bool {
            match message {
                MyMessage::ChangeMyMother(_) => todo!(),
                MyMessage::ChangeMyFather(_) => todo!(),
            }
            todo!()
        }
    }
}
