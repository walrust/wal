use gloo::timers::callback::Timeout;
use wal::{
    component::{behavior::Behavior, callback::Callback, Component},
    virtual_dom::VNode,
};
use wal_macros::rsx;
use wal_routing::prelude::RouterBuilder;

enum FatherMessages {
    Add,
}

#[derive(Hash)]
struct FatherProperties(i32);

struct FatherComponent(i32);
impl Default for FatherComponent {
    fn default() -> Self {
        Self::new(())
    }
}
impl Component for FatherComponent {
    type Message = FatherMessages;
    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        Self(1)
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        let callback = _behavior.create_callback(|()| FatherMessages::Add);

        rsx! {
            <div>
                if self.0 % 2 == 0 {
                    <ChildComponent props = {ChildProperties(self.0, callback.clone())} />
                } else {
                    <ChildComponent props = {ChildProperties(self.0, callback.clone())} />
                    <ChildComponent props = {ChildProperties(-self.0, callback.clone())} />
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
        rsx! {
            <div counter_child="0"></div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

#[allow(dead_code)]
pub fn start() {
    RouterBuilder::default()
        .add_page::<FatherComponent>("/")
        .build()
        .start();
}
