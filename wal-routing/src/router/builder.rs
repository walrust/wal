use std::{any::Any, collections::HashMap, marker::PhantomData};

use wal::component::{node::AnyComponentNode, Component};

use super::{PageRenderer, Router};

pub struct Invalid;
pub struct Valid;
pub struct RouterBuilder<T> {
    pages: HashMap<&'static str, PageRenderer>,
    _marker: PhantomData<T>,
}

impl RouterBuilder<Invalid> {
    pub fn new() -> RouterBuilder<Invalid> {
        RouterBuilder {
            pages: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> RouterBuilder<T> {
    pub fn add_page<C>(self, path: &'static str, props: C::Properties) -> RouterBuilder<Valid>
    where
        C: Component + 'static,
    {
        let generator = Box::new(|props: Box<dyn Any>| {
            let props = props.downcast::<C::Properties>().unwrap();
            AnyComponentNode::new(C::new(*props), wal::virtual_dom::dom::get_root_element())
        });

        let mut pages = self.pages;
        pages.insert(path, PageRenderer::new::<C>(generator, props));

        RouterBuilder {
            pages,
            _marker: PhantomData,
        }
    }
}

impl RouterBuilder<Valid> {
    pub fn build(self) -> Router {
        Router::new(self.pages)
    }
}
