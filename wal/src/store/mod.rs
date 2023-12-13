use crate::component::callback::Callback;

pub struct Store<T: Copy> {
    content: T,
    subscriber_callbacks: Vec<Callback<T>>,
}

impl<T: Copy> Store<T> {
    pub fn new(initial_content: T) -> Self {
        Self {
            content: initial_content,
            subscriber_callbacks: vec![],
        }
    }

    pub fn add_subscriber(&mut self, callback: Callback<T>) {
        self.subscriber_callbacks.push(callback);
    }

    pub fn notify_all_subscribers(&mut self) {
        for s_callback in self.subscriber_callbacks.iter() {
            s_callback.emit(self.content);
        }
    }
}

// pub(crate) trait AnyStore {
//     fn new(initial_content: Box<dyn Any>) -> Self
//     where
//         Self: Sized;
//     fn add_subscriber(&mut self, callback: Box<dyn Any>);

//     fn notify_all_subscribers(&mut self);
// }

// impl<T> AnyStore for Store<T>
// where
//     T: Copy + 'static,
// {
//     fn new(initial_content: Box<dyn Any>) -> Self
//     where
//         Self: Sized,
//     {
//         let initial_content = *initial_content.downcast::<T>().expect(
//             "Failed to downcast initial_content in AnyStore to initial_content of a real store",
//         );
//         Store::<T>::new(initial_content)
//     }

//     fn add_subscriber(&mut self, callback: Box<dyn Any>) {
//         let callback = *callback
//             .downcast::<Callback<T>>()
//             .expect("Failed to downcast callback in AnyStore to callback of a real store");

//         self.add_subscriber(callback);
//     }

//     fn notify_all_subscribers(&mut self) {
//         self.notify_all_subscribers();
//     }
// }
