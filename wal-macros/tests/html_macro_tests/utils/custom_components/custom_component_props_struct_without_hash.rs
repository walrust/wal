struct CustomComponentPropsStructWithoutHash;

#[derive(Default)]
struct PropsWithoutHash;

impl Component for CustomComponentPropsStructWithoutHash {
    type Properties = PropsWithoutHash;
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
