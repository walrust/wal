use crate::virtual_dom::VNode;

use super::{behavior::Behavior, Component};

pub trait RootComponent: Sized {
    type Message: 'static;
    fn new_root() -> Self;
    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode;
    fn update(&mut self, message: Self::Message) -> bool;
}

impl<T: RootComponent> Component for T {
    type Message = T::Message;
    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        Self::new_root()
    }

    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode {
        self.view(behavior)
    }

    fn update(&mut self, message: Self::Message) -> bool {
        self.update(message)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        component::{
            behavior::{AnyComponentBehavior, Behavior},
            Component,
        },
        virtual_dom::{VNode, VText},
    };

    use super::RootComponent;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct RootComp(i32);
    enum RootMessage {
        Add,
        Substract,
        Nothing,
    }
    impl RootComponent for RootComp {
        type Message = RootMessage;
        fn new_root() -> Self {
            RootComp(0x859)
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new("RootComp").into()
        }
        fn update(&mut self, message: Self::Message) -> bool {
            match message {
                RootMessage::Add => {
                    self.0 += 1;
                    true
                }
                RootMessage::Substract => {
                    self.0 -= 1;
                    true
                }
                RootMessage::Nothing => false,
            }
        }
    }

    #[wasm_bindgen_test]
    fn root_component_new() {
        let root1: RootComp = RootComponent::new_root();
        let root2: RootComp = Component::new(());

        assert_eq!(root1.0, root2.0);
    }

    #[wasm_bindgen_test]
    fn root_component_update() {
        let mut root1: RootComp = RootComp::new_root();
        let mut root2: RootComp = Component::new(());

        let add1 = RootComponent::update(&mut root1, RootMessage::Add);
        let add2 = Component::update(&mut root2, RootMessage::Add);
        assert_eq!(add1, add2);
        assert_eq!(root1.0, root2.0);

        let sub1 = RootComponent::update(&mut root1, RootMessage::Substract);
        let sub2 = Component::update(&mut root2, RootMessage::Substract);
        assert_eq!(sub1, sub2);
        assert_eq!(root1.0, root2.0);

        let nothing1 = RootComponent::update(&mut root1, RootMessage::Nothing);
        let nothing2 = Component::update(&mut root2, RootMessage::Nothing);
        assert_eq!(nothing1, nothing2);
        assert_eq!(root1.0, root2.0);
    }

    #[wasm_bindgen_test]
    fn root_component_view() {
        let root1: RootComp = RootComp::new_root();
        let root2: RootComp = Component::new(());
        let mut behavior1 = AnyComponentBehavior::new();
        let mut behavior2 = AnyComponentBehavior::new();

        let vdom1 = RootComponent::view(&root1, &mut behavior1);
        let vdom2 = Component::view(&root2, &mut behavior2);

        assert!(vdom1.eq(&vdom2));
    }
}
