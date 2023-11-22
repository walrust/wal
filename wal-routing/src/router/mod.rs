pub mod builder;

use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use gloo::utils::{body, history, window};
use wal::{
    component::{node::AnyComponentNode, Component},
    virtual_dom::dom,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event, EventTarget};

pub struct PageRenderer {
    props: Box<dyn Any>,
    generator: Box<dyn Fn(Box<dyn Any>) -> Rc<RefCell<AnyComponentNode>>>,
}

impl PageRenderer {
    pub fn new<C>(
        generator: impl Fn(Box<dyn Any>) -> Rc<RefCell<AnyComponentNode>> + 'static,
        props: C::Properties,
    ) -> PageRenderer
    where
        C: Component + 'static,
    {
        PageRenderer {
            generator: Box::new(generator),
            props: Box::new(props),
        }
    }

    pub fn render(&self) -> Rc<RefCell<AnyComponentNode>> {
        todo!()
        // (*self.generator)(self.props)
    }
}

thread_local! {
    pub static ROUTER: RefCell<Router> = RefCell::new(Router::mock());
}

const WAL_ROUTING_ATTR: &'static str = "data_link";

pub struct Router {
    pages: HashMap<&'static str, PageRenderer>,
    cur_path: String,
    cur_page: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl Router {
    pub(crate) fn mock() -> Router {
        Router {
            pages: [].into(),
            cur_path: "undefined".to_string(),
            cur_page: None,
        }
    }

    pub(crate) fn new(pages: HashMap<&'static str, PageRenderer>) -> Router {
        let mut pages = pages;
        let cur_path = "/".to_string();
        let page = pages.get_mut(cur_path.as_str()).unwrap().render();
        page.borrow_mut().patch(None, &dom::get_root_element());
        Router {
            pages,
            cur_path,
            cur_page: Some(page),
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
            if pathname.eq(&router.cur_path) {
                return;
            }

            let page_renderer = router.pages.get_mut(pathname.as_str()).unwrap();
            let new_page = page_renderer.render();
            let old_page = router.cur_page.take().unwrap();

            new_page.borrow_mut().view();
            new_page
                .borrow_mut()
                .patch(Some(old_page), &dom::get_root_element());

            router.cur_page = Some(new_page);
            router.cur_path = pathname;
        });
    }

    fn click(e: Event) {
        let target = e.target().unwrap().unchecked_into::<Element>();
        let matches = target
            .matches(&("[".to_owned() + WAL_ROUTING_ATTR + "]"))
            .unwrap();
        if matches {
            e.prevent_default();
            Self::navigate_to(target.get_attribute("href").unwrap().as_str());
        }
    }

    fn navigate_to(url: &str) {
        history()
            .push_state_with_url(&JsValue::null(), "", Some(url))
            .unwrap();
        Self::route();
    }

    fn add_event_listener<T: ?Sized>(target: EventTarget, type_: &str, listener: &Closure<T>) {
        target
            .add_event_listener_with_callback(type_, listener.as_ref().unchecked_ref())
            .unwrap();
    }
}
