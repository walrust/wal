use std::{borrow::Cow, fmt::Debug};

use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};

use crate::component::callback::Callback;

// change fields to private
pub struct MouseEventHandler {
    pub event_type: Cow<'static, str>,
    pub callback: Callback<MouseEvent>,
}

impl EventHandler for MouseEventHandler {
    fn get_event_type(&self) -> Cow<'static, str> {
        self.event_type.clone()
    }

    fn get_callback(&self) -> Box<dyn FnMut(&Event)> {
        let callback = self.callback.clone();
        Box::new(move |event: &Event| {
            let event = event.clone().unchecked_into();
            callback.emit(event)
        })
    }
}

pub trait EventHandler {
    fn get_event_type(&self) -> Cow<'static, str>;
    fn get_callback(&self) -> Box<dyn FnMut(&Event)>;
}

impl Debug for dyn EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("EventHandler of type {}", self.get_event_type()).fmt(f)
    }
}

// use web_sys::MouseEvent;

// use crate::component::callback::Callback;

// pub mod onclick {
//     use web_sys::MouseEvent;

//     use crate::component::callback::Callback;

//     use super::Listener;

//     pub type Event = MouseEvent;

//     pub struct Wrapper {
//         callback: Callback<Event>,
//     }

//     impl Wrapper {
//         pub fn new(callback: Callback<Event>) -> Self {
//             Wrapper { callback }
//         }
//     }

//     impl Listener<Event> for Wrapper {
//         fn kind(&self) -> super::ListenerKind {
//             super::ListenerKind::onclick
//         }

//         fn handle(&self, event: Event) {
//             self.callback.emit(event)
//         }

//         fn passive(&self) -> bool {
//             false
//         }
//     }
// }

// pub trait Listener<Event> {
//     /// Returns the name of the event
//     fn kind(&self) -> ListenerKind;

//     /// Handles an event firing
//     fn handle(&self, event: Event);

//     /// Makes the event listener passive. See
//     /// [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener).
//     fn passive(&self) -> bool;
// }
