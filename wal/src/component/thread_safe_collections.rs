use std::{
    collections::BinaryHeap,
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
