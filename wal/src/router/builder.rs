use super::{
    not_found_component::{NotFoundComponent, NOT_FOUND_PATH},
    PageRenderer, Router,
};
use crate::{
    component::{node::AnyComponentNode, Component},
    virtual_dom::dom,
};
use std::{collections::HashMap, marker::PhantomData};

/// Struct representing [RouterBuilder] state. In [Invalid] state, it is not possible to create application.
pub struct Invalid;
/// Struct representing [RouterBuilder] state. In [Valid] state, it is possible to create application.
pub struct Valid;

/// Builds application with routing.
pub struct RouterBuilder<T> {
    pages: HashMap<&'static str, PageRenderer>,
    not_found_path: Option<&'static str>,
    _marker: PhantomData<T>,
}

impl Default for RouterBuilder<Invalid> {
    /// Creates default [RouterBuilder] in [Invalid] state.
    fn default() -> Self {
        RouterBuilder {
            pages: HashMap::new(),
            not_found_path: None,
            _marker: PhantomData,
        }
    }
}

impl<T> RouterBuilder<T> {
    /// Adds provided page to application under provided path in variable path. Page is represented by **custom component** - struct implementing trait [Component] and [Default].
    pub fn add_page<C>(self, path: &'static str) -> RouterBuilder<Valid>
    where
        C: Component + Default + 'static,
    {
        let generator =
            Box::new(|| AnyComponentNode::new_root_routing(C::default(), dom::get_root_element()));

        let mut pages = self.pages;
        pages.insert(path, PageRenderer::new(generator));

        RouterBuilder {
            pages,
            not_found_path: self.not_found_path,
            _marker: PhantomData,
        }
    }

    /// Adds provided not found page to application. All routes which cannot be resolved will be redirected to this page. Adding more than one **not found page** results in undefined behavior.
    pub fn add_not_found_page<C>(self, path: &'static str) -> RouterBuilder<Valid>
    where
        C: Component + Default + 'static,
    {
        let router: RouterBuilder<T> = RouterBuilder {
            pages: self.pages,
            not_found_path: Some(path),
            _marker: PhantomData,
        };

        router.add_page::<C>(path)
    }
}

impl RouterBuilder<Valid> {
    /// Builds [router](Router). If **not found page** was *not* specified, default one is provided under /404 route.
    pub fn build(self) -> Router {
        let mut pages = self.pages;
        let mut not_found_path = self.not_found_path;

        if not_found_path.is_none() {
            not_found_path = Some(NOT_FOUND_PATH);
            pages.insert(
                NOT_FOUND_PATH,
                PageRenderer::new(|| {
                    AnyComponentNode::new_root_routing(NotFoundComponent, dom::get_root_element())
                }),
            );
        }

        Router::new(pages, not_found_path)
    }
}

#[cfg(test)]
mod tests {
    use super::RouterBuilder;
    use crate::router::{
        builder::{Invalid, Valid},
        not_found_component::NOT_FOUND_PATH,
    };
    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{VNode, VText},
    };
    use std::any::{Any, TypeId};
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    const VALID_TEXT: &str = "";

    #[wasm_bindgen_test]
    fn generic_struct_typeid_check() {
        assert_ne!(
            TypeId::of::<RouterBuilder<Invalid>>(),
            TypeId::of::<RouterBuilder<Valid>>()
        );
    }

    #[wasm_bindgen_test]
    fn invalid() {
        let invalid = RouterBuilder::default();
        assert_eq!(TypeId::of::<RouterBuilder<Invalid>>(), invalid.type_id());
        assert_eq!(invalid.not_found_path, None);
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
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert_eq!(valid.pages.len(), 1);
        assert_eq!(valid.not_found_path, None);
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
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert_eq!(valid.not_found_path, None);
    }

    #[wasm_bindgen_test]
    fn build_valid_multiple_pages() {
        let valid = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2");
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert_eq!(valid.not_found_path, None);

        let router = valid.build();
        assert!(router.pages.contains_key("/"));
        assert!(router.pages.contains_key("/2"));
        assert_eq!(router.pages.len(), 3);
        assert_eq!(router.not_found_path, Some(NOT_FOUND_PATH));
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build_valid_custom_not_found_page() {
        let path = "/not-found";
        let valid = RouterBuilder::default().add_not_found_page::<Root>(path);
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key(path));
        assert_eq!(valid.pages.len(), 1);
        assert_eq!(valid.not_found_path, Some(path));

        let router = valid.build();
        assert!(router.pages.contains_key(path));
        assert_eq!(router.pages.len(), 1);
        assert_eq!(router.not_found_path, Some(path));
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build_valid_default_not_found_page() {
        let path = "/";
        let valid = RouterBuilder::default().add_page::<Root>(path);
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key(path));
        assert_eq!(valid.pages.len(), 1);
        assert_eq!(valid.not_found_path, None);

        let router = valid.build();
        assert!(router.pages.contains_key(path));
        assert_eq!(router.pages.len(), 2);
        assert_eq!(router.not_found_path, Some(NOT_FOUND_PATH));
        assert!(router.current.is_none());
    }

    #[wasm_bindgen_test]
    fn build() {
        let valid = RouterBuilder::default()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2")
            .build();

        assert_eq!(valid.pages.len(), 3);
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.not_found_path, Some(NOT_FOUND_PATH));
        assert!(valid.current.is_none());
    }
}
