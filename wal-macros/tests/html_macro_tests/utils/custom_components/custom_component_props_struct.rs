struct CustomComponentPropsStruct;

#[derive(Hash)]
struct PropsStruct {
    x: i32,
}

impl PropsStruct {
    fn new(x: i32) -> Self {
        Self { x }
    }
}

impl Component for CustomComponentPropsStruct {
    type Properties = PropsStruct;
    type Message = ();

    fn new(_props: Self::Properties) -> Self {
        unimplemented!();
    }

    fn view(&self, _behavior: &mut ComponentBehavior<Self>) -> VNode {
        unimplemented!();
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        unimplemented!()
    }
}
