use std::{hash::Hash, rc::Rc};

use gloo::console::log;

pub struct Callback<IN> {
    wrapper: Rc<dyn Fn(IN)>,
}

impl<IN> Callback<IN> {
    pub fn new<F>(wrapper: F) -> Self
    where
        F: Fn(IN) + 'static,
    {
        Callback {
            wrapper: Rc::new(wrapper),
        }
    }

    pub fn emit(&self, input: IN) {
        log!("Emitting callback");
        (self.wrapper)(input);
    }
}

impl<IN> Hash for Callback<IN> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self.wrapper.as_ref() as *const dyn Fn(IN);
        ptr.hash(state);
    }
}

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        Self {
            wrapper: self.wrapper.clone(),
        }
    }
}
