pub mod builder;
pub(crate) mod not_found_component;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{component::node::AnyComponentNode, virtual_dom::dom};
use gloo::utils::{body, history, window};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event, EventTarget};

pub(crate) struct PageRenderer {
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
    /// Static [Router] instance.
    pub(crate) static ROUTER: RefCell<Router> = RefCell::new(Router::empty());
}

struct CurrentPage {
    pub path: String,
    pub page: Rc<RefCell<AnyComponentNode>>,
}

/// Router of the application. Handles routing in the application and correctly resolves paths.
pub struct Router {
    pages: HashMap<&'static str, PageRenderer>,
    not_found_path: Option<&'static str>,
    current: Option<CurrentPage>,
}

impl Router {
    pub(crate) fn empty() -> Router {
        Router {
            pages: [].into(),
            not_found_path: None,
            current: None,
        }
    }

    pub(crate) fn new(
        pages: HashMap<&'static str, PageRenderer>,
        not_found_path: Option<&'static str>,
    ) -> Router {
        Router {
            pages,
            not_found_path,
            current: None,
        }
    }

    /// Start of the application. Moves router instance. Should be called only *once* in application.
    ///
    /// # Example
    /// ```
    /// #[derive(Default)]
    /// struct MainPage;
    /// impl Component for MainPage {...}
    ///
    /// //..
    ///
    /// // Start of the application
    /// RouterBuilder::default()
    ///     .add_page::<MainPage>("/")
    ///     .build()
    ///     .start();
    /// ```
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

        let pathname = window().location().pathname().unwrap();
        Self::navigate_to(pathname.as_str());
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

    const WAL_ROUTING_ATTR: &'static str = "data_link";

    fn click(e: Event) {
        let target = e.target().unwrap().unchecked_into::<Element>();
        let matches = target
            .closest(&format!("[{}]", Self::WAL_ROUTING_ATTR))
            .unwrap();
        if let Some(el) = matches {
            e.prevent_default();
            Self::navigate_to(el.get_attribute("href").unwrap().as_str());
        }
    }

    fn navigate_to(url: &str) {
        let mut url = url;
        ROUTER.with(|router| {
            if !router.borrow().pages.contains_key(url) {
                url = router.borrow().not_found_path.unwrap();
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
    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{VNode, VText},
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::router::not_found_component::NOT_FOUND_PATH;

    use super::{builder::RouterBuilder, Router, ROUTER};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    const VALID_TEXT: &str = "";

    #[wasm_bindgen_test]
    fn empty() {
        let empty = Router::empty();

        assert_eq!(empty.pages.len(), 0);
        assert!(empty.not_found_path.is_none());
        assert!(empty.current.is_none());
    }

    struct Root;
    impl Default for Root {
        fn default() -> Self {
            Self::new(())
        }
    }
    impl Component for Root {
        type Message = ();
        type Properties = ();
        fn new(_props: Self::Properties) -> Self {
            Root
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new(VALID_TEXT).into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn new_router() {
        let router = RouterBuilder::default().add_page::<Root>("/").build();

        assert!(router.pages.contains_key("/"));
        assert_eq!(router.pages.len(), 2);
        assert_eq!(router.not_found_path, Some(NOT_FOUND_PATH));
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn start() {
        let router = RouterBuilder::default().add_page::<Root>("/").build();
        let router2 = RouterBuilder::default().add_page::<Root>("/").build();

        router.start();

        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.pages.keys().all(|x| router2.pages.contains_key(*x)));
            assert_eq!(router.not_found_path, router2.not_found_path);
            assert!(router.current.is_some());
            if let Some(cur) = &router.current {
                assert_eq!(cur.path, "/");
            }
        });
    }

    struct Root2;
    impl Component for Root2 {
        type Message = ();
        type Properties = ();
        fn new(_props: Self::Properties) -> Self {
            Root2
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new(VALID_TEXT).into()
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
                assert_eq!(cur.path, NOT_FOUND_PATH);
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
        Router::navigate_to("/");
        ROUTER.with(move |router| {
            let router = router.borrow();
            assert!(router.current.is_some());
            if let Some(cur) = &router.current {
                assert_eq!(cur.path, "/");
            }
        });
    }
}
