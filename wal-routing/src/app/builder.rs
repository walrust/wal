use std::{collections::HashMap, marker::PhantomData};

use wal::component::{node::AnyComponentNode, Component};

use super::{LazyPage, App};

pub struct EmptyApp;
pub struct ValidApp;
pub struct AppBuilder<T> {
    pages: HashMap<&'static str, LazyPage>,
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

impl AppBuilder<ValidApp> {
    pub fn build(self) -> App {
        App::new(self.pages)
    }
}
