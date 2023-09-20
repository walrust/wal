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
    use crate::virtual_dom::VNode;

    use super::{Component, callback::Callback};
    struct MyChild;
    impl Component for MyChild {
        type Message=();
        type Properties=MyChildProps;
        fn new(props: Self::Properties) -> Self { todo!() }
        fn view(&self) -> VNode { todo!() }
        fn update(&mut self, message: Self::Message) -> bool { todo!() }
    }

    #[derive(Hash)]
    struct MyProps;
    #[derive(Hash)]
    struct MyChildProps {
        onclick: Callback<i64>,
    }

    enum MyMessage {
        ChangeMyMother(i64),
        ChangeMyFather(i64),
    }    

    struct MyComponent;
    impl Component for MyComponent {
        type Message=MyMessage;
        type Properties=MyProps;
        fn new(_props: Self::Properties) -> Self {
            todo!()
        }

        fn view(&self) -> VNode {
            let callback: Callback<_> = 
            if true {
                Self::create_callback(MyMessage::ChangeMyMother)
            } else {
                Self::create_callback(MyMessage::ChangeMyFather)
            };

            //html! {
            //    <MyChild props={ MyChildProps { onclick: callback }} />
            //}
            

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
