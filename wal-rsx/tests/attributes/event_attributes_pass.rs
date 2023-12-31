use std::collections::HashMap;
use wal_core::{
    component::callback::Callback,
    events::{onclick, oncopy, EventHandler},
    virtual_dom::{VElement, VNode},
};
use wal_rsx::rsx;

include!("../utils/new_velement_str.rs");

fn main() {
    single_specialized_event_attribute();
    single_unspecialized_event_attribute();
    multiple_event_attributes();
}

fn single_specialized_event_attribute() {
    let rsx =
        rsx! { <div onclick={Callback::new(|_event: wal_core::events::MouseEvent| {})}></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![EventHandler::new(onclick(Callback::new(
                |_event: wal_core::events::MouseEvent| {}
            )))],
            None,
            Vec::new(),
        ))
    );
}

fn single_unspecialized_event_attribute() {
    let rsx = rsx! { <div oncopy={Callback::new(|_event: wal_core::events::Event| {})}></div> };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![EventHandler::new(oncopy(Callback::new(
                |_event: wal_core::events::Event| {}
            )))],
            None,
            Vec::new(),
        ))
    );
}

fn multiple_event_attributes() {
    let rsx = rsx! {
        <div
            onclick={Callback::new(|_event: wal_core::events::MouseEvent| {})}
            oncopy={Callback::new(|_event: wal_core::events::Event| {})}>
        </div>
    };
    assert_eq!(
        rsx,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![
                EventHandler::new(onclick(Callback::new(
                    |_event: wal_core::events::MouseEvent| {}
                ))),
                EventHandler::new(oncopy(Callback::new(|_event: wal_core::events::Event| {}))),
            ],
            None,
            Vec::new(),
        ))
    );
}
