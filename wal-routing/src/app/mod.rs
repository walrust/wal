pub mod builder;

use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap};

use wal::component::node::AnyComponentNode;

// Consider using this enum in whole thing, maybe not rly threadsafe but we are singlethreaded still
// pub enum Lazy<T> {
//     NotRendered(Box<dyn FnOnce() -> T>),
//     Rendered(T),
// }

// impl<T> Lazy<T> {
//     pub fn render(&mut self) {
//         match self {
//             Lazy::NotRendered(gen) => {
//                 let gen = std::mem::replace(
//                     gen,
//                     Box::new(|| { panic!("This function should never be called") })
//                 );
//                 let t = gen();
//                 *self = Lazy::Rendered(t);
//             },
//             Lazy::Rendered(_) => {},
//         }

//     }
// }

pub struct LazyPage {
    generator: Option<Box<dyn FnOnce() -> Rc<RefCell<AnyComponentNode>>>>,
    page: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl LazyPage {
    pub fn new(generator: Box<dyn FnOnce() -> Rc<RefCell<AnyComponentNode>>>) -> LazyPage {
        LazyPage { 
            generator: Some(generator), 
            page: None,
        }
    }

    pub fn page(&mut self) -> Weak<RefCell<AnyComponentNode>> {
        match &self.page {
            Some(rc) => Rc::downgrade(rc),
            None => {
                let x = (self.generator.take().unwrap())();
                self.page = Some(x.clone());
                Rc::downgrade(&x)
            },
        }
    }
}

pub struct App {
    pages: HashMap<&'static str, LazyPage>,
    cur_page: Weak<RefCell<AnyComponentNode>>,
}

impl App {
    pub(crate) fn new(pages: HashMap<&'static str, LazyPage>) -> App {
        let mut pages = pages;
        let cur_page = pages.get_mut("/").unwrap().page();
        App {
            pages,
            cur_page,
        }
    }

    pub fn switch_page(&mut self, path: &'static str) {
        self.cur_page = self.pages.get_mut(path).unwrap().page();
    }
}
