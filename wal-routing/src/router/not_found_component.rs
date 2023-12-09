use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_rsx::rsx;

pub(crate) const NOT_FOUND_PATH: &str = "/404";

pub(crate) struct NotFoundComponent;
impl Default for NotFoundComponent {
    fn default() -> Self {
        Self::new(())
    }
}
impl Component for NotFoundComponent {
    type Message = ();
    type Properties = ();
    fn new(_props: Self::Properties) -> Self {
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
