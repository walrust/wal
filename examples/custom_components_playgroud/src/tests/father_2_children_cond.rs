use gloo::timers::callback::Interval;
use wal::{
    component::{callback::Callback, component_node::ComponentBehavior, Component},
    virtual_dom::{VComponent, VElement, VNode},
};

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

    fn view(&self, _behavior: &mut ComponentBehavior<Self>) -> VNode {
        let callback = _behavior.create_callback(|()| FatherMessages::Add);

        VElement {
            tag_name: "div".to_string(),
            attr: [("father".to_string(), "true".to_string())].into(),
            children: {
                if self.0 % 2 == 0 {
                    vec![
                        VComponent::new::<ChildComponent>(ChildProperties(self.0, callback)).into(),
                    ]
                } else {
                    vec![
                        VComponent::new::<ChildComponent>(ChildProperties(
                            self.0,
                            callback.clone(),
                        ))
                        .into(),
                        VComponent::new::<ChildComponent>(ChildProperties(self.0 * -1, callback))
                            .into(),
                    ]
                }
            },
            dom: None,
        }
        .into()
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
        Interval::new(5000, move || {
            cb.emit(());
        })
        .forget();

        Self(props.0, props.1)
    }

    fn view(&self, _behavior: &mut ComponentBehavior<Self>) -> VNode {
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

pub fn start() {
    let comp = FatherComponent(0);
    wal::app::start(comp);
}
