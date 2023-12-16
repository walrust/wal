use crate::{
    component::{behavior::Behavior, Component},
    virtual_dom::{VNode, VText},
};

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
        VText::new("Page was not found").into()
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
