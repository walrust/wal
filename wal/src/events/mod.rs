use std::{borrow::Cow, fmt::Debug};

use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};

use crate::component::callback::Callback;

#[macro_use]
mod macros;

pub trait EventHandler {
    fn get_event_type(&self) -> Cow<'static, str>;
    fn get_callback(&self) -> Box<dyn FnMut(&Event)>;
}

impl Debug for dyn EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("EventHandler of type {}", self.get_event_type()).fmt(f)
    }
}

trait EventHandlerType {
    type Handler: EventHandler;
}

pub struct UnspecializedEventHandler {
    pub event_type: Cow<'static, str>,
    pub callback: Callback<Event>,
}

impl EventHandler for UnspecializedEventHandler {
    fn get_event_type(&self) -> Cow<'static, str> {
        self.event_type.clone()
    }

    fn get_callback(&self) -> Box<dyn FnMut(&Event)> {
        let callback = self.callback.clone();
        Box::new(move |event: &Event| {
            let event = event.clone();
            callback.emit(event);
        })
    }
}

event_handlers! {
    MouseEventHandler(MouseEvent)
}

unspecialized_event_handlers_constructor! {
    onabort
}

event_handlers_constructor! {
    onclick(MouseEvent),
    ondblclick(MouseEvent)
}
