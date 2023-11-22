use super::{PageRenderer, Router};
use std::{collections::HashMap, marker::PhantomData};
use wal::component::{node::AnyComponentNode, root::RootComponent};

const ERROR_PATH: &'static str = "/404";

pub struct Invalid;
pub struct Valid;

pub struct RouterBuilder<T> {
    pages: HashMap<&'static str, PageRenderer>,
    error_path: &'static str,
    _marker: PhantomData<T>,
}

impl RouterBuilder<Invalid> {
    pub fn new() -> RouterBuilder<Invalid> {
        RouterBuilder {
            pages: HashMap::new(),
            error_path: ERROR_PATH,
            _marker: PhantomData,
        }
    }

    pub fn add_page<C>(self, path: &'static str) -> RouterBuilder<Valid>
    where
        C: RootComponent + 'static,
    {
        let tmp: RouterBuilder<Valid> = RouterBuilder {
            pages: self.pages,
            error_path: path,
            _marker: PhantomData,
        };

        tmp.add_page::<C>(path)
    }
}

impl RouterBuilder<Valid> {
    pub fn add_page<C>(self, path: &'static str) -> RouterBuilder<Valid>
    where
        C: RootComponent + 'static,
    {
        let generator = Box::new(|| {
            AnyComponentNode::new(C::new_root(), wal::virtual_dom::dom::get_root_element())
        });

        let mut pages = self.pages;
        pages.insert(path, PageRenderer::new(generator));

        RouterBuilder {
            pages,
            error_path: self.error_path,
            _marker: PhantomData,
        }
    }

    pub fn build(self) -> Router {
        Router::new(self.pages, self.error_path)
    }
}

#[cfg(test)]
mod tests {
    use super::RouterBuilder;
    use crate::router::builder::{Invalid, Valid};
    use std::any::{Any, TypeId};
    use wal::{
        component::{behavior::Behavior, root::RootComponent},
        virtual_dom::{VNode, VText},
    };
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn generic_struct_typeid_check() {
        assert_ne!(
            TypeId::of::<RouterBuilder<Invalid>>(),
            TypeId::of::<RouterBuilder<Valid>>()
        );
    }

    #[wasm_bindgen_test]
    fn invalid() {
        let invalid = RouterBuilder::new();
        assert_eq!(TypeId::of::<RouterBuilder<Invalid>>(), invalid.type_id());
        assert_eq!(invalid.error_path, "/404");
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
    fn valid_one_page() {
        let valid = RouterBuilder::new().add_page::<Root>("/");
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert_eq!(valid.pages.len(), 1);
        assert_eq!(valid.error_path, "/");
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
    fn valid_multiple_pages() {
        let valid = RouterBuilder::new()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2");
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert_eq!(valid.error_path, "/");
    }

    #[wasm_bindgen_test]
    fn build_valid_multiple_pages() {
        let valid = RouterBuilder::new()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2");
        assert_eq!(TypeId::of::<RouterBuilder<Valid>>(), valid.type_id());
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.pages.len(), 2);
        assert_eq!(valid.error_path, "/");
    }

    #[wasm_bindgen_test]
    fn build() {
        let valid = RouterBuilder::new()
            .add_page::<Root>("/")
            .add_page::<Root2>("/2")
            .build();

        assert_eq!(valid.pages.len(), 2);
        assert!(valid.pages.contains_key("/"));
        assert!(valid.pages.contains_key("/2"));
        assert_eq!(valid.error_path, "/");
        assert_eq!(valid.cur_path, "undefined".to_string());
        assert!(valid.cur_page.is_none());
    }

}
