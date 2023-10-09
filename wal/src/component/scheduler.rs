use std::{any::Any, cell::RefCell, rc::Rc};

use super::{
    component::AnyComponent,
    context_node::{AnyComponentBehavior, Observer, RerenderObserver},
    thread_safe_collections::ThreadSafePriorityQueue,
};

enum SchedulerMessage {
    Update(UpdateMessage),
    Rerender(RerenderMessage),
}

struct UpdateMessage {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    message: Box<dyn Any>,
    rerender_observer: Rc<RefCell<RerenderObserver>>,
}

struct RerenderMessage {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    behavior: Rc<AnyComponentBehavior>,
    depth: u32,
}

impl PartialEq for SchedulerMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Update(_), Self::Update(_)) => true,
            (Self::Rerender(_), Self::Rerender(_)) => true,
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
            (Self::Update(_), Self::Rerender(_)) => std::cmp::Ordering::Less,
            (Self::Rerender(_), Self::Update(_)) => std::cmp::Ordering::Greater,
            (Self::Update(_), Self::Update(_)) => std::cmp::Ordering::Equal,
            (Self::Rerender(_), Self::Rerender(_)) => std::cmp::Ordering::Equal,
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
                            update_message.rerender_observer.borrow().notify();
                        }
                    }
                    SchedulerMessage::Rerender(rerender_message) => {
                        let vdom = rerender_message
                            .component
                            .borrow()
                            .view(rerender_message.behavior.as_ref());
                        // here observer pattern to notify about the new vdom
                    }
                }
            });
        }
    }

    pub fn add_update_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        message: Box<dyn Any>,
        rerender_observer: Rc<RefCell<RerenderObserver>>,
    ) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            scheduler
                .borrow_mut()
                .priority_queue
                .push(SchedulerMessage::Update(UpdateMessage {
                    component,
                    message,
                    rerender_observer,
                }))
        });
    }

    pub fn add_rerender_message(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        behavior: Rc<AnyComponentBehavior>,
        depth: u32,
    ) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            scheduler
                .borrow_mut()
                .priority_queue
                .push(SchedulerMessage::Rerender(RerenderMessage {
                    component,
                    behavior,
                    depth,
                }))
        });
    }
}
