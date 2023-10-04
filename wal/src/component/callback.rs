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
