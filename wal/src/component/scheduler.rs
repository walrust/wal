use std::{
    any::Any,
    cell::RefCell,
};

use super::{
    component::AnyComponent,
    context_node::{AnyComponentBehavior, RerenderObserver},
    thread_safe_collections::ThreadSafePriorityQueue,
};

enum SchedulerMessage {
    Update(UpdateMessage),
    Rerender(RerenderMessage),
}

struct UpdateMessage {
    component: Box<dyn AnyComponent>,
    message: Box<dyn Any>,
    rerender_observer: RerenderObserver,
}

struct RerenderMessage {
    component: Box<dyn AnyComponent>,
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
    pub priority_queue: ThreadSafePriorityQueue<SchedulerMessage>,
}

impl Scheduler {
    pub fn event_loop() {
        loop {
            SCHEDULER_INSTANCE.with(|scheduler| {
                let scheduler_message = scheduler.borrow_mut().priority_queue.pop();
                match scheduler_message {
                    SchedulerMessage::Update(update_message) => {
                        // let to_rerender = update_message.component.update(update_message.message);
                        // if to_rerender {
                        //     update_message.rerender_observer.notify();
                        // }
                    }
                    SchedulerMessage::Rerender(rerender_message) => {
                        // let vdom = rerender_message.component.view();
                    }
                }
            });
        }
    }

    pub fn add_update_message(
        component: Box<dyn AnyComponent>,
        message: Box<dyn Any>,
        rerender_observer: RerenderObserver,
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
        component: Box<dyn AnyComponent>,
        behavior: AnyComponentBehavior,
        depth: u32,
    ) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            scheduler
                .borrow_mut()
                .priority_queue
                .push(SchedulerMessage::Rerender(RerenderMessage {
                    component,
                    depth,
                }))
        });
    }
}
