use std::{hash::Hash, rc::Rc};

/// Callback is a wrapper around a function that is used to send messages to the [Component](../trait.Component.html).
///
/// `IN` is the type of the input of the wrapped function.
/// Meaning if we would like to send a message to the [component](../trait.Component.html) we need to provide input of type `IN`.
/// Callback should be used for defining child to parent [component](../trait.Component.html) communication and subscribing
/// to HTML [events](../../events/index.html).
pub struct Callback<IN> {
    wrapper: Rc<dyn Fn(IN)>,
}

impl<IN> Callback<IN> {
    /// Function that creates a new instance of the Callback.
    /// Using a Callback defined by this function will not send [messages](../trait.Component.html#associatedtype.Message) to the [component](../trait.Component.html).
    /// It should be used only if the Callback is to send [messages](../trait.Component.html#associatedtype.Message)
    /// to the father [component](../trait.Component.html) and this logic should be contained in `wrapper` argument function.
    /// If sending the [message](../trait.Component.html#associatedtype.Message) to the current [component](../trait.Component.html)
    /// is the goal [create_callback](../behavior/trait.Behavior.html#tymethod.create_callback) function should be used.
    pub fn new<F>(wrapper: F) -> Self
    where
        F: Fn(IN) + 'static,
    {
        Callback {
            wrapper: Rc::new(wrapper),
        }
    }

    /// Function that calls the [Callback] using the provided `input`.
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

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        Self {
            wrapper: self.wrapper.clone(),
        }
    }
}
