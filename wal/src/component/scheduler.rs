use std::{any::Any, cell::RefCell, rc::Rc};

use super::{
    component::AnyComponent,
    component_node::{AnyComponentBehavior, ToRerenderObserver, VDomObserver},
    thread_safe_collections::ThreadSafePriorityQueue,
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
    priority_queue: ThreadSafePriorityQueue<SchedulerMessage>,
}

impl Scheduler {
    pub fn event_loop() {
        loop {
            SCHEDULER_INSTANCE.with(|scheduler| {
                let scheduler_message = scheduler.borrow_mut().priority_queue.pop();
                match scheduler_message {
                    SchedulerMessage::Update(update_message) => {
                        let to_rerender = update_message
                            .component
                            .borrow_mut()
                            .update(update_message.message);
                        if to_rerender {
                            update_message.to_rerender_observer.borrow().notify();
                        }
                    }
                    SchedulerMessage::Rerender(rerender_message) => {
                        let vdom = rerender_message
                            .component
                            .borrow()
                            .view(rerender_message.behavior.as_ref());
                        rerender_message.vdom_observer.borrow_mut().notify(vdom);
                    }
                }
            });
        }
    }

    pub fn add_update_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        message: Box<dyn Any>,
        to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    ) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            let message = SchedulerMessage::Update(UpdateMessage {
                component,
                message,
                to_rerender_observer,
            });
            scheduler.borrow_mut().priority_queue.push(message);
        });
    }

    pub fn add_rerender_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        behavior: Rc<AnyComponentBehavior>,
        vdom_observer: Rc<RefCell<VDomObserver>>,
        depth: u32,
    ) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            let message = SchedulerMessage::Rerender(RerenderMessage {
                component,
                behavior,
                vdom_observer,
                depth,
            });
            scheduler.borrow_mut().priority_queue.push(message);
        });
    }
}
