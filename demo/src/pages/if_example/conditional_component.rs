use wal::component::Component;
use wal_macros::rsx;

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
        rsx! {
            <button onclick={click}> "DO you want to see something else?" </button>
            if self.current_value == 0 {
                <div>
                    "First if"
                </div>
            } else if self.current_value == 1 {
                <div>
                    "Second if"
                </div>
            } else {
                <div>
                    "Else"
                </div>
            }
        }
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
