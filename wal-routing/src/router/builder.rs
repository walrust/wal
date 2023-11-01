use std::{collections::HashMap, marker::PhantomData};

use wal::component::{node::AnyComponentNode, Component};

use super::{LazyPage, Router};


pub struct Invalid;
pub struct Valid;
pub struct Builder<T> {
    pages: HashMap<&'static str, LazyPage>,
    _marker: PhantomData<T>,
}

impl Builder<Invalid> {
    pub fn new() -> Builder<Invalid> {
        Builder {
            pages: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> Builder<T> {
    pub fn add_page<C>(self, path: &'static str, props: C::Properties) -> Builder<Valid>
    where 
        C: Component + 'static
    {
        let generator = Box::new(
            move || 
            AnyComponentNode::new(
                C::new(props), 
                wal::virtual_dom::dom::get_root_element()
            )
        );

        let mut pages = self.pages;
        pages.insert(path, LazyPage::new(generator));

        Builder { 
            pages, 
            _marker: PhantomData 
        }
    }
}

impl Builder<Valid> {
    pub fn build(self) -> Router {
        Router::new(self.pages)
    }
}