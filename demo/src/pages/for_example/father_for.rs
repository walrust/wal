use crate::pages::for_example::child_for::ChildForComponent;
use wal::{
    component::{behavior::Behavior, Component},
    events::MouseEvent,
    virtual_dom::VNode,
};
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_macros::rsx;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/for_father.css");
}

pub(crate) struct FatherForComponent {
    child_number: u32,
}

impl Component for FatherForComponent {
    type Message = ();
    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        FatherForComponent { child_number: 0 }
    }

    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode {
        let click = behavior.create_callback(|_event: MouseEvent| ());
        let child_number = self.child_number;

        CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    <button id={&css["btn"]} onclick={click}>{"Add child"}</button>
                    <div class={&css["children-container"]}>
                        for { Iterator::map(0..child_number, |x| rsx! { <ChildForComponent props = {x} /> }) }
                    </div>
                </div>
            }
        })
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        self.child_number += 1;
        true
    }
}

impl Default for FatherForComponent {
    fn default() -> Self {
        Self::new(())
    }
}
