use std::{borrow::Cow, fmt::Debug, hash::Hash};

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{
    AnimationEvent, DragEvent, Element, Event, FocusEvent, InputEvent, KeyboardEvent, MouseEvent,
    PointerEvent, ProgressEvent, SubmitEvent, TouchEvent, TransitionEvent, WheelEvent,
};

use crate::{component::callback::Callback, virtual_dom::Dom};

#[macro_use]
mod macros;

pub trait EventCreator {
    fn get_event_type(&self) -> Cow<'static, str>;
    fn create_callback(&self) -> Box<dyn FnMut(&Event)>;
}

impl Debug for dyn EventCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("EventCreator of type {}", self.get_event_type()).fmt(f)
    }
}

impl PartialEq for dyn EventCreator {
    fn eq(&self, other: &Self) -> bool {
        self.get_event_type() == other.get_event_type()
    }
}

trait EventCreatorType {
    type Creator: EventCreator;
}

pub struct UnspecializedEventCreator {
    pub event_type: Cow<'static, str>,
    pub callback: Callback<Event>,
}

impl EventCreator for UnspecializedEventCreator {
    fn get_event_type(&self) -> Cow<'static, str> {
        self.event_type.clone()
    }

    fn create_callback(&self) -> Box<dyn FnMut(&Event)> {
        let callback = self.callback.clone();
        Box::new(move |event: &Event| {
            let event = event.clone();
            callback.emit(event);
        })
    }
}

// TODO maybe insted of using web_sys events we can wrap them so that the user dont have to import web_sys
event_creators! {
    AnimationEventCreator(AnimationEvent),
    DragEventCreator(DragEvent),
    FocusEventCreator(FocusEvent),
    InputEventCreator(InputEvent),
    KeyboardEventCreator(KeyboardEvent),
    MouseEventCreator(MouseEvent),
    PointerEventCreator(PointerEvent),
    ProgressEventCreator(ProgressEvent),
    SubmitEventCreator(SubmitEvent),
    TouchEventCreator(TouchEvent),
    TransitionEventCreator(TransitionEvent),
    WheelEventCreator(WheelEvent)
}

unspecialized_event_creators_constructor! {
    onabort,
    oncancel,
    oncanplay,
    oncanplaythrough,
    onchange,
    onclose,
    oncopy,
    oncuechange,
    oncut,
    ondurationchange,
    onemptied,
    onended,
    onerror,
    oninvalid,
    onload,
    onloadeddata,
    onloadedmetadata,
    onpaste,
    onpause,
    onplay,
    onplaying,
    onpointerlockchange,
    onpointerlockerror,
    onratechange,
    onreset,
    onresize,
    onscroll,
    onsecuritypolicyviolation,
    onseeked,
    onseeking,
    onselect,
    onselectionchange,
    onselectstart,
    onshow,
    onslotchange,
    onstalled,
    onsuspend,
    ontimeupdate,
    ontoggle,
    onvolumechange,
    onwaiting,

    // FormData Events
    onformdata  // web_sys is missing `FormDataEvent`` so it is handled as Unspecialized Event
}

event_creators_constructor! {
    // Animation Events
    onanimationcancel(AnimationEvent),
    onanimationend(AnimationEvent),
    onanimationiteration(AnimationEvent),
    onanimationstart(AnimationEvent),

    // Drag Events
    ondrag(DragEvent),
    ondragend(DragEvent),
    ondragenter(DragEvent),
    ondragexit(DragEvent),
    ondragleave(DragEvent),
    ondragover(DragEvent),
    ondragstart(DragEvent),
    ondrop(DragEvent),

    // Focus Events
    onblur(FocusEvent),
    onfocus(FocusEvent),
    onfocusin(FocusEvent),
    onfocusout(FocusEvent),

    // Input Events
    oninput(InputEvent),

    // Keyboard Events
    onkeydown(KeyboardEvent),
    onkeypress(KeyboardEvent),
    onkeyup(KeyboardEvent),

    // Mouse Events
    onauxclick(MouseEvent),
    onclick(MouseEvent),
    oncontextmenu(MouseEvent),
    ondblclick(MouseEvent),
    onmousedown(MouseEvent),
    onmouseenter(MouseEvent),
    onmouseleave(MouseEvent),
    onmousemove(MouseEvent),
    onmouseout(MouseEvent),
    onmouseover(MouseEvent),
    onmouseup(MouseEvent),

    // Pointer Events
    ongotpointercapture(PointerEvent),
    onlostpointercapture(PointerEvent),
    onpointercancel(PointerEvent),
    onpointerdown(PointerEvent),
    onpointerenter(PointerEvent),
    onpointerleave(PointerEvent),
    onpointermove(PointerEvent),
    onpointerout(PointerEvent),
    onpointerover(PointerEvent),
    onpointerup(PointerEvent),

    // Progress Events
    onloadend(ProgressEvent),
    onloadstart(ProgressEvent),
    onprogress(ProgressEvent),

    // Submit Events
    onsubmit(SubmitEvent),

    // Touch Events
    ontouchcancel(TouchEvent),
    ontouchend(TouchEvent),
    ontouchmove(TouchEvent),
    ontouchstart(TouchEvent),

    // Transition Events
    ontransitioncancel(TransitionEvent),
    ontransitionend(TransitionEvent),
    ontransitionrun(TransitionEvent),
    ontransitionstart(TransitionEvent),

    // Wheel Events
    onwheel(WheelEvent)
}

#[derive(Debug)]
pub struct EventHandler {
    event_creator: Box<dyn EventCreator>,
    event_listener: Option<EventListener>,
}

impl EventHandler {
    pub fn new(event_creator: Box<dyn EventCreator>) -> Self {
        Self {
            event_creator,
            event_listener: None,
        }
    }

    pub(crate) fn attach(&mut self, element: &Element) {
        let event_type = self.event_creator.get_event_type();
        let callback = self.event_creator.create_callback();
        self.event_listener = Some(Dom::create_event_listener(element, event_type, callback));
    }

    pub(crate) fn get_event_type_from_creator(&self) -> Cow<'static, str> {
        self.event_creator.get_event_type()
    }
}

impl PartialEq for EventHandler {
    fn eq(&self, other: &Self) -> bool {
        let event_listener_eq = match (&self.event_listener, &other.event_listener) {
            (Some(self_event_listener), Some(other_event_listener)) => {
                self_event_listener.event_type() == other_event_listener.event_type()
            }
            (None, None) => true,
            _ => false,
        };
        event_listener_eq && *self.event_creator == *other.event_creator
    }
}

impl Eq for EventHandler {}

impl Hash for EventHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_event_type_from_creator().hash(state);
    }
}