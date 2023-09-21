// use std::path::Component;
// use super::{messeage_queue::MessageQueue, callback::Callback};

// pub struct ContextNode<'a>{
//     component: Component<'a>,
//     message_queue: MessageQueue<???>,
//     children: Vec<ContextNode>,
//     parent: Option<ContextNode>
// }

// impl <'a> ContextNode<'a>{

//     pub fn new(parent: Option<ContextNode>, component: Component, ) -> ContextNode<'a> {
//         let mut new_node = ContextNode {
//             component,
//             message_queue: MessageQueue::new(component),
//             children: Vec::new(),
//             parent
//         };

//         if let Some(p) = parent {
//             p.children.push_back(new_node);
//         };
//         new_node
//     }

//     pub fn emit_callback(&mut self, callback: Callback<_>) {
//         if if let Some(p) = self.parent {
//             p.message_queue.add_messeage(???)
//         };
//     }

// }
