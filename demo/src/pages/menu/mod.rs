use wal::component::Component;
use wal_macros::rsx;

pub(crate) struct MenuComponent;

impl Component for MenuComponent {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        MenuComponent
    }

    fn view(
        &self,
        _behavior: &mut impl wal::component::behavior::Behavior<Self>,
    ) -> wal::virtual_dom::VNode {
        rsx!(
            <div>
                <Link to="/immediate_reload">"Immediate reload"</Link>
                <Link to="/not_immediate_reload">"Not immediate reload"</Link>
                <Link to="/for_example">"For example"</Link>
                <Link to="/if_example">"If example"</Link>
            </div>
        )
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

impl Default for MenuComponent {
    fn default() -> Self {
        Self::new(())
    }
}
