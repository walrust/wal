use wal_core::component::Component;
use wal_rsx::rsx;

pub(crate) struct NestedComponent1 {
    pub(crate) counter: u8,
}

impl Component for NestedComponent1 {
    type Message = ();

    type Properties = u8;

    fn new(props: Self::Properties) -> Self {
        Self { counter: props }
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx! {
            "First version of nested component"
            if self.counter != 0 {
                <div>
                    <NestedComponent1 props = {self.counter - 1} />
                </div>
            }
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

pub(crate) struct NestedComponent2 {
    pub(crate) counter: u8,
}

impl Component for NestedComponent2 {
    type Message = ();

    type Properties = u8;

    fn new(props: Self::Properties) -> Self {
        Self { counter: props }
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx! {
            "Second version of nested component"
            if self.counter != 0 {
                <span>
                    <NestedComponent2 props = {self.counter - 1} />
                </span>
            }
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
