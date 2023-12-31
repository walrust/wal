use super::{not_found_component::NotFoundComponent, PageRenderer, Router};
use crate::{
    component::{node::AnyComponentNode, Component},
    virtual_dom::dom,
};
use std::collections::HashMap;

/// Builds application with routing.
pub struct RouterBuilder {
    pages: HashMap<&'static str, PageRenderer>,
    not_found_page: Option<PageRenderer>,
}

impl Default for RouterBuilder {
    /// Creates default [RouterBuilder].
    fn default() -> Self {
        RouterBuilder {
            pages: HashMap::new(),
            not_found_page: None,
        }
    }
}

impl RouterBuilder {
    /// Adds provided page to application under provided path in variable path. Page is represented by **custom component** - struct implementing trait [Component] and [Default].
    pub fn add_page<C>(mut self, path: &'static str) -> RouterBuilder
    where
        C: Component + Default + 'static,
    {
        let mut pages = self.pages;
        pages.insert(
            path,
            PageRenderer::new(|| {
                AnyComponentNode::new_root_routing(C::default(), dom::get_root_element())
            }),
        );
        self.pages = pages;

        // RouterBuilder {
        //     pages,
        //     not_found_page: self.not_found_page,
        // }
        self
    }

    /// Adds provided not found page to application.
    /// All routes which cannot be resolved will be redirected to this page.
    /// Adding more than one **not found page** results in undefined behavior.
    pub fn add_not_found_page<C>(self) -> RouterBuilder
    where
        C: Component + Default + 'static,
    {
        RouterBuilder {
            pages: self.pages,
            not_found_page: Some(PageRenderer::new(|| {
                AnyComponentNode::new_root_routing(C::default(), dom::get_root_element())
            })),
        }
    }
}

impl RouterBuilder {
    /// Builds [router](Router). If **not found page** was *not* specified, default one is provided.
    pub fn build(self) -> Router {
        if let Some(not_found_page) = self.not_found_page {
            Router::new(self.pages, not_found_page)
        } else {
            Router::new(
                self.pages,
                PageRenderer::new(|| {
                    AnyComponentNode::new_root_routing(NotFoundComponent, dom::get_root_element())
                }),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RouterBuilder;
    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{VNode, VText},
    };
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    const VALID_TEXT: &str = "";

    #[wasm_bindgen_test]
    fn invalid() {
        let invalid = RouterBuilder::default();
        assert!(invalid.not_found_page.is_none());
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
    fn valid_one_page() {
        let valid = RouterBuilder::default().add_page::<Root>("/");
        assert!(valid.pages.contains_key("/"));
        assert_eq!(valid.pages.len(), 1);
        assert!(valid.not_found_page.is_none());
    }

    struct Root2;
    impl Default for Root2 {
        fn default() -> Self {
            Self::new(())
        }
    }
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
    fn valid_multiple_pages() {
        let valid = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2");
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert!(valid.not_found_page.is_none());
    }

    #[wasm_bindgen_test]
    fn build_valid_multiple_pages() {
        let valid = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2");
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert!(valid.not_found_page.is_none());

        let router = valid.build();
        assert!(router.pages.contains_key("/"));
        assert!(router.pages.contains_key("/2"));
        assert_eq!(router.pages.len(), 2);
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build_valid_custom_not_found_page() {
        let valid = RouterBuilder::default().add_not_found_page::<Root>();
        assert_eq!(valid.pages.len(), 0);
        assert!(valid.not_found_page.is_some());

        let router = valid.build();
        assert_eq!(router.pages.len(), 0);
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build_valid_default_not_found_page() {
        let path = "/";
        let valid = RouterBuilder::default().add_page::<Root>(path);
        assert!(valid.pages.contains_key(path));
        assert_eq!(valid.pages.len(), 1);
        assert!(valid.not_found_page.is_none());

        let router = valid.build();
        assert!(router.pages.contains_key(path));
        assert_eq!(router.pages.len(), 1);
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build() {
        let valid = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2")
            .build();

        assert_eq!(valid.pages.len(), 2);
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert!(valid.current.is_none());
    }
}
