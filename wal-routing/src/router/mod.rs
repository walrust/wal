pub mod builder;

use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap, path, ptr::null};

use wal::{component::{node::AnyComponentNode, scheduler::Scheduler}, virtual_dom::dom, app::ROOT_INSTANCE, utils::debug};
use web_sys::{Node, window, Location};
use wasm_bindgen::{prelude::{self, Closure}, JsCast};

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

thread_local!{
    pub static ROUTER: RefCell<Router> = RefCell::new(Router::mock());
}

pub struct Router {
    pages: HashMap<&'static str, LazyPage>,
    cur_page: Weak<RefCell<AnyComponentNode>>,
}

impl Router {
    pub(crate) fn mock() -> Router {
        debug::log("router::mock");
        Router { pages: [].into(), cur_page: Weak::new() } 
    }

    pub(crate) fn new(pages: HashMap<&'static str, LazyPage>) -> Router {
        let mut pages = pages;
        debug::log("router::new");
        let cur_page = pages.get_mut("/").unwrap().page();
        cur_page.upgrade().unwrap().borrow_mut().patch(None, &dom::get_root_element());
        Router {
            pages,
            cur_page,
        }
    }

    pub fn start(self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let root = self.cur_page.upgrade().unwrap();

        ROOT_INSTANCE.with(move |root_instance| {
            *root_instance.borrow_mut() = Some(root);
        });
        ROUTER.with(move |router| {
            *router.borrow_mut() = self;
        });

        let closure = Closure::<dyn Fn()>::new(|| {
            ROUTER.with(|router| { ROOT_INSTANCE.with(move |root_instance| {
                let path = window().unwrap().location().pathname().unwrap();
                let mut router = router.borrow_mut();
                if let Some(page) = router.pages.get_mut(path.as_str()) {
                    debug::log(format!("There is page {}", path));
                    router.cur_page = page.page();
                } else {
                    debug::log(format!("There is no page {}", path));
                    router.cur_page = router.pages.get_mut("/").unwrap().page();
                }
                let mut instance = root_instance.borrow_mut();
                let old = instance.take();
                *instance = Some(router.cur_page.upgrade().unwrap());
                let mut node = instance.as_mut().unwrap().borrow_mut();
                node.patch(old, &dom::get_root_element());
            });});
        });
        window().unwrap().add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    // pub fn switch_page(&mut self, path: &'static str) {
    //     self.cur_page = self.pages.get_mut(path).unwrap().page();
    // }
}