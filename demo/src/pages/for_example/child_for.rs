use wal::{component::{Component, behavior::Behavior}, virtual_dom::VNode};
use wal_macros::rsx;

pub(crate) struct ChildForComponent {
    id: u32,
}

impl Component for ChildForComponent {
    type Message = ();
    type Properties = u32;

    fn new(props: Self::Properties) -> Self {
        ChildForComponent { id: props }
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        rsx! {
            <div>
                {format!("Child {}", self.id)}
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}