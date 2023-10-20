use wal::{
    component::{Component, behavior::Behavior},
    virtual_dom::{VComponent, VElement, VNode},
};

#[allow(dead_code)]
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
        VElement {
            tag_name: "div".to_string(),
            attr: [("father".to_string(), "true".to_string())].into(),
            children: vec![VComponent::new::<ChildComponent>(ChildProperties(self.0)).into()],
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
struct ChildProperties(i32);

struct ChildComponent(i32);
impl Component for ChildComponent {
    type Message = ChildMessages;
    type Properties = ChildProperties;

    fn new(props: Self::Properties) -> Self {
        Self(props.0)
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

#[allow(dead_code)]
pub fn start() {
    let comp = FatherComponent(0);
    wal::app::start(comp);
}
