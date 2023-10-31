use std::{collections::HashMap, marker::PhantomData};

use wal::component::{node::AnyComponentNode, Component};

use super::App;

pub enum Page {
    NotRendered(Box<dyn FnOnce() -> AnyComponentNode>),
    Rendered(AnyComponentNode),
}

pub struct EmptyApp;
pub struct ValidApp;
pub struct AppBuilder<T> {
    pages: HashMap<&'static str, Page>,
    _marker: PhantomData<T>,
}

impl AppBuilder<EmptyApp> {
    pub fn new() -> AppBuilder<EmptyApp> {
        AppBuilder {
            pages: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> AppBuilder<T> {
    pub fn add_page<C>(self, path: &'static str, props: C::Properties) -> AppBuilder<ValidApp>
    where 
        C: Component + 'static
    {
        let generator = Box::new(
            move || 
            AnyComponentNode::new(
                C::new(props), 
                wal::virtual_dom::Dom::get_root_element()
            )
        );

        let mut pages = self.pages;
        //pages.insert(path, Page::NotRendered(generator));

        AppBuilder { 
            pages, 
            _marker: PhantomData 
        }
    }
}
