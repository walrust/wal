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
    pub static ROUTER: RefCell<Router> = RefCell::new(Router::empty());
}

const WAL_ROUTING_ATTR: &str = "data_link";

struct CurrentPage {
    pub path: String,
    pub page: Rc<RefCell<AnyComponentNode>>,
}

pub struct Router {
    pages: HashMap<&'static str, PageRenderer>,
    error_path: Option<&'static str>,
    current: Option<CurrentPage>,
}

impl Router {
    pub(crate) fn empty() -> Router {
        Router {
            pages: [].into(),
            error_path: None,
            current: None,
        }
    }

    pub(crate) fn new(
        pages: HashMap<&'static str, PageRenderer>,
        error_path: &'static str,
    ) -> Router {
        Router {
            pages,
            error_path: Some(error_path),
            current: None,
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

    fn route() {
        ROUTER.with(|router| {
            let mut router = router.borrow_mut();
            let pathname = window().location().pathname().unwrap();

            if let Some(old_current) = &router.current {
                if pathname.eq(&old_current.path) {
                    return;
                }
            }

            let old_current = router.current.take();
            let page_renderer = router.pages.get_mut(pathname.as_str()).unwrap();
            let new_page = page_renderer.render();
            let old_page = old_current.map(|x| x.page);

            new_page.borrow_mut().view();
            new_page
                .borrow_mut()
                .patch(old_page, &dom::get_root_element());

            router.current = Some(CurrentPage {
                path: pathname,
                page: new_page,
            });
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
                url = router.borrow().error_path.unwrap();
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
    fn empty() {
        let empty = Router::empty();

        assert_eq!(empty.pages.len(), 0);
        assert!(empty.error_path.is_none());
        assert!(empty.current.is_none());
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
        let router = RouterBuilder::default().add_page::<Root>("/").build();

        assert!(router.pages.contains_key("/"));
        assert_eq!(router.pages.len(), 1);
        assert_eq!(router.error_path, Some("/"));
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn start() {
        let router = RouterBuilder::default().add_page::<Root>("/").build();
        let router2 = RouterBuilder::default().add_page::<Root>("/").build();

        router.start();

        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.pages.keys().eq(router2.pages.keys()));
            assert_eq!(router.error_path, router2.error_path);
            assert!(router.current.is_some());
            if let Some(cur) = &router.current {
                assert_eq!(cur.path, "/");
            }
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
        let router = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root>("/2")
            .build();

        router.start();

        Router::navigate_to("url");
        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.current.is_some());
            if let Some(cur) = &router.current {
                assert_eq!(cur.path, "/");
            }
        });
        Router::navigate_to("/2");
        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.current.is_some());
            if let Some(cur) = &router.current {
                assert_eq!(cur.path, "/2");
            }
        });
    }
}
