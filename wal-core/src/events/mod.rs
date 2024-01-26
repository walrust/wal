use std::{borrow::Cow, fmt::Debug, hash::Hash, ops::Deref};

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::{component::callback::Callback, virtual_dom::dom};

#[macro_use]
mod macros;

define_events!(
    Event,
    AnimationEvent,
    DragEvent,
    FocusEvent,
    InputEvent,
    KeyboardEvent,
    MouseEvent,
    PointerEvent,
    ProgressEvent,
    SubmitEvent,
    TouchEvent,
    TransitionEvent,
    WheelEvent
);

#[doc(hidden)]
pub trait EventCreator {
    fn get_event_type(&self) -> Cow<'static, str>;
    fn create_callback(&self) -> Box<dyn FnMut(&web_sys::Event)>;
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

#[doc(hidden)]
pub struct UnspecializedEventCreator {
    pub event_type: Cow<'static, str>,
    pub callback: Callback<Event>,
}

impl EventCreator for UnspecializedEventCreator {
    fn get_event_type(&self) -> Cow<'static, str> {
        self.event_type.clone()
    }

    fn create_callback(&self) -> Box<dyn FnMut(&web_sys::Event)> {
        let callback = self.callback.clone();
        Box::new(move |event: &web_sys::Event| {
            let event = Event(event.clone());
            callback.emit(event);
        })
    }
}

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

#[doc(hidden)]
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
        let event_type = self.get_event_type();
        let callback = self.event_creator.create_callback();
        self.event_listener = Some(dom::create_event_listener(element, event_type, callback));
    }

    pub(crate) fn get_event_type(&self) -> Cow<'static, str> {
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
        self.get_event_type().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn onclick_sepcialized_event_handler_constructor_should_create_event_handler_of_correct_type() {
        // Arrange
        let callback = Callback::new(|_| {});

        // Act
        let event_creator = onclick(callback);

        // Assert
        let event_type = event_creator.get_event_type();
        assert_eq!(event_type, "click");
    }

    #[wasm_bindgen_test]
    fn onclick_specialized_event_handler_constructor_should_create_event_handler_with_correct_callback(
    ) {
        // Arrange
        let was_callback_executed = Rc::new(RefCell::new(false));
        let was_callback_executed_clone = was_callback_executed.clone();
        let click = "click";
        let callback = Callback::new(move |mouse_event: MouseEvent| {
            assert_eq!(mouse_event.type_(), click);
            *was_callback_executed_clone.borrow_mut() = true;
        });
        let web_sys_mouse_click_event = web_sys::MouseEvent::new(click).unwrap();

        // Act
        let event_creator = onclick(callback);

        // Assert
        let mut event_callback = event_creator.create_callback();
        (*event_callback)(web_sys_mouse_click_event.as_ref());
        assert!(*was_callback_executed.borrow());
    }

    #[wasm_bindgen_test]
    fn onabort_unspecialized_event_handler_constructor_should_create_event_handler_of_correct_type()
    {
        // Arrange
        let callback = Callback::new(|_| {});

        // Act
        let event_creator = onabort(callback);

        // Assert
        let event_type = event_creator.get_event_type();
        assert_eq!(event_type, "abort");
    }

    #[wasm_bindgen_test]
    fn onabort_unspecialized_event_handler_constructor_should_create_event_handler_with_correct_callback(
    ) {
        // Arrange
        let was_callback_executed = Rc::new(RefCell::new(false));
        let was_callback_executed_clone = was_callback_executed.clone();
        let abort = "abort";
        let callback = Callback::new(move |event: Event| {
            assert_eq!(event.type_(), abort);
            *was_callback_executed_clone.borrow_mut() = true;
        });

        // Act
        let event_creator = onabort(callback);

        // Assert
        let mut event_callback = event_creator.create_callback();
        (*event_callback)(web_sys::Event::new(abort).unwrap().as_ref());
        assert!(*was_callback_executed.borrow());
    }

    #[wasm_bindgen_test]
    fn define_mouse_event_should_define_correct_event() {
        // Arrange
        let click = "click";
        let web_sys_mouse_click_event = web_sys::MouseEvent::new(click).unwrap();

        // Act
        let event = MouseEvent::new(web_sys_mouse_click_event);

        // Assert
        assert_eq!(event.type_(), click);
    }

    struct TestEventCreator {
        event_type: Cow<'static, str>,
        flag: Rc<RefCell<bool>>,
    }

    impl EventCreator for TestEventCreator {
        fn get_event_type(&self) -> Cow<'static, str> {
            self.event_type.clone()
        }

        fn create_callback(&self) -> Box<dyn FnMut(&web_sys::Event)> {
            let flag = self.flag.clone();
            Box::new(move |_| {
                *flag.borrow_mut() = true;
            })
        }
    }

    #[wasm_bindgen_test]
    fn event_handler_attach_should_attach_event_to_an_element() {
        // Arrange
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let flag = Rc::new(RefCell::new(false));
        let event_creator = Box::new(TestEventCreator {
            event_type: Cow::from("click"),
            flag: flag.clone(),
        });
        let mut handler = EventHandler::new(event_creator);
        handler.attach(&body);
        let event = web_sys::Event::new("click").unwrap();

        // Act
        body.dispatch_event(&event).unwrap();

        // Assert
        assert!(*flag.borrow());
    }

    #[wasm_bindgen_test]
    fn get_event_type_should_get_correct_event_type() {
        // Arrange
        let callback = Callback::new(|_| {});
        let event_creator = onclick(callback);
        let handler = EventHandler::new(event_creator);

        // Act
        let event_type = handler.get_event_type();

        // Assert
        assert_eq!(event_type, "click");
    }
}
