use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Rc};

use gloo::console::log;
use wasm_bindgen_futures::spawn_local;

use super::{
    component::AnyComponent,
    component_node::{AnyComponentBehavior, ToRerenderObserver, VDomObserver},
};

enum SchedulerMessage {
    Update(UpdateMessage),
    Rerender(RerenderMessage),
}

struct UpdateMessage {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    message: Box<dyn Any>,
    to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl UpdateMessage {
    fn handle(self) {
        log!("UpdateMessage handle1");
        let to_rerender = self.component.borrow_mut().update(self.message);
        log!("UpdateMessage handle2");
        if to_rerender {
            log!("UpdateMessage handle3");
            self.to_rerender_observer.borrow().notify();
            log!("UpdateMessage handle4");
        }
        log!("UpdateMessage handle5");
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
        log!("RerenderMessage handle1");
        let vdom = self.component.borrow().view(self.behavior.as_ref());
        log!("RerenderMessage handle2");
        let vdom_observer = self.vdom_observer.borrow();
        log!("RerenderMessage handle3");
        vdom_observer.notify(vdom);
        log!("RerenderMessage handle4");
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

#[derive(Default)]
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

    pub fn handle_messages() {
        log!("Handling messages1");
        let scheduler_messages: Vec<SchedulerMessage> = SCHEDULER_INSTANCE.with(|scheduler| {
            log!("Handling messages2");
            let mut scheduler = scheduler.borrow_mut();
            log!("Handling messages3");
            let messages = scheduler.messages.drain().collect();
            log!("Handling messages4");
            scheduler.is_handle_messages_scheduled = false;
            log!("Handling messages5");
            messages
        });
        log!("Hangdling massage6");
        for scheduler_message in scheduler_messages {
            log!("handling messages for");
            match scheduler_message {
                SchedulerMessage::Update(update_message) => {
                    update_message.handle();
                }
                SchedulerMessage::Rerender(rerender_message) => {
                    rerender_message.handle();
                }
            }
        }
    }

    fn add_message(&mut self, message: SchedulerMessage) {
        self.messages.push(message);
        self.schedule_handle_messages();
    }

    fn schedule_handle_messages(&mut self) {
        if !self.is_handle_messages_scheduled {
            self.is_handle_messages_scheduled = true;
            spawn_local(async {
                Scheduler::handle_messages();
            });
        }
    }

    pub fn add_update_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        message: Box<dyn Any>,
        to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    ) {
        log!("Adding update message1");
        SCHEDULER_INSTANCE.with(|scheduler| {
            log!("Adding update message2");
            let message = SchedulerMessage::Update(UpdateMessage {
                component,
                message,
                to_rerender_observer,
            });
            log!("Adding update message3");
            scheduler.borrow_mut().add_message(message);
            log!("Adding update message4");
        });
        log!("Adding update message5");
    }

    pub fn add_rerender_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        behavior: Rc<AnyComponentBehavior>,
        vdom_observer: Rc<RefCell<VDomObserver>>,
        to_rerender: Rc<RefCell<bool>>,
        depth: u32,
    ) {
        log!("add rerender message1");
        SCHEDULER_INSTANCE.with(|scheduler| {
            log!("add rerender message2");
            let message = SchedulerMessage::Rerender(RerenderMessage {
                component,
                behavior,
                vdom_observer,
                to_rerender,
                depth,
            });
            log!("add rerender message3");
            scheduler.borrow_mut().add_message(message);
            log!("add rerender message4");
        });

        log!("add rerender message5");
    }
}
