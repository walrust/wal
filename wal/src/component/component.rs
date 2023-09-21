use crate::virtual_dom::VNode;
use std::any::Any;
use std::hash::Hash;

use super::context_node::ContextNode;

pub trait Component {
    type Message;
    type Properties: Hash;

    fn new(props: Self::Properties) -> Self;
    fn view(&self, context_node: &mut ContextNode) -> VNode;
    fn update(&mut self, message: Self::Message) -> bool;
}

pub trait DynamicComponent {
    fn message_boxed(&self) -> Box<dyn Any>;
}

#[cfg(test)]
mod tests {
    use wal_macros::html;

    use crate::virtual_dom::VNode;

    use super::{
        callback::Callback,
        context_node::{self, ContextNode},
        Component,
    };
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

        fn view(&self, context_node: &mut ContextNode) -> VNode {
            let callback1: Callback<_> =
                context_node.create_callback::<Self>(MyMessage::ChangeMyMother);
            let callback2: Callback<_> =
                context_node.create_callback::<Self>(|_x: i64| MyMessage::ChangeMyFather(_x));

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

// ======================================== Pomysl mandero ========================================

// trait Component {
//     type Message;

//     fn message(&self) -> Self::Message;
// }

// trait DynamicComponent {
//     fn message_boxed(&self) -> Box<dyn Any>;
// }

// impl<T: Component + 'static> DynamicComponent for T {
//     fn message_boxed(&self) -> Box<dyn Any> {
//         Box::new(self.message()) as Box<dyn Any>
//     }
// }

// struct ComponentA;

// impl Component for ComponentA {
//     type Message = String;

//     fn message(&self) -> Self::Message {
//         "ComponentA Message".to_string()
//     }
// }

// struct ComponentB;

// impl Component for ComponentB {
//     type Message = i32;

//     fn message(&self) -> Self::Message {
//         42
//     }
// }

// fn main() {
//     let mut components: Vec<Box<dyn DynamicComponent>> = Vec::new();

//     components.push(Box::new(ComponentA));
//     components.push(Box::new(ComponentB));

// }
