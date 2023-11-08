use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Weak};
use wasm_bindgen_futures::spawn_local;
use super::{
    behavior::AnyComponentBehavior,
    observer::{ToRerenderObserver, VDomObserver},
    AnyComponent,
};
use crate::utils::debug;
use super::component_node::AnyComponentNode;

enum SchedulerMessage {
    Update(UpdateMessage),
    Rerender(RerenderMessage),
}

impl SchedulerMessage {
    fn handle(self) {
        match self {
            SchedulerMessage::Update(msg) => msg.handle(),
            SchedulerMessage::Rerender(msg) => msg.handle(),
        }
    }
}

struct UpdateMessage {
    message: Box<dyn Any>,
    any_component_node: Weak<RefCell<AnyComponentNode>>,
}

impl UpdateMessage {
    fn handle(self) {
        if let Some(any_component_node) = self.any_component_node.upgrade() {
            let to_rerender = any_component_node.borrow_mut().update(self.message);
            if to_rerender {
                Scheduler::add_rerender_message(
                    self.any_component_node,
                    any_component_node.borrow().depth,
                );
            }
        } else {
            debug::log("Weak reference to AnyComponentNode is not attached to AnyComponentNode");
        }
    }
}

struct RerenderMessage {
    any_component_node: Weak<RefCell<AnyComponentNode>>,
    depth: u32,
}

impl RerenderMessage {
    fn handle(self) {
        if let Some(any_component_node) = self.any_component_node.upgrade() {
            any_component_node.borrow_mut().view_and_patch();
        } else {
            debug::log("Weak reference to AnyComponentNode is not attached to AnyComponentNode");
        }
    }
}

impl PartialEq for SchedulerMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Update(s_msg), Self::Update(o_msg)) => {
                Weak::ptr_eq(&s_msg.any_component_node, &o_msg.any_component_node)
                    && &s_msg.message as *const dyn Any == &o_msg.message as *const dyn Any
            }
            (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                Weak::ptr_eq(&s_msg.any_component_node, &o_msg.any_component_node)
                    && s_msg.depth == o_msg.depth
            }
            _ => false,
        }
    }
}

impl Eq for SchedulerMessage {}

impl PartialOrd for SchedulerMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SchedulerMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Update(_), Self::Rerender(_)) => std::cmp::Ordering::Greater,
            (Self::Rerender(_), Self::Update(_)) => std::cmp::Ordering::Less,
            (Self::Update(_), Self::Update(_)) => std::cmp::Ordering::Equal,
            (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                s_msg.depth.cmp(&o_msg.depth).reverse()
            }
        }
    }
}

thread_local! {
    pub static SCHEDULER_INSTANCE: RefCell<Scheduler> = RefCell::new(Scheduler::new());
}

pub struct Scheduler {
    messages: BinaryHeap<SchedulerMessage>,
    is_handle_messages_scheduled: bool,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            messages: BinaryHeap::new(),
            is_handle_messages_scheduled: false,
        }
    }

    fn schedule_handle_messages(&mut self) {
        if !self.is_handle_messages_scheduled {
            self.is_handle_messages_scheduled = true;
            spawn_local(async {
                Scheduler::handle_messages();
            });
        }
    }

    fn handle_messages() {
        let scheduler_messages: Vec<SchedulerMessage> = SCHEDULER_INSTANCE.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            let messages = scheduler.messages.drain().collect();
            scheduler.is_handle_messages_scheduled = false;
            messages
        });

        for scheduler_message in scheduler_messages {
            scheduler_message.handle();
        }
    }

    pub fn add_update_message(
        message: Box<dyn Any>,
        any_component_node: Weak<RefCell<AnyComponentNode>>,
    ) {
        let message = SchedulerMessage::Update(UpdateMessage {
            message,
            any_component_node,
        });
        Self::add_message(message);
    }

    pub fn add_rerender_message(any_component_node: Weak<RefCell<AnyComponentNode>>, depth: u32) {
        let message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node,
            depth,
        });
        Self::add_message(message);
    }

    fn add_message(message: SchedulerMessage) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            scheduler.messages.push(message);
            scheduler.schedule_handle_messages();
        });
    }
}
