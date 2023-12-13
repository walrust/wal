use crate::component::callback::Callback;

pub struct Store<T: Copy> {
    content: T,
    subscriber_callbacks: Vec<Callback<T>>,
}

// z makra:
// behavior.create_callback(|value: T| Message::StoreValueChanged(value))

impl<T: Copy> Store<T> {
    pub fn add_subscriber(&mut self, callback: Callback<T>) {
        self.subscriber_callbacks.push(callback);
    }

    pub fn notify_all_subscribers(&mut self) {
        for s_callback in self.subscriber_callbacks.iter() {
            s_callback.emit(self.content);
        }
    }
}
