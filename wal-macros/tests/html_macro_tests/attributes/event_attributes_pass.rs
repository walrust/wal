use std::collections::HashMap;
use wal::{
    component::callback::Callback,
    events::{onclick, oncopy, EventHandler},
    virtual_dom::{VElement, VNode},
};
use wal_macros::html;

include!("../utils/new_velement_str.rs");

fn main() {
    single_specialized_event_attribute();
    single_unspecialized_event_attribute();
    multiple_event_attributes();
}

fn single_specialized_event_attribute() {
    let html = html! { <div onclick={Callback::new(|_event: web_sys::MouseEvent| {})}></div> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![EventHandler::new(onclick(Callback::new(
                |_event: web_sys::MouseEvent| {}
            )))],
            None,
            Vec::new(),
        ))
    );
}

fn single_unspecialized_event_attribute() {
    let html = html! { <div oncopy={Callback::new(|_event: web_sys::Event| {})}></div> };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![EventHandler::new(oncopy(Callback::new(
                |_event: web_sys::Event| {}
            )))],
            None,
            Vec::new(),
        ))
    );
}

fn multiple_event_attributes() {
    let html = html! {
        <div
            onclick={Callback::new(|_event: web_sys::MouseEvent| {})}
            oncopy={Callback::new(|_event: web_sys::Event| {})}>
        </div>
    };
    assert_eq!(
        html,
        VNode::Element(new_velement_str(
            "div",
            HashMap::new(),
            vec![
                EventHandler::new(onclick(Callback::new(|_event: web_sys::MouseEvent| {}))),
                EventHandler::new(oncopy(Callback::new(|_event: web_sys::Event| {}))),
            ],
            None,
            Vec::new(),
        ))
    );
}
