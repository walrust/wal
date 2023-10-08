use std::{
    collections::{BinaryHeap, VecDeque},
    sync::{Arc, Condvar, Mutex},
};

pub struct ThreadSafePriorityQueue<T: Ord> {
    queue: Arc<Mutex<BinaryHeap<T>>>,
    condvar: Condvar,
}

impl<T: Ord> Default for ThreadSafePriorityQueue<T> {
    fn default() -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            condvar: Condvar::new(),
        }
    }
}

impl<T: Ord> ThreadSafePriorityQueue<T> {
    pub fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
        self.condvar.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        if let Some(item) = queue.pop() {
            return item;
        }

        queue = self.condvar.wait(queue).unwrap();
        queue.pop().unwrap()
    }
}

pub struct ThreadSafeQueue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    condvar: Condvar,
}

impl<T: Ord> ThreadSafeQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            condvar: Condvar::new(),
        }
    }

    pub fn push_back(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        self.condvar.notify_one();
    }

    pub fn pop_front(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        if let Some(item) = queue.pop_front() {
            return item;
        }

        queue = self.condvar.wait(queue).unwrap();
        queue.pop_front().unwrap()
    }
}
