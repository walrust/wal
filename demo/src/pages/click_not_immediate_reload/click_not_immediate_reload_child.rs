use wal::{
    component::{callback::Callback, Component},
    events::MouseEvent,
    utils::debug,
};
use wal_macros::rsx;
use wasm_bindgen::JsCast;
use web_sys::{
    console::{log, log_0, log_1},
    HtmlInputElement,
};

pub(crate) struct ClickNotImmediateReloadChild {
    props: ClickNotImmediateReloadChildProperties,
}

pub(crate) struct ToBeUpdatedMessage {
    pub(crate) id: i32,
    pub(crate) count: i32,
    pub(crate) name: String,
}

#[derive(Hash, Clone)]
pub(crate) struct ClickNotImmediateReloadChildProperties {
    pub(crate) id: i32,
    pub(crate) count: i32,
    pub(crate) name: String,
    pub(crate) click: Callback<ToBeUpdatedMessage>,
}

impl Component for ClickNotImmediateReloadChild {
    type Message = ();

    type Properties = ClickNotImmediateReloadChildProperties;

    fn new(props: Self::Properties) -> Self {
        ClickNotImmediateReloadChild { props }
    }

    fn view(
        &self,
        behavior: &mut impl wal::component::behavior::Behavior<Self>,
    ) -> wal::virtual_dom::VNode {
        let update_counter_on_click = behavior.create_callback(|_event: MouseEvent| ());

        let props = self.props.clone();

        let save_changes_on_click = Callback::new(move |_event: MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();
            let element = document
                .get_element_by_id(props.id.to_string().as_str())
                .unwrap();
            let input_element = element.dyn_into::<HtmlInputElement>().unwrap();
            let new_name = input_element.value();

            let message = ToBeUpdatedMessage {
                id: props.id,
                count: props.count,
                name: new_name,
            };
            props.click.emit(message);
        });

        rsx! {
            <div>
                <button onclick={update_counter_on_click}>
                    "update counter"
                </button>
                <input id={self.props.id} value = {props.name} />
                <button onclick={save_changes_on_click}>
                    "save changes"
                </button>
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        self.props.count += 1;
        true
    }
}
