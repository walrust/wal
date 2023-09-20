use gloo::utils::document;
use web_sys::Node;

use crate::{component::Component, virtual_dom::VNode};


pub struct App<Root>
    where Root: Component 
{
    root: Root,
    vdom: VNode,
}

impl<Root> App<Root>
    where Root: Component
{
    pub fn new(root: Root) -> App<Root> {
        let mut vdom = root.view();
        vdom.patch(None, &document().body().unwrap());
        App { root, vdom }
    } 
}

pub fn start<C: Component>(root: C) {
    App::new(root);
}

#[cfg(test)]
mod tests {
}