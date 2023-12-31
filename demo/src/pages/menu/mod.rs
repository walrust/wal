use wal_core::component::Component;
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_rsx::rsx;

pub(crate) struct MenuComponent;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/menu.css");
}

impl Component for MenuComponent {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        MenuComponent
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        CSS.with(|css| {
            rsx!(
                <div class={&css["container"]}>
                    <h1> "Page menu" </h1>
                    <div class={&css["menu-container"]}>
                        <Link to="/immediate_reload">"Immediate reload"</Link>
                        <Link to="/not_immediate_reload">"Not immediate reload"</Link>
                        <Link to="/for_example">"For example"</Link>
                        <Link to="/if_example">"If example"</Link>
                    </div>
                </div>
            )
        })
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
