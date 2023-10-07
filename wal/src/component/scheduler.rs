use once_cell::sync::Lazy;
use std::{
    any::Any,
    collections::BinaryHeap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Condvar, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::{component::AnyComponent, context_node::AnyComponentBehavior};

pub static SCHEDULER_INSTANCE: Lazy<Scheduler> = Lazy::new(|| Scheduler::new());

pub struct Scheduler {
    update_queue: UpdateQueue,
    rerender_queue: RerenderPriorityQueue,
}

unsafe impl Sync for Scheduler {}

impl Scheduler {
    fn new() -> Self {
        Self {
            update_queue: UpdateQueue::new(),
            rerender_queue: RerenderPriorityQueue::new(),
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

    fn rerender_queue_loop() -> JoinHandle<()> {
        thread::spawn(move || loop {
            let rerender_queue_item = SCHEDULER_INSTANCE.rerender_queue.pop();
            let behavior = AnyComponentBehavior::new(rerender_queue_item.component.clone());
            let vnode = rerender_queue_item
                .component
                .lock()
                .unwrap()
                .view(&behavior);
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

    pub fn add_rerender_message(component: Arc<Mutex<Box<dyn AnyComponent>>>, depth: u32) {
        SCHEDULER_INSTANCE
            .rerender_queue
            .push(RerenderQueueItem { component, depth });
    }
}

type UpdateQueueItem = (Arc<Mutex<Box<dyn AnyComponent>>>, Box<dyn Any + Send>);
type UpdateQueueSender = Sender<UpdateQueueItem>;
type UpdateQueueReceiver = Receiver<UpdateQueueItem>;

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

struct ThreadSafePriorityQueue<T> {
    queue: Arc<Mutex<BinaryHeap<T>>>,
    condvar: Condvar,
}

impl<T: Ord> ThreadSafePriorityQueue<T> {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            condvar: Condvar::new(),
        }
    }

    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
        self.condvar.notify_one();
    }

    fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        if let Some(item) = queue.pop() {
            return item;
        }

        queue = self.condvar.wait(queue).unwrap();
        queue.pop().unwrap()
    }
}

struct RerenderQueueItem {
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    depth: u32,
}

impl PartialEq for RerenderQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl Eq for RerenderQueueItem {}

impl PartialOrd for RerenderQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.depth.cmp(&self.depth))
    }
}

impl Ord for RerenderQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.depth.cmp(&self.depth)
    }
}

type RerenderPriorityQueue = ThreadSafePriorityQueue<RerenderQueueItem>;
