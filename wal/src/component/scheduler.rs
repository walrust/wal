use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Rc};

use wasm_bindgen_futures::spawn_local;

use super::{
    component_node::{ToRerenderObserver, VDomObserver},
    AnyComponent, behavior::AnyComponentBehavior,
};

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
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    message: Box<dyn Any>,
    to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl UpdateMessage {
    fn handle(self) {
        let to_rerender = self.component.borrow_mut().update(self.message);
        if to_rerender {
            self.to_rerender_observer.borrow().notify();
        }
    }
}

struct RerenderMessage {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    behavior: Rc<AnyComponentBehavior>,
    vdom_observer: Rc<RefCell<VDomObserver>>,
    to_rerender: Rc<RefCell<bool>>,
    depth: u32,
}

impl RerenderMessage {
    fn handle(self) {
        let vdom = self.component.borrow().view(self.behavior.as_ref());
        self.vdom_observer.borrow().notify(vdom);
        *self.to_rerender.borrow_mut() = false;
    }
}

impl PartialEq for SchedulerMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Update(s_msg), Self::Update(o_msg)) => {
                Rc::ptr_eq(&s_msg.component, &o_msg.component)
                    && &s_msg.message as *const dyn Any == &o_msg.message as *const dyn Any
                    && Rc::ptr_eq(&s_msg.to_rerender_observer, &o_msg.to_rerender_observer)
            }
            (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                Rc::ptr_eq(&s_msg.component, &o_msg.component)
                    && Rc::ptr_eq(&s_msg.behavior, &o_msg.behavior)
                    && Rc::ptr_eq(&s_msg.vdom_observer, &o_msg.vdom_observer)
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
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        message: Box<dyn Any>,
        to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    ) {
        let message = SchedulerMessage::Update(UpdateMessage {
            component,
            message,
            to_rerender_observer,
        });
        Self::add_message(message);
    }

    pub fn add_rerender_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        behavior: Rc<AnyComponentBehavior>,
        vdom_observer: Rc<RefCell<VDomObserver>>,
        to_rerender: Rc<RefCell<bool>>,
        depth: u32,
    ) {
        let message = SchedulerMessage::Rerender(RerenderMessage {
            component,
            behavior,
            vdom_observer,
            to_rerender,
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
