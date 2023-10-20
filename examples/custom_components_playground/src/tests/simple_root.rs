use wal::{
    component::{component_node::ComponentBehavior, Component},
    virtual_dom::{VElement, VNode},
};

#[allow(dead_code)]
enum RootMessages {
    Add,
    Substract,
}

#[derive(Hash)]
struct RootProperties {
    starting_count: i32,
}

struct RootComponent(i32);
impl Component for RootComponent {
    type Message = RootMessages;
    type Properties = RootProperties;

    fn new(props: Self::Properties) -> Self {
        Self(props.starting_count)
    }

    fn view(&self, _behavior: &mut ComponentBehavior<Self>) -> VNode {
        VElement {
            tag_name: "div".to_string(),
            attr: [("counter".to_string(), self.0.to_string())].into(),
            children: vec![],
            dom: None,
        }
        .into()
    }

    fn update(&mut self, message: Self::Message) -> bool {
        self.0 += match message {
            RootMessages::Add => 1,
            RootMessages::Substract => -1,
        };
        true
    }
}

#[allow(dead_code)]
pub fn start() {
    let comp = RootComponent(0);
    wal::app::start(comp);
}
