use std::collections::HashMap;

use gloo::console::log;
use wal::{
    component::{behavior::Behavior, callback::Callback, Component},
    events::{EventHandler, MouseEventCreator},
    virtual_dom::{VComponent, VElement, VList, VNode, VText},
};
use web_sys::MouseEvent;

enum FatherMessages {
    Clicked,
}

#[derive(Hash)]
struct FatherProperties;

struct FatherComponent(i32);

impl Component for FatherComponent {
    type Message = FatherMessages;
    type Properties = FatherProperties;

    fn new(_props: Self::Properties) -> Self {
        Self(0)
    }

    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode {
        let callback = behavior.create_callback(|()| FatherMessages::Clicked);

        VNode::List(VList::new(vec![
            VNode::Text(VText {
                text: format!("My child got clicked {} times", self.0),
                dom: None,
            }),
            VNode::Component(VComponent::new::<ChildComponent>(ChildProperties(
                callback, self.0,
            ))),
        ]))
    }

    fn update(&mut self, message: Self::Message) -> bool {
        self.0 += match message {
            FatherMessages::Clicked => 1,
        };
        true
    }
}

#[derive(Hash)]
struct ChildProperties(Callback<()>, i32);

struct ChildComponent(Callback<()>, i32);

impl Component for ChildComponent {
    type Message = ();
    type Properties = ChildProperties;

    fn new(props: Self::Properties) -> Self {
        Self(props.0, props.1)
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        let cb = self.0.clone();
        let i = self.1;
        let on_click = Callback::new(move |_event: MouseEvent| {
            log!(format!(
                "Child got clicked!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {}",
                i
            ));
            cb.emit(());
        });

        VElement {
            tag_name: "button".to_string(),
            attr: HashMap::new(),
            children: vec![VNode::Text(VText {
                text: "click me".to_string(),
                dom: None,
            })],
            event_handlers: vec![EventHandler::new(Box::new(MouseEventCreator::new(
                "click".to_string().into(),
                on_click,
            )))],
            dom: None,
        }
        .into()
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

pub fn start() {
    let comp = FatherComponent(0);
    wal::app::start(comp);
}
