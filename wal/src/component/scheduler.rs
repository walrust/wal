use once_cell::sync::Lazy;
use std::{
    any::Any,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::component::AnyComponent;

pub static SCHEDULER_INSTANCE: Lazy<Scheduler> = Lazy::new(|| Scheduler::new());

pub struct Scheduler {
    update_queue: UpdateQueue,
}

unsafe impl Sync for Scheduler {}

impl Scheduler {
    fn new() -> Self {
        Self {
            update_queue: UpdateQueue::new(),
        }
    }

    pub fn event_loop() {
        let update_queue_thread = Self::update_queue_loop();

        update_queue_thread
            .join()
            .expect("Failed to join the update queue thread");
    }

    fn update_queue_loop() -> JoinHandle<()> {
        thread::spawn(move || loop {
            for (any_component, any_message) in SCHEDULER_INSTANCE.update_queue.receiver.iter() {
                let to_rerender = any_component.lock().unwrap().update(any_message);
            }
        })
    }

    pub fn add_update_message(
        component: Arc<Mutex<Box<dyn AnyComponent>>>,
        message: Box<dyn Any + Send>,
    ) {
        SCHEDULER_INSTANCE
            .update_queue
            .sender
            .send((component, message))
            .expect("Failed to send message to the update queue");
    }
}

pub type UpdateQueueSender = Sender<(Arc<Mutex<Box<dyn AnyComponent>>>, Box<dyn Any + Send>)>;
type UpdateQueueReceiver = Receiver<(Arc<Mutex<Box<dyn AnyComponent>>>, Box<dyn Any + Send>)>;

pub struct UpdateQueue {
    pub sender: UpdateQueueSender,
    receiver: UpdateQueueReceiver,
}

impl UpdateQueue {
    fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self { sender, receiver }
    }
}
