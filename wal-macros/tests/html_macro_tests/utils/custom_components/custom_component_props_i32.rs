struct CustomComponentPropsI32;

impl Component for CustomComponentPropsI32 {
    type Properties = i32;
    type Message = ();

    fn new(_props: Self::Properties) -> Self {
        unimplemented!();
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        unimplemented!();
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        unimplemented!()
    }
}
