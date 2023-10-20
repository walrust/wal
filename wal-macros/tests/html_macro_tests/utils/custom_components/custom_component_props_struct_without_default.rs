struct CustomComponentPropsStructWithoutDefault;

#[derive(Hash)]
struct PropsWithoutDefault;

impl Component for CustomComponentPropsStructWithoutDefault {
    type Properties = PropsWithoutDefault;
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
