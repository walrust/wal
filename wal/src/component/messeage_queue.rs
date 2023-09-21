use std::collections::VecDeque;

use super::Component;

pub struct MessageQueue<C: Component> {
    pending_messeages: VecDeque<C::Message>,
    component: C,
}

impl<C: Component> MessageQueue<C> {
    pub fn new(&mut self, component: C) -> Self {
        MessageQueue {
            pending_messeages: VecDeque::new(),
            component,
        }
    }

    pub fn add_messeage(&mut self, msg: C::Message) {
        self.pending_messeages.push_back(msg);
    }

    pub fn handle_messeage(&mut self, msg: C::Message) {
        let msg = self.pending_messeages.pop_front().unwrap();

        // TODO: separate to another layer later to not call update directly
        self.component.update(msg);
    }
}
