pub mod builder;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gloo::utils::{body, history, window};
use wal::{component::node::AnyComponentNode, virtual_dom::dom};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event, EventTarget};

pub struct PageRenderer {
    generator: Box<dyn Fn() -> Rc<RefCell<AnyComponentNode>>>,
}

impl PageRenderer {
    pub fn new(generator: impl Fn() -> Rc<RefCell<AnyComponentNode>> + 'static) -> PageRenderer {
        PageRenderer {
            generator: Box::new(generator),
        }
    }

    pub fn render(&self) -> Rc<RefCell<AnyComponentNode>> {
        (*self.generator)()
    }
}

thread_local! {
    pub static ROUTER: RefCell<Router> = RefCell::new(Router::mock());
}

const WAL_ROUTING_ATTR: &str = "data_link";

pub struct Router {
    pages: HashMap<&'static str, PageRenderer>,
    error_path: &'static str,
    cur_path: String,
    cur_page: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl Router {
    pub(crate) fn mock() -> Router {
        Router {
            pages: [].into(),
            error_path: "undefined",
            cur_path: "undefined".to_string(),
            cur_page: None,
        }
    }

    pub(crate) fn new(
        pages: HashMap<&'static str, PageRenderer>,
        error_path: &'static str,
    ) -> Router {
        Router {
            pages,
            error_path,
            cur_path: "undefined".to_string(),
            cur_page: None,
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

        Self::navigate_to(window().location().pathname().unwrap().as_str());
    }
}

impl Router {
    fn route() {
        ROUTER.with(|router| {
            let pathname = window().location().pathname().unwrap();
            let mut router = router.borrow_mut();
            if pathname.eq(&router.cur_path) {
                return;
            }

            let page_renderer = router.pages.get_mut(pathname.as_str()).unwrap();
            let new_page = page_renderer.render();
            let old_page = router.cur_page.take();

            new_page.borrow_mut().view();
            new_page
                .borrow_mut()
                .patch(old_page, &dom::get_root_element());

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
        let mut url = url;
        ROUTER.with(|router| {
            if !router.borrow().pages.contains_key(url) {
                url = router.borrow().error_path;
            }
        });

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

#[cfg(test)]
mod tests {
    use wal::{
        component::{behavior::Behavior, root::RootComponent},
        virtual_dom::{VNode, VText},
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{builder::RouterBuilder, Router, ROUTER};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn mock() {
        let mock = Router::mock();

        assert_eq!(mock.pages.len(), 0);
        assert_eq!(mock.error_path, "undefined");
        assert_eq!(mock.cur_path, "undefined".to_string());
        assert!(mock.cur_page.is_none());
    }

    struct Root;
    impl RootComponent for Root {
        type Message = ();
        fn new_root() -> Self {
            Root
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new("RootComponent so cool").into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn new_router() {
        let router = RouterBuilder::new().add_page::<Root>("/").build();

        assert!(router.pages.contains_key("/"));
        assert_eq!(router.pages.len(), 1);
        assert_eq!(router.error_path, "/");
        assert_eq!(router.cur_path, "undefined");
        assert!(router.cur_page.is_none());
    }

    #[wasm_bindgen_test]
    fn start() {
        let router = RouterBuilder::new().add_page::<Root>("/").build();
        let router2 = RouterBuilder::new().add_page::<Root>("/").build();

        router.start();

        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.pages.keys().eq(router2.pages.keys()));
            assert_eq!(router.error_path, router2.error_path);
            assert_eq!(router.cur_path, "/");
            assert!(router.cur_page.is_some());
        });
    }

    struct Root2;
    impl RootComponent for Root2 {
        type Message = ();
        fn new_root() -> Self {
            Root2
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new("RootComponent so cool").into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn navigate_to() {
        let router = RouterBuilder::new()
            .add_page::<Root>("/")
            .add_page::<Root>("/2")
            .build();

        router.start();

        Router::navigate_to("url");
        ROUTER.with(move |router| {
            let router = router.borrow();
            assert_eq!(router.cur_path, "/");
        });
        Router::navigate_to("/2");
        ROUTER.with(move |router| {
            let router = router.borrow();
            assert_eq!(router.cur_path, "/2");
        });
    }
}
