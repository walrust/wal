use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_macros::rsx;

pub(crate) struct ChildForComponent {
    id: u32,
}

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/for_child.css");
}

impl Component for ChildForComponent {
    type Message = ();
    type Properties = u32;

    fn new(props: Self::Properties) -> Self {
        ChildForComponent { id: props }
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    {format!("Child {}", self.id)}
                </div>
            }
        })
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
