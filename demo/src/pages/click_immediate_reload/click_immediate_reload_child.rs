use wal::{
    component::{behavior::Behavior, callback::Callback, Component},
    events::{InputEvent, MouseEvent},
    virtual_dom::VNode,
};
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_rsx::rsx;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/click_child.css");
}

#[derive(Hash)]
pub(crate) struct ChildImmediateReloadProperties {
    pub(crate) click: Callback<()>,
    pub(crate) on_change_name: Callback<String>,
    pub(crate) name: String,
}

pub(crate) struct ChildImmediateReloadComponent(ChildImmediateReloadProperties);

impl Component for ChildImmediateReloadComponent {
    type Message = ();
    type Properties = ChildImmediateReloadProperties;

    fn new(props: Self::Properties) -> Self {
        Self(props)
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        let click = self.0.click.clone();
        let on_click = Callback::new(move |_event: MouseEvent| {
            click.emit(());
        });

        let change_name = self.0.on_change_name.clone();
        let on_change_name = Callback::new(move |event: InputEvent| {
            let target = event.target().unwrap();
            let input_element = target.dyn_ref::<HtmlInputElement>().unwrap();
            change_name.emit(input_element.value());
        });

        CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    <button  class={&css["fill-btn"]} onclick={on_click}>
                        "click me"
                    </button>
                    <input class={&css["input-txt"]} value = {self.0.name.clone()} oninput = {on_change_name} />
                </div>
            }
        })
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}
