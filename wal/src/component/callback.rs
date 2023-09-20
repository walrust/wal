use std::hash::{Hash, Hasher};

/// Once created callback for child shouldnt change, the change isnt registered in runtime
pub struct Callback<IN> {
    wrapper: Box<dyn Fn(IN)>,
}

impl<IN> Hash for Callback<IN> {
    fn hash<H: Hasher>(&self, _state: &mut H) {
    }
}

impl<IN> Callback<IN> {
    pub fn new<F>(wrapper: F) -> Self
    where
        F: Fn(IN) + 'static,
    {
        Callback {
            wrapper: Box::new(wrapper),
        }
    }

    pub fn emit(&self, input: IN) {
        (self.wrapper)(input);
    }
}
