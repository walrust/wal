use wal::{
    component::{behavior::Behavior, root::RootComponent},
    virtual_dom::VNode,
};
use wal_macros::rsx;

pub(crate) const NOT_FOUND_PATH: &str = "/404";

pub(crate) struct NotFoundComponent;
impl RootComponent for NotFoundComponent {
    type Message = ();
    fn new_root() -> Self {
        NotFoundComponent
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        rsx! {
            "Page was not found :(("
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
