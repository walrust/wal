use std::collections::VecDeque;

pub struct MessageQueue<T> {
    messages: VecDeque<T>,
}

impl<T> MessageQueue<T> {
    pub fn new() -> Self {
        MessageQueue {
            messages: VecDeque::new(),
        }
    }

    pub fn add_messeage(&mut self, msg: T) {
        self.messages.push_back(msg);
    }
}
