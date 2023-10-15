use std::hash::Hash;

pub struct Callback<IN> {
    wrapper: Box<dyn Fn(IN)>,
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

impl<IN> Hash for Callback<IN> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self.wrapper.as_ref() as *const dyn Fn(IN);
        ptr.hash(state);
    }
}
