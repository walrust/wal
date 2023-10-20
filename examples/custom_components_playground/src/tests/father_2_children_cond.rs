use gloo::timers::callback::Timeout;
use wal::{
    component::{callback::Callback, Component, behavior::Behavior},
    virtual_dom::{VElement, VNode}, utils::debug_warn,
};
use wal_macros::html;

enum FatherMessages {
    Add,
}

#[derive(Hash)]
struct FatherProperties(i32);

struct FatherComponent(i32);
impl Component for FatherComponent {
    type Message = FatherMessages;
    type Properties = FatherProperties;

    fn new(props: Self::Properties) -> Self {
        Self(props.0)
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        let callback = _behavior.create_callback(|()| FatherMessages::Add);

        html! {
            <div>
                if self.0 % 2 == 0 {
                    <ChildComponent props = {ChildProperties(self.0, callback)} />
                } else {
                    <ChildComponent props = {ChildProperties(self.0, callback.clone())} />
                    <ChildComponent props = {ChildProperties(self.0 * -1, callback)} />
                }
            </div>
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        self.0 += match message {
            FatherMessages::Add => 1,
        };
        true
    }
}

enum ChildMessages {}

#[derive(Hash)]
struct ChildProperties(i32, Callback<()>);

struct ChildComponent(i32, Callback<()>);
impl Component for ChildComponent {
    type Message = ChildMessages;
    type Properties = ChildProperties;

    fn new(props: Self::Properties) -> Self {
        let cb = props.1.clone();
        Timeout::new(5000, move || {
            cb.emit(());
        })
        .forget();

        Self(props.0, props.1)
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        VElement {
            tag_name: "div".to_string(),
            attr: [("counter-child".to_string(), self.0.to_string())].into(),
            children: vec![],
            dom: None,
        }
        .into()
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

impl Drop for ChildComponent {
    fn drop(&mut self) {
        debug_warn("TO DELETE, ChildComponent is dropped");
    }
}

#[allow(dead_code)]
pub fn start() {
    let comp = FatherComponent(0);
    wal::app::start(comp);
}