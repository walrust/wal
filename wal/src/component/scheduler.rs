use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Rc};

use gloo::console::log;

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

struct RerenderMessage {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    behavior: Rc<AnyComponentBehavior>,
    vdom_observer: Rc<RefCell<VDomObserver>>,
    depth: u32,
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
    pub static SCHEDULER_INSTANCE: RefCell<Scheduler> = Default::default();
}

#[derive(Default)]
pub struct Scheduler {
    messages: BinaryHeap<SchedulerMessage>,
}

impl Scheduler {
    pub fn handle_messages() {
        log!("Handling messages-1");
        let scheduler_messages: Vec<SchedulerMessage> = SCHEDULER_INSTANCE.with(|scheduler| {
            log!("Handling messages-2");
            let messages = scheduler.borrow_mut().messages.drain().collect();
            log!("Handling messages-3");
            messages
        });
        log!("Hangdling massage0");
        for scheduler_message in scheduler_messages {
            log!("handling messages for");
            match scheduler_message {
                SchedulerMessage::Update(update_message) => {
                    log!("Handling update message1");
                    let to_rerender = update_message
                        .component
                        .borrow_mut()
                        .update(update_message.message);
                    log!("Handling update message2");
                    if to_rerender {
                        log!("Handling update message3");
                        update_message.to_rerender_observer.borrow().notify();
                        log!("Handling update message4");
                    }
                    log!("Handling update message5");
                }
                SchedulerMessage::Rerender(rerender_message) => {
                    log!("handle rerender message1");
                    let vdom = rerender_message
                        .component
                        .borrow()
                        .view(rerender_message.behavior.as_ref());
                    log!("handle rerender message2");
                    let vdom_observer = rerender_message.vdom_observer.borrow();
                    log!("handle rerender message2.5");
                    vdom_observer.notify(vdom);
                    log!("handle rerender message3");
                }
            }
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
            scheduler.borrow_mut().messages.push(message);
            log!("Adding update message4");
        });
        log!("Adding update message5");
        Scheduler::handle_messages();
    }

    pub fn add_rerender_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        behavior: Rc<AnyComponentBehavior>,
        vdom_observer: Rc<RefCell<VDomObserver>>,
        depth: u32,
    ) {
        log!("add rerender message1");
        SCHEDULER_INSTANCE.with(|scheduler| {
            log!("add rerender message2");
            let message = SchedulerMessage::Rerender(RerenderMessage {
                component,
                behavior,
                vdom_observer,
                depth,
            });
            log!("add rerender message3");
            let mut scheduler = scheduler.borrow_mut();
            log!("add rerender message3.5");
            scheduler.messages.push(message);
            log!("add rerender message4");
        });

        log!("add rerender message5");
        Scheduler::handle_messages();
    }
}
