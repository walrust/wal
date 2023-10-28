use std::{borrow::Cow, fmt::Debug};

use wasm_bindgen::JsCast;
use web_sys::{
    AnimationEvent, DragEvent, Event, FocusEvent, InputEvent, KeyboardEvent, MouseEvent,
    PointerEvent, ProgressEvent, SubmitEvent, TouchEvent, TransitionEvent, WheelEvent,
};

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
    AnimationEventHandler(AnimationEvent),
    DragEventHandler(DragEvent),
    FocusEventHandler(FocusEvent),
    InputEventHandler(InputEvent),
    KeyboardEventHandler(KeyboardEvent),
    MouseEventHandler(MouseEvent),
    PointerEventHandler(PointerEvent),
    ProgressEventHandler(ProgressEvent),
    SubmitEventHandler(SubmitEvent),
    TouchEventHandler(TouchEvent),
    TransitionEventHandler(TransitionEvent),
    WheelEventHandler(WheelEvent)
}

unspecialized_event_handlers_constructor! {
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

event_handlers_constructor! {
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
