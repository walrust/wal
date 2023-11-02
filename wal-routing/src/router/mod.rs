pub mod builder;

use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap, path, ptr::null};

use gloo::utils::{document, window, history, body};
use wal::{component::{node::AnyComponentNode, scheduler::Scheduler}, virtual_dom::dom, utils::debug};
// use wal::app::ROOT_INSTANCE;
use web_sys::{Node, Location, MouseEvent, EventTarget, Element, Event};
use wasm_bindgen::{prelude::{self, Closure}, JsCast, JsValue};

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

const WAL_ROUTING_ATTR: &'static str = "data_link";

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
        let click = Closure::<dyn Fn(Event)>::new(|e: Event| {
            debug::log("In click");
            let target = e.target().unwrap().unchecked_into::<Element>();
            let b = target.matches(&("[".to_owned() + WAL_ROUTING_ATTR + "]")).unwrap();
            if b {
                debug::log("\tNavigating in spa");
                e.prevent_default();
                Self::navigate_to(target.get_attribute("href").unwrap().as_str());
            }
        });
        let router = Closure::<dyn Fn()>::new(Self::route);
        
        body()
            .add_event_listener_with_callback(
                "click", 
                click.as_ref().unchecked_ref()
            )
            .unwrap();

        window()
            .add_event_listener_with_callback(
                "popstate", 
                router.as_ref().unchecked_ref()
            )
            .unwrap();

        ROUTER.with(move |router| {
            let mut router = router.borrow_mut();
            *router = self;
        });
        click.forget();
        router.forget();
        // Self::route();
    }

    fn route() {
    ROUTER.with(|router| {
        let mut router = router.borrow_mut();
        let x = router.pages.iter().find(|(s, _p)| window().location().pathname().unwrap().eq(*s));
        let x = x.map(|(s, _p)| s);

        debug::log(format!("{:#?}", x));

        let pathname = window().location().pathname().unwrap();
        let new_page = router
        .pages.get_mut(pathname.as_str()).unwrap()
        .page()
        .upgrade().unwrap();
        let old_page = router.cur_page.upgrade().unwrap();
        new_page.borrow_mut().patch(None, &dom::get_root_element());
        router.cur_page = Rc::downgrade(&new_page);
    });
    }

    fn navigate_to(url: &str) {
        history().push_state_with_url(&JsValue::null(), "", Some(url)).unwrap();
        Self::route();
    }

    // pub fn switch_page(&mut self, path: &'static str) {
    //     self.cur_page = self.pages.get_mut(path).unwrap().page();
    // }
}