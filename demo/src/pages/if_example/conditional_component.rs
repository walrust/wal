use wal::component::Component;
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_macros::rsx;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/condition_component.css");
}

pub(crate) struct ConditionComponent {
    current_value: u32,
}

impl Component for ConditionComponent {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        ConditionComponent { current_value: 0 }
    }

    fn view(
        &self,
        behavior: &mut impl wal::component::behavior::Behavior<Self>,
    ) -> wal::virtual_dom::VNode {
        let click = behavior.create_callback(|_event: wal::events::MouseEvent| ());

        CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    <button id={&css["special-btn"]} onclick={click}> "DO you want to see something else?" </button>

                    if self.current_value == 0 {
                        <div class={&css["case-div"]}>
                            "First if"
                        </div>
                    } else if self.current_value == 1 {
                        <div class={&css["case-div"]}>
                            "Second if"
                        </div>
                    } else {
                        <div class={&css["case-div"]}>
                            "Else"
                        </div>
                    }
                </div>
            }
        })
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        self.current_value += 1;
        true
    }
}

impl Default for ConditionComponent {
    fn default() -> Self {
        Self::new(())
    }
}
