use wal_core::component::Component;
use wal_rsx::rsx;

use crate::nested_component::{NestedComponent1, NestedComponent2};

#[derive(Default)]
pub(crate) struct RootComponent {
    pub(crate) value: u8,
}

impl Component for RootComponent {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        Self { value: 0 }
    }

    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let on_click = behavior.create_callback(|_| ());

        rsx! {
            <div>
                <button id="button" onclick={ on_click }> "Click me" </button>
                if self.value % 2 == 0 {
                    <NestedComponent1 props = 2 />
                } else {
                    <NestedComponent2 props = 2 />
                }
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        self.value += 1;
        true
    }
}
