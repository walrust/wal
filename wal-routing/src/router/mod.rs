pub mod builder;

use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap};

use gloo::utils::{window, history, body};
use wal::{component::node::AnyComponentNode, virtual_dom::dom};
use web_sys::{EventTarget, Element, Event};
use wasm_bindgen::{prelude:: Closure, JsCast, JsValue};

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
    cur_path: String,
    cur_page: Weak<RefCell<AnyComponentNode>>,
}

impl Router {
    pub(crate) fn mock() -> Router {
        Router { pages: [].into(), cur_path: "undefined".to_string(), cur_page: Weak::new() } 
    }

    pub(crate) fn new(pages: HashMap<&'static str, LazyPage>) -> Router {
        let mut pages = pages;
        let cur_path = "/".to_string();
        let cur_page = pages.get_mut(cur_path.as_str()).unwrap().page();
        cur_page.upgrade().unwrap().borrow_mut().patch(None, &dom::get_root_element());
        Router {
            pages,
            cur_path,
            cur_page,
        }
    }

    pub fn start(self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        let click = Closure::<dyn Fn(Event)>::new(Self::click);
        Self::add_event_listener(body().into(), "click", &click);
        click.forget();

        let route = Closure::<dyn Fn()>::new(Self::route);
        Self::add_event_listener(window().into(), "popstate", &route);
        route.forget();

        ROUTER.with(move |router| {
            let mut router = router.borrow_mut();
            *router = self;
        });
    }
}

impl Router {
    fn route() {
    ROUTER.with(|router| {
        let mut router = router.borrow_mut();
        let pathname = window().location().pathname().unwrap();
        if pathname.eq(&router.cur_path) { return; }

        let lazy_page = router.pages.get_mut(pathname.as_str()).unwrap();
        let new_weak_page = lazy_page.page();
        let new_page = new_weak_page.upgrade().unwrap();
            
        let old_page = router.cur_page.upgrade().unwrap();

        new_page.borrow_mut().view();
        new_page.borrow_mut().patch(Some(old_page), &dom::get_root_element());

        router.cur_page = new_weak_page;
        router.cur_path = pathname;
    });
    }

    fn click(e: Event) {
        let target = e.target().unwrap().unchecked_into::<Element>();
        let matches = target.matches(&("[".to_owned() + WAL_ROUTING_ATTR + "]")).unwrap();
        if matches {
            e.prevent_default();
            Self::navigate_to(target.get_attribute("href").unwrap().as_str());
        }
    }

    fn navigate_to(url: &str) {
        history().push_state_with_url(&JsValue::null(), "", Some(url)).unwrap();
        Self::route();
    }


    fn add_event_listener<T: ?Sized>(target: EventTarget, type_: &str, listener: &Closure<T>) 
    {
        target
            .add_event_listener_with_callback(
                type_, 
                listener.as_ref().unchecked_ref()
            )
            .unwrap();
    }
}