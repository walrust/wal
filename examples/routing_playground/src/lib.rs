use wasm_bindgen::prelude::wasm_bindgen;
use wal_routing::app;
use wal::{self, component::{Component, behavior::Behavior, callback::Callback}, utils::debug};
use wal_macros::html;
use web_sys::MouseEvent;

struct RootComp;

#[derive(Hash)]
struct RootProp;

impl Component for RootComp {
    type Message=();

    type Properties=RootProp;

    fn new(_props: Self::Properties) -> Self {
        RootComp
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Jasna dupa");
        });
        html! {
            <div id="rootcomp">
                <button onclick={call}>"Ale jazda"</button>
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

#[wasm_bindgen(start)]
fn start() {
    wal::app::start(RootComp);
    // let _app = app::builder::AppBuilder::new()
    //     .add_page::<RootComp>("/", RootProp{})
    //     .build();
}