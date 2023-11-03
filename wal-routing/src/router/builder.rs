use std::{collections::HashMap, marker::PhantomData};

use wal::component::{node::AnyComponentNode, Component};

use super::{LazyPage, Router};


pub struct Invalid;
pub struct Valid;
pub struct AppBuilder<T> {
    pages: HashMap<&'static str, LazyPage>,
    _marker: PhantomData<T>,
}

impl AppBuilder<Invalid> {
    pub fn new() -> AppBuilder<Invalid> {
        AppBuilder {
            pages: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> AppBuilder<T> {
    pub fn add_page<C>(self, path: &'static str, props: C::Properties) -> AppBuilder<Valid>
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

        AppBuilder { 
            pages, 
            _marker: PhantomData 
        }
    }
}

impl AppBuilder<Valid> {
    pub fn build(self) -> Router {
        Router::new(self.pages)
    }
}