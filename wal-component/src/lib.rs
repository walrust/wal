use gloo::utils::document;
use wal_vdom::virtual_dom::VNode;
use web_sys::Node;


pub struct Callback<IN> {
    #[allow(dead_code)]
    wrapper: Box<dyn Fn(IN)>,
}

impl<IN> Callback<IN> {
    fn new<F>(wrapper: F) -> Self
    where
        F: Fn(IN) + 'static,
    {
        Callback {
            wrapper: Box::new(wrapper),
        }
    }

    #[allow(dead_code)]
    fn emit(&self, input: IN) {
        (self.wrapper)(input);
    }
}

pub trait Component {
   type Message;
   type Properties: PartialEq;

    fn new(props: Self::Properties) -> Self;
    fn view(&self) -> VNode;
    fn update(&mut self, message: Self::Message) -> bool;

    fn create_callback<IN, F>(wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> Self::Message + 'static,
    {
        Callback::new(move |data| {
            let _message = wrapper(data);
            todo!("send message to a queue, which will be processed later - meaning an update method will be called")
        })
    }
}

pub type Html = VNode;

pub struct App<Root>
    where Root: Component 
{
    #[allow(dead_code)]
    root: Root,
    vdom: VNode,
    dom: Node,
}

impl<Root> App<Root>
    where Root: Component
{
    pub fn new(root: Root) -> App<Root> {
        let vdom = root.view();
        //App { root }
        todo!()
    } 
}

#[cfg(test)]
mod tests {
}

